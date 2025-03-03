use self::params::SearchParams;
use super::stats::SearchStats;
use super::*;
use crate::cache::counters::CMTable;
use crate::cache::history::HTable;
use crate::cache::killers::KTable;
use crate::cache::pawns::PHTable;
use crate::cache::search::TTable;
use crate::engine::clock;
use crate::state::movescan::Move;
use crate::state::representation::Board;
use crate::utils::panic_fast;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::SystemTime;

pub struct SearchContext {
    pub board: Board,
    pub params: SearchParams,
    pub search_id: u8,
    pub time: u32,
    pub inc_time: u32,
    pub current_depth: i8,
    pub forced_depth: i8,
    pub max_nodes_count: u64,
    pub max_soft_nodes_count: u64,
    pub max_move_time: u32,
    pub moves_to_go: u32,
    pub moves_to_search: Vec<Move>,
    pub search_time_start: SystemTime,
    pub time_hard_bound: u32,
    pub multipv: bool,
    pub lines: Vec<SearchResultLine>,
    pub search_done: bool,
    pub uci_debug: bool,
    pub ponder_mode: bool,
    pub soft_nodes: bool,
    pub search_noise: bool,
    pub syzygy_enabled: bool,
    pub syzygy_probe_limit: u32,
    pub syzygy_probe_depth: i8,
    pub ttable: Arc<TTable>,
    pub phtable: Arc<PHTable>,
    pub ktable: KTable,
    pub htable: HTable,
    pub cmtable: CMTable,
    pub helper_contexts: Arc<RwLock<Vec<SearchContext>>>,
    pub abort_flag: Arc<AtomicBool>,
    pub ponder_flag: Arc<AtomicBool>,
    pub stats: SearchStats,
    pub last_score: i16,
}

pub struct SearchResult {
    pub time: u32,
    pub depth: i8,
}

pub struct SearchResultLine {
    pub score: i16,
    pub pv_line: Vec<Move>,
}

impl SearchContext {
    /// Constructs a new instance of [SearchContext] with parameters as follows:
    ///  - `board` - initial position of the board
    ///  - `ttable` - transposition table
    ///  - `phtable` - pawn hash table
    ///  - `abort_flag` - flag used to abort search from the outside of the context
    ///  - `ponder_flag` - flag used to change a search mode from pondering to the regular one
    pub fn new(board: Board, ttable: Arc<TTable>, phtable: Arc<PHTable>, abort_flag: Arc<AtomicBool>, ponder_flag: Arc<AtomicBool>) -> Self {
        Self {
            board,
            params: SearchParams::default(),
            search_id: 0,
            time: 0,
            inc_time: 0,
            current_depth: 1,
            forced_depth: 0,
            max_nodes_count: 0,
            max_soft_nodes_count: 0,
            max_move_time: 0,
            moves_to_go: 0,
            moves_to_search: Vec::new(),
            search_time_start: SystemTime::now(),
            time_hard_bound: 0,
            multipv: false,
            lines: Vec::new(),
            search_done: false,
            uci_debug: false,
            ponder_mode: false,
            soft_nodes: false,
            search_noise: false,
            syzygy_enabled: false,
            syzygy_probe_limit: 0,
            syzygy_probe_depth: 0,
            ttable,
            phtable,
            ktable: KTable::default(),
            htable: HTable::default(),
            cmtable: CMTable::default(),
            helper_contexts: Arc::default(),
            abort_flag,
            ponder_flag,
            stats: SearchStats::default(),
            last_score: 0,
        }
    }
}

impl Iterator for SearchContext {
    type Item = SearchResult;

