use std::cmp;
use std::ops;

#[derive(Default)]
pub struct SearchStats {
    pub nodes_count: u64,
    pub q_nodes_count: u64,
    pub leafs_count: u64,
    pub q_leafs_count: u64,

    pub beta_cutoffs: u64,
    pub q_beta_cutoffs: u64,

    pub tb_hits: u64,

    pub perfect_cutoffs: u64,
    pub q_perfect_cutoffs: u64,
    pub non_perfect_cutoffs: u64,
    pub q_non_perfect_cutoffs: u64,

    pub pvs_full_window_searches: u64,
    pub pvs_zero_window_searches: u64,
    pub pvs_rejected_searches: u64,

    pub snmp_attempts: u64,
    pub snmp_accepted: u64,
    pub snmp_rejected: u64,

    pub nmp_attempts: u64,
    pub nmp_accepted: u64,
    pub nmp_rejected: u64,

    pub lmp_accepted: u64,
    pub lmp_rejected: u64,

    pub razoring_attempts: u64,
    pub razoring_accepted: u64,
    pub razoring_rejected: u64,

    pub q_score_pruning_accepted: u64,
    pub q_score_pruning_rejected: u64,

    pub q_futility_pruning_accepted: u64,
    pub q_futility_pruning_rejected: u64,

    pub tt_added: u64,
    pub tt_hits: u64,
    pub tt_misses: u64,

    pub tt_legal_hashmoves: u64,
    pub tt_illegal_hashmoves: u64,
    pub ktable_legal_moves: u64,
    pub ktable_illegal_moves: u64,
    pub cmtable_legal_moves: u64,
    pub cmtable_illegal_moves: u64,

    pub phtable_added: u64,
    pub phtable_hits: u64,
    pub phtable_misses: u64,

    pub movegen_hash_move_stages: u64,
    pub movegen_captures_stages: u64,
    pub movegen_killers_stages: u64,
    pub movegen_counters_stages: u64,
    pub movegen_quiets_stages: u64,

    pub max_ply: u16,
}

impl ops::AddAssign<&SearchStats> for SearchStats {
    /// Implements `+=` operator for [SearchStats] by adding all corresponding squares together (except `max_ply`, where the highest value is taken).
    fn add_assign(&mut self, rhs: &SearchStats) {
        self.nodes_count += rhs.nodes_count;
        self.q_nodes_count += rhs.q_nodes_count;
        self.leafs_count += rhs.leafs_count;
        self.q_leafs_count += rhs.q_leafs_count;
        self.beta_cutoffs += rhs.beta_cutoffs;
        self.q_beta_cutoffs += rhs.q_beta_cutoffs;

        self.tb_hits += rhs.tb_hits;

        self.perfect_cutoffs += rhs.perfect_cutoffs;
        self.q_perfect_cutoffs += rhs.q_perfect_cutoffs;
        self.non_perfect_cutoffs += rhs.non_perfect_cutoffs;
        self.q_non_perfect_cutoffs += rhs.q_non_perfect_cutoffs;

        self.pvs_full_window_searches += rhs.pvs_full_window_searches;
        self.pvs_zero_window_searches += rhs.pvs_zero_window_searches;
        self.pvs_rejected_searches += rhs.pvs_rejected_searches;

        self.snmp_attempts += rhs.snmp_attempts;
        self.snmp_accepted += rhs.snmp_accepted;
        self.snmp_rejected += rhs.snmp_rejected;

        self.nmp_attempts += rhs.nmp_attempts;
        self.nmp_accepted += rhs.nmp_accepted;
        self.nmp_rejected += rhs.nmp_rejected;

        self.lmp_accepted += rhs.lmp_accepted;
        self.lmp_rejected += rhs.lmp_rejected;

        self.razoring_attempts += rhs.razoring_attempts;
        self.razoring_accepted += rhs.razoring_accepted;
        self.razoring_rejected += rhs.razoring_rejected;

        self.q_score_pruning_accepted += rhs.q_score_pruning_accepted;
        self.q_score_pruning_rejected += rhs.q_score_pruning_rejected;

        self.q_futility_pruning_accepted += rhs.q_futility_pruning_accepted;
        self.q_futility_pruning_rejected += rhs.q_futility_pruning_rejected;

        self.tt_added += rhs.tt_added;
        self.tt_hits += rhs.tt_hits;
        self.tt_misses += rhs.tt_misses;

        self.tt_legal_hashmoves += rhs.tt_legal_hashmoves;
        self.tt_illegal_hashmoves += rhs.tt_illegal_hashmoves;

        self.phtable_added += rhs.phtable_added;
        self.phtable_hits += rhs.phtable_hits;
        self.phtable_misses += rhs.phtable_misses;

        self.movegen_hash_move_stages += rhs.movegen_hash_move_stages;
        self.movegen_captures_stages += rhs.movegen_captures_stages;
        self.movegen_quiets_stages += rhs.movegen_quiets_stages;

        self.max_ply = cmp::max(self.max_ply, rhs.max_ply);
    }
}