    /// Performs the next iteration of the search, using data stored in the context. Returns [None] if any of the following conditions is true:
    ///  - the search has been done in the previous iteration or the current depth is about to exceed [MAX_DEPTH] value
    ///  - forced depth is not 0 and the current depth is about to exceed this value
    ///  - soft nodes are enabled and the nodes count exceeded the maximal value
    ///  - instant move is possible
    ///  - Syzygy tablebase move is possible
    ///  - time allocated for the current search has expired
    ///  - mate score has detected and was recognized as reliable
    ///  - search was aborted
    fn next(&mut self) -> Option<Self::Item> {
        // This loop works here as goto, which allows restarting search when switching from pondering mode to regular search within the same iteration
        loop {
            if self.search_done || self.current_depth >= MAX_DEPTH {
                return None;
            }

            if self.forced_depth != 0 && self.current_depth > self.forced_depth {
                return None;
            }

            // If the max depth was reached, but search is in ponder mode, wait for "ponderhit" or "stop" command before executing the last iteration
            if self.ponder_mode && self.forced_depth != 0 && self.current_depth == self.forced_depth {
                loop {
                    if self.abort_flag.load(Ordering::Relaxed) {
                        break;
                    }
                }
            }

            // Check instant move and Syzygy tablebase move only if there's no forced depth to reach
            if self.forced_depth == 0 && self.current_depth == 1 {
                if let Some(r#move) = self.board.get_instant_move() {
                    self.search_done = true;
                    self.lines.push(SearchResultLine::new(0, vec![r#move]));

                    return Some(SearchResult::new(0, self.current_depth));
                }

                if self.syzygy_enabled {
                    if let Some((r#move, score)) = self.board.get_tablebase_move(self.syzygy_probe_limit) {
                        self.search_done = true;
                        self.stats.tb_hits = 1;
                        self.lines.push(SearchResultLine::new(score, vec![r#move]));

                        return Some(SearchResult::new(0, self.current_depth));
                    }
                }
            }

            // With soft nodes enabled, search is stopped after completing the depth instead aborting it in the middle
            if self.soft_nodes && self.max_soft_nodes_count > 0 && self.stats.nodes_count + self.stats.q_nodes_count >= self.max_soft_nodes_count {
                return None;
            }

            let (time_soft_bound, time_hard_bound) = clock::get_time_bounds(self);

            self.time_hard_bound = time_hard_bound;
            self.lines.clear();

            let helper_contexts_arc = self.helper_contexts.clone();
            let mut helper_contexts_lock = helper_contexts_arc.write().unwrap();

            thread::scope(|scope| {
                let depth = self.current_depth;
                let mut threads = Vec::new();

                for helper_context in helper_contexts_lock.iter_mut() {
                    helper_context.forced_depth = depth;
                    threads.push(scope.spawn(move || {
                        search::run(helper_context, depth);
                    }));
                }

                search::run(self, self.current_depth);

                let reset_abort_flag = !self.abort_flag.load(Ordering::Relaxed);
                self.abort_flag.store(true, Ordering::Relaxed);

                for thread in threads {
                    thread.join().unwrap();
                }

                if reset_abort_flag {
                    self.abort_flag.store(false, Ordering::Relaxed);
                }
            });

            for helper_context in helper_contexts_lock.iter() {
                self.stats += &helper_context.stats;
            }

            if self.abort_flag.load(Ordering::Relaxed) {
                // If ponder flag is set, the search is completly restarted within the same iteration
                if self.ponder_flag.load(Ordering::Relaxed) {
                    self.current_depth = 1;
                    self.forced_depth = 0;
                    self.search_time_start = SystemTime::now();
                    self.stats = SearchStats::default();

                    self.ponder_flag.store(false, Ordering::Relaxed);
                    self.abort_flag.store(false, Ordering::Relaxed);

                    continue;
                } else {
                    if self.uci_debug {
                        println!("info string Search aborted");
                    }

                    return None;
                }
            }

            if self.lines.is_empty() || self.lines[0].pv_line.is_empty() {
                println!("info string Invalid position");
                return None;
            }

            if self.lines[0].pv_line[0].is_empty() {
                panic_fast!("Invalid PV move: {}", self.lines[0].pv_line[0]);
            }

            let search_time = self.search_time_start.elapsed().unwrap().as_millis() as u32;

            self.lines.sort_by(|a, b| a.score.cmp(&b.score).reverse());
            self.current_depth += 1;

            if self.forced_depth == 0 && self.max_nodes_count == 0 {
                if search_time > time_soft_bound {
                    self.search_done = true;
                }

                // Checkmate score must indicate that the depth it was found is equal or smaller than the current one, to prevent endless move sequences
                if is_score_near_checkmate(self.lines[0].score) && self.current_depth >= (CHECKMATE_SCORE - self.lines[0].score.abs()) as i8 {
                    self.search_done = true;
                }
            }

            return Some(SearchResult::new(search_time, self.current_depth - 1));
        }
    }
}

impl SearchResult {
    /// Constructs a new instance of [SearchResult] with stored `time` and `depth`.
    pub fn new(time: u32, depth: i8) -> Self {
        Self { time, depth }
    }
}

impl SearchResultLine {
    /// Constructs a new instance of [SearchResultLine] with stored `score` and `pv_line`.
    pub fn new(score: i16, pv_line: Vec<Move>) -> Self {
        Self { score, pv_line }
    }
}
