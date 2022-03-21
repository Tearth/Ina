use super::context::SearchContext;
use super::qsearch;
use super::*;
use crate::cache::search::TranspositionTableScoreType;
use crate::state::movegen;
use crate::state::movescan::Move;
use crate::state::*;
use chrono::Utc;
use std::cmp;
use std::mem::MaybeUninit;

pub const RAZORING_MIN_DEPTH: i8 = 1;
pub const RAZORING_MAX_DEPTH: i8 = 4;
pub const RAZORING_DEPTH_MARGIN_BASE: i16 = 300;
pub const RAZORING_DEPTH_MARGIN_MULTIPLIER: i16 = 300;

pub const STATIC_NULL_MOVE_PRUNING_MIN_DEPTH: i8 = 1;
pub const STATIC_NULL_MOVE_PRUNING_MAX_DEPTH: i8 = 8;
pub const STATIC_NULL_MOVE_PRUNING_DEPTH_MARGIN_BASE: i16 = 150;
pub const STATIC_NULL_MOVE_PRUNING_DEPTH_MARGIN_MULTIPLIER: i16 = 150;

pub const NULL_MOVE_MIN_DEPTH: i8 = 2;
pub const NULL_MOVE_R_CHANGE_DEPTH: i8 = 5;
pub const NULL_MOVE_MIN_GAME_PHASE: f32 = 0.15;
pub const NULL_MOVE_SMALL_R: i8 = 2;
pub const NULL_MOVE_BIG_R: i8 = 3;

pub const LATE_MOVE_PRUNING_MIN_DEPTH: i8 = 1;
pub const LATE_MOVE_PRUNING_MAX_DEPTH: i8 = 3;
pub const LATE_MOVE_PRUNING_MOVE_INDEX_MARGIN_BASE: usize = 5;
pub const LATE_MOVE_PRUNING_MOVE_INDEX_MARGIN_MULTIPLIER: usize = 10;
pub const LATE_MOVE_PRUNING_MAX_SCORE: i16 = 0;

pub const LATE_MOVE_REDUCTION_MIN_DEPTH: i8 = 2;
pub const LATE_MOVE_REDUCTION_MIN_MOVE_INDEX: usize = 2;
pub const LATE_MOVE_REDUCTION_MAX_SCORE: i16 = 90;
pub const LATE_MOVE_REDUCTION_REDUCTION_BASE: usize = 1;
pub const LATE_MOVE_REDUCTION_REDUCTION_STEP: usize = 4;
pub const LATE_MOVE_REDUCTION_MAX_REDUCTION: i8 = 3;

pub const REDUCTION_PRUNING_DEPTH_THRESHOLD: i8 = 0;

pub const MOVE_ORDERING_HASH_MOVE: i16 = 10000;
pub const MOVE_ORDERING_WINNING_CAPTURES_OFFSET: i16 = 100;
pub const MOVE_ORDERING_QUEEN_PROMOTION: i16 = 99;
pub const MOVE_ORDERING_ROOK_PROMOTION: i16 = 98;
pub const MOVE_ORDERING_BISHOP_PROMOTION: i16 = 97;
pub const MOVE_ORDERING_KNIGHT_PROMOTION: i16 = 96;
pub const MOVE_ORDERING_KILLER_MOVE: i16 = 95;
pub const MOVE_ORDERING_CASTLING: i16 = 94;
pub const MOVE_ORDERING_HISTORY_MOVE: u8 = 180;
pub const MOVE_ORDERING_HISTORY_MOVE_OFFSET: i16 = -90;
pub const MOVE_ORDERING_LOSING_CAPTURES_OFFSET: i16 = -100;

pub const LAZY_SMP_NOISE: i16 = 10;

#[derive(std::cmp::PartialEq)]
enum MoveGeneratorStage {
    ReadyToCheckHashMove,
    ReadyToGenerateCaptures,
    Captures,
    ReadyToGenerateQuietMoves,
    AllGenerated,
}

/// Entry point of the regular search, with generic `PV` parameter determining if the current node is a PV (principal variation) node in PVS framework.
/// The implementation contains a typical alpha-beta approach, together with a bunch of reduction and prunings to optimize search. The most important
/// parameter here, `context` contains the current state of the search, board state, statistics, and is passed by reference to all nodes. Besides obvious
/// parameters like `depth`, `ply`, `alpha` and `beta`, there's also `allow_null_move` which prevents two null move checks in a row, and `friendly_king_checked`
/// which is used to share friendly king check status between nodes (it's always calculated one depth earlier, as it's used as one of the LMR constraints).
///
/// Search steps for PV node:
///  - Test of initial constraints: abort flag, forced depth
///  - Test if the enemy king is checked
///  - Test if there's threefold repetition draw, fifty move rule draw or insufficient material draw
///  - Switch to quiescence search if the depth is equal to zero
///  - Read from transposition table, return score if possible or update alpha/beta (<https://www.chessprogramming.org/Transposition_Table>)
///  - Generate evasion mask if the friendly king is checked
///  - Main loop:
///     - Late move reduction (<https://www.chessprogramming.org/Late_Move_Reductions>)
///     - Reduction pruning (<https://www.chessprogramming.org/History_Leaf_Pruning>)
///     - PVS framework (<https://www.chessprogramming.org/Principal_Variation_Search>)
///  - Test of abort flag
///  - Test if stalemate draw is detected
///  - Update transposition table
///
/// Search steps for non-PV node:
///  - Test of initial constraints: abort flag, forced depth, max nodes count
///  - Test if the enemy king is checked
///  - Test if there's threefold repetition draw, fifty move rule draw or insufficient material draw
///  - Switch to quiescence search if the depth is equal to zero
///  - Read from transposition table, return score if possible or update alpha/beta (<https://www.chessprogramming.org/Transposition_Table>)
///  - Razoring (<https://www.chessprogramming.org/Razoring>)
///  - Static null move pruning (<https://www.chessprogramming.org/Reverse_Futility_Pruning>)
///  - Null move pruning (<https://www.chessprogramming.org/Null_Move_Pruning>)
///  - Generate evasion mask if the friendly king is checked
///  - Main loop:
///     - Late move pruning (<https://www.chessprogramming.org/Futility_Pruning#MoveCountBasedPruning>)
///     - Late move reduction (<https://www.chessprogramming.org/Late_Move_Reductions>)
///     - Reduction pruning (<https://www.chessprogramming.org/History_Leaf_Pruning>)
///     - PVS framework (<https://www.chessprogramming.org/Principal_Variation_Search>)
///  - Test of abort flag
///  - Test if stalemate draw is detected
///  - Update transposition table
pub fn run<const PV: bool>(
    context: &mut SearchContext,
    depth: i8,
    ply: u16,
    mut alpha: i16,
    mut beta: i16,
    allow_null_move: bool,
    friendly_king_checked: bool,
) -> i16 {
    if context.abort_token.triggered {
        return INVALID_SCORE;
    }

    if context.forced_depth == 0 && context.max_nodes_count == 0 && context.statistics.nodes_count % 10000 == 0 {
        if (Utc::now() - context.search_time_start).num_milliseconds() > context.deadline as i64 {
            context.abort_token.triggered = true;
            return INVALID_SCORE;
        }
    }

    if PV && context.max_nodes_count != 0 {
        if context.statistics.nodes_count + context.statistics.q_nodes_count >= context.max_nodes_count {
            context.abort_token.triggered = true;
            return INVALID_SCORE;
        }
    }

    context.statistics.nodes_count += 1;

    if context.board.is_king_checked(context.board.active_color ^ 1) {
        context.statistics.leafs_count += 1;
        return CHECKMATE_SCORE - (ply as i16);
    }

    if context.board.is_threefold_repetition_draw() || context.board.is_fifty_move_rule_draw() || context.board.is_insufficient_material_draw() {
        context.statistics.leafs_count += 1;
        return DRAW_SCORE;
    }

    if depth <= 0 {
        context.statistics.leafs_count += 1;
        return qsearch::run(context, depth, ply, alpha, beta);
    }

    let original_alpha = alpha;
    let mut tt_entry_found = false;
    let mut hash_move = Default::default();
    let mut collision = false;

    match context.transposition_table.get(context.board.hash, ply, &mut collision) {
        Some(entry) => {
            context.statistics.tt_hits += 1;

            if entry.best_move != Default::default() {
                if entry.best_move.is_legal(context.board) {
                    hash_move = entry.best_move;
                    context.statistics.tt_legal_hashmoves += 1;
                } else {
                    context.statistics.tt_illegal_hashmoves += 1;
                }
            }

            if ply > 0 && entry.depth >= depth as i8 {
                tt_entry_found = true;
                match entry.get_flags() {
                    TranspositionTableScoreType::ALPHA_SCORE => {
                        if entry.score < beta {
                            beta = entry.score;
                        }
                    }
                    TranspositionTableScoreType::BETA_SCORE => {
                        if entry.score > alpha {
                            alpha = entry.score;
                        }
                    }
                    _ => {
                        if !PV || entry.get_age() == 0 {
                            context.statistics.leafs_count += 1;
                            return entry.score;
                        }
                    }
                }

                if alpha >= beta {
                    context.statistics.leafs_count += 1;
                    context.statistics.beta_cutoffs += 1;
                    return entry.score;
                }
            }
        }
        None => {
            if collision {
                context.statistics.tt_collisions += 1;
            }

            context.statistics.tt_misses += 1;
        }
    };

    if razoring_can_be_applied::<PV>(depth, alpha, friendly_king_checked) {
        let margin = razoring_get_margin(depth);
        let lazy_evaluation = -((context.board.active_color as i16) * 2 - 1) * context.board.evaluate_lazy();

        context.statistics.razoring_attempts += 1;
        if lazy_evaluation + margin <= alpha {
            let score = qsearch::run(context, depth, ply, alpha, beta);
            if score <= alpha {
                context.statistics.leafs_count += 1;
                context.statistics.razoring_accepted += 1;
                return score;
            } else {
                context.statistics.razoring_rejected += 1;
            }
        }
    }

    if static_null_move_pruning_can_be_applied::<PV>(depth, beta, friendly_king_checked) {
        let margin = static_null_move_pruning_get_margin(depth);
        let lazy_evaluation = -((context.board.active_color as i16) * 2 - 1) * context.board.evaluate_lazy();

        context.statistics.static_null_move_pruning_attempts += 1;
        if lazy_evaluation - margin >= beta {
            context.statistics.leafs_count += 1;
            context.statistics.static_null_move_pruning_accepted += 1;
            return lazy_evaluation - margin;
        } else {
            context.statistics.static_null_move_pruning_rejected += 1;
        }
    }

    if null_move_pruning_can_be_applied::<PV>(context, depth, beta, allow_null_move, friendly_king_checked) {
        let r = null_move_pruning_get_r(depth);
        context.statistics.null_move_pruning_attempts += 1;

        context.board.make_null_move();
        let king_checked = context.board.is_king_checked(context.board.active_color);
        let score = -run::<false>(context, depth - r - 1, ply + 1, -beta, -beta + 1, false, king_checked);
        context.board.undo_null_move();

        if score >= beta {
            context.statistics.leafs_count += 1;
            context.statistics.null_move_pruning_accepted += 1;
            return score;
        } else {
            context.statistics.null_move_pruning_rejected += 1;
        }
    }

    let evasion_mask = if friendly_king_checked {
        if context.board.pieces[context.board.active_color as usize][KING as usize] == 0 {
            u64::MAX
        } else {
            let king_field_index = bit_scan(context.board.pieces[context.board.active_color as usize][KING as usize]);
            let occupancy = context.board.occupancy[WHITE as usize] | context.board.occupancy[BLACK as usize];

            movegen::get_queen_moves(occupancy, king_field_index as usize) | movegen::get_knight_moves(king_field_index as usize)
        }
    } else {
        u64::MAX
    };

    let mut best_score = -CHECKMATE_SCORE;
    let mut best_move = Default::default();
    let mut moves: [Move; MAX_MOVES_COUNT] = unsafe { MaybeUninit::uninit().assume_init() };
    let mut move_scores: [i16; MAX_MOVES_COUNT] = unsafe { MaybeUninit::uninit().assume_init() };
    let mut move_generator_stage = MoveGeneratorStage::ReadyToCheckHashMove;

    let mut move_index = 0;
    let mut moves_count = 0;

    while let Some(r#move) = get_next_move(
        context,
        &mut move_generator_stage,
        &mut moves,
        &mut move_scores,
        &mut move_index,
        &mut moves_count,
        hash_move,
        evasion_mask,
        ply,
    ) {
        if late_move_pruning_can_be_applied::<PV>(depth, move_index, move_scores[move_index], friendly_king_checked) {
            context.statistics.late_move_pruning_accepted += 1;
            break;
        } else {
            context.statistics.late_move_pruning_rejected += 1;
        }

        context.board.make_move(r#move);

        let king_checked = context.board.is_king_checked(context.board.active_color);
        let r = if late_move_reduction_can_be_applied(depth, r#move, move_index, move_scores[move_index], friendly_king_checked, king_checked) {
            late_move_reduction_get_r(move_index)
        } else {
            0
        };

        if reduction_pruning_can_be_applied::<PV>(depth, r) {
            context.board.undo_move(r#move);
            context.statistics.reduction_pruning_accepted += 1;

            continue;
        } else {
            context.statistics.reduction_pruning_rejected += 1;
        }

        let score = if PV {
            if move_index == 0 {
                context.statistics.pvs_full_window_searches += 1;
                -run::<true>(context, depth - 1, ply + 1, -beta, -alpha, true, king_checked)
            } else {
                let zero_window_score = -run::<false>(context, depth - r - 1, ply + 1, -alpha - 1, -alpha, true, king_checked);
                context.statistics.pvs_zero_window_searches += 1;

                if zero_window_score > alpha {
                    context.statistics.pvs_rejected_searches += 1;
                    -run::<true>(context, depth - 1, ply + 1, -beta, -alpha, true, king_checked)
                } else {
                    zero_window_score
                }
            }
        } else {
            let zero_window_score = -run::<false>(context, depth - r - 1, ply + 1, -beta, -alpha, true, king_checked);
            context.statistics.pvs_zero_window_searches += 1;

            if zero_window_score > alpha && r > 0 {
                context.statistics.pvs_rejected_searches += 1;
                -run::<false>(context, depth - 1, ply + 1, -beta, -alpha, true, king_checked)
            } else {
                zero_window_score
            }
        };

        context.board.undo_move(r#move);

        if score > best_score {
            best_score = score;
            best_move = r#move;
        }

        if best_score > alpha {
            alpha = best_score;

            if alpha >= beta {
                if r#move.is_quiet() {
                    context.killers_table.add(ply, r#move);
                    context.history_table.add(r#move.get_from(), r#move.get_to(), depth as u8);
                }

                context.statistics.beta_cutoffs += 1;
                if move_index == 0 {
                    context.statistics.perfect_cutoffs += 1;
                } else {
                    context.statistics.non_perfect_cutoffs += 1;
                }

                break;
            }
        }
    }

    if context.abort_token.triggered {
        return INVALID_SCORE;
    }

    if best_score == -CHECKMATE_SCORE + (ply as i16) + 1 && !friendly_king_checked {
        return DRAW_SCORE;
    }

    if !tt_entry_found || alpha != original_alpha {
        let score_type = if alpha <= original_alpha {
            TranspositionTableScoreType::ALPHA_SCORE
        } else if alpha >= beta {
            TranspositionTableScoreType::BETA_SCORE
        } else {
            TranspositionTableScoreType::EXACT_SCORE
        };

        context
            .transposition_table
            .add(context.board.hash, alpha, best_move, depth as i8, ply, score_type);
        context.statistics.tt_added += 1;
    }

    best_score
}

/// Assigns scores for `moves` by filling `move_scores` array with `moves_count` length (starting from `start_index`), based on current `context`.
/// If transposition table move is available, it's passed as `tt_move` too. Moves are prioritized as follows (from most important to the less ones):
///  - for transposition table move, assign [MOVE_ORDERING_HASH_MOVE]
///  - for every positive capture, assign SEE score + [MOVE_ORDERING_WINNING_CAPTURES_OFFSET]
///  - for every promotion (excluding these with capture), assign [MOVE_ORDERING_QUEEN_PROMOTION], [MOVE_ORDERING_ROOK_PROMOTION],
///    [MOVE_ORDERING_BISHOP_PROMOTION] or [MOVE_ORDERING_KNIGHT_PROMOTION]
///  - for every move found in killer table, assign [MOVE_ORDERING_KILLER_MOVE]
///  - for every castling, assign [MOVE_ORDERING_CASTLING]
///  - for every quiet move which wasn't categoried in other categories, assign score from history table + [MOVE_ORDERING_HISTORY_MOVE_OFFSET] + random noise
///    defined by [LAZY_SMP_NOISE] if Lazy SMP is enabled
///  - for every negative capture, assign SEE score + [MOVE_ORDERING_LOSING_CAPTURES_OFFSET]
fn assign_move_scores(context: &SearchContext, moves: &[Move], move_scores: &mut [i16], start_index: usize, moves_count: usize, tt_move: Move, ply: u16) {
    for move_index in start_index..moves_count {
        let r#move = moves[move_index];

        if r#move == tt_move {
            move_scores[move_index] = MOVE_ORDERING_HASH_MOVE;
            continue;
        }

        if r#move.is_quiet() {
            if context.killers_table.exists(ply, r#move) {
                move_scores[move_index] = MOVE_ORDERING_KILLER_MOVE;
                continue;
            }

            let mut value = context.history_table.get(r#move.get_from(), r#move.get_to(), MOVE_ORDERING_HISTORY_MOVE) as i16;
            if context.helper_thread && value + LAZY_SMP_NOISE < MOVE_ORDERING_HISTORY_MOVE as i16 {
                value += fastrand::i16(0..=LAZY_SMP_NOISE);
            }

            value += MOVE_ORDERING_HISTORY_MOVE_OFFSET;
            move_scores[move_index] = value;

            continue;
        } else if r#move.is_capture() {
            if r#move.is_en_passant() {
                move_scores[move_index] = MOVE_ORDERING_WINNING_CAPTURES_OFFSET;
                continue;
            }

            let field = r#move.get_to();
            let attacking_piece = context.board.get_piece(r#move.get_from());
            let captured_piece = context.board.get_piece(r#move.get_to());
            let attackers = context.board.get_attacking_pieces(context.board.active_color ^ 1, field);
            let defenders = context.board.get_attacking_pieces(context.board.active_color, field);
            let see = see::get(attacking_piece, captured_piece, attackers, defenders);

            move_scores[move_index] = if see >= 0 {
                see + MOVE_ORDERING_WINNING_CAPTURES_OFFSET
            } else {
                see + MOVE_ORDERING_LOSING_CAPTURES_OFFSET
            };

            continue;
        } else if r#move.is_promotion() {
            move_scores[move_index] = match r#move.get_promotion_piece() {
                QUEEN => MOVE_ORDERING_QUEEN_PROMOTION,
                ROOK => MOVE_ORDERING_ROOK_PROMOTION,
                BISHOP => MOVE_ORDERING_BISHOP_PROMOTION,
                KNIGHT => MOVE_ORDERING_KNIGHT_PROMOTION,
                _ => panic!("Invalid promotion piece"),
            };

            continue;
        } else if r#move.is_castling() {
            move_scores[move_index] = MOVE_ORDERING_CASTLING;
            continue;
        }

        move_scores[move_index] = 0;
    }
}

/// Gets the next move to analyze. This function acts as pseudo-iterator and takes care about managing move generator stages, which is basically
/// a state machine (<https://en.wikipedia.org/wiki/Finite-state_machine>) with following rules:
///  - [MoveGeneratorStage::ReadyToCheckHashMove] - default state, returns transposition table move if possible
///  - [MoveGeneratorStage::ReadyToGenerateCaptures] - generates all captures in the position
///  - [MoveGeneratorStage::Captures] - returns subsequent elements until the end or score is less than [MOVE_ORDERING_WINNING_CAPTURES_OFFSET]
///  - [MoveGeneratorStage::ReadyToGenerateQuietMoves] - generates all quiet moves in the position
///  - [MoveGeneratorStage::AllGenerated] - returns subsequent elements until the end
///
/// Both [MoveGeneratorStage::ReadyToGenerateCaptures] and [MoveGeneratorStage::ReadyToGenerateQuietMoves] are generating moves and assigning scores
/// for move ordering purposes. If the last stage is set and there are no more moves, [None] is returned.
fn get_next_move(
    context: &mut SearchContext,
    stage: &mut MoveGeneratorStage,
    moves: &mut [Move],
    move_scores: &mut [i16],
    move_index: &mut usize,
    moves_count: &mut usize,
    hash_move: Move,
    evasion_mask: u64,
    ply: u16,
) -> Option<Move> {
    if *stage == MoveGeneratorStage::Captures || *stage == MoveGeneratorStage::AllGenerated {
        *move_index += 1;
    }

    loop {
        match stage {
            MoveGeneratorStage::ReadyToCheckHashMove => {
                *stage = MoveGeneratorStage::ReadyToGenerateCaptures;

                if hash_move != Default::default() {
                    moves[0] = hash_move;
                    move_scores[0] = MOVE_ORDERING_HASH_MOVE;
                    context.statistics.move_generator_hash_move_stages += 1;

                    *moves_count = 1;
                    return Some(hash_move);
                }
            }
            MoveGeneratorStage::ReadyToGenerateCaptures => {
                *moves_count = context.board.get_moves::<true>(moves, 0, evasion_mask);
                context.statistics.move_generator_captures_stages += 1;

                if *moves_count == 0 {
                    *stage = MoveGeneratorStage::ReadyToGenerateQuietMoves;
                    continue;
                }

                assign_move_scores(context, moves, move_scores, 0, *moves_count, hash_move, ply);
                *stage = MoveGeneratorStage::Captures;
            }
            MoveGeneratorStage::Captures => {
                if move_index >= moves_count {
                    *stage = MoveGeneratorStage::ReadyToGenerateQuietMoves;
                    continue;
                }

                let r#move = sort_next_move(moves, move_scores, *move_index, *moves_count);
                if r#move == hash_move {
                    *move_index += 1;
                    continue;
                }

                if move_scores[*move_index] < MOVE_ORDERING_WINNING_CAPTURES_OFFSET {
                    *stage = MoveGeneratorStage::ReadyToGenerateQuietMoves;
                    continue;
                }

                return Some(r#move);
            }
            MoveGeneratorStage::ReadyToGenerateQuietMoves => {
                let original_moves_count = *moves_count;
                *moves_count = context.board.get_moves::<false>(moves, *moves_count, evasion_mask);
                context.statistics.move_generator_quiet_moves_stages += 1;

                assign_move_scores(context, moves, move_scores, original_moves_count, *moves_count, hash_move, ply);

                *stage = MoveGeneratorStage::AllGenerated;
            }
            MoveGeneratorStage::AllGenerated => {
                if move_index >= moves_count {
                    return None;
                }

                let r#move = sort_next_move(moves, move_scores, *move_index, *moves_count);
                if r#move == hash_move {
                    *move_index += 1;
                    continue;
                }

                return Some(r#move);
            }
        }
    }
}

/// The main idea of the razoring is to detect and prune all nodes, which (based on lazy evaluation) are hopeless compared to the current alpha and
/// the chance to improve the score is too small to spend time here. To make it more safe and not to skip positions where we're somewhere in the
/// middle of capture sequence, there's a quiescence search performed to verify if the final score is still below alpha - margin.
///
/// Conditions:
///  - Only non-PV nodes
///  - Depth >= [RAZORING_MIN_DEPTH]
///  - Depth <= [RAZORING_MAX_DEPTH]
///  - Alpha is not a mate score
///  - Friendly king is not checked
fn razoring_can_be_applied<const PV: bool>(depth: i8, alpha: i16, friendly_king_checked: bool) -> bool {
    !PV && depth >= RAZORING_MIN_DEPTH && depth <= RAZORING_MAX_DEPTH && !is_score_near_checkmate(alpha) && !friendly_king_checked
}

/// Gets the razoring margin, based on `depth`. The further from the horizon we are, the more margin should we take to determine if node can be pruned.
fn razoring_get_margin(depth: i8) -> i16 {
    RAZORING_DEPTH_MARGIN_BASE + ((depth - RAZORING_MIN_DEPTH) as i16) * RAZORING_DEPTH_MARGIN_MULTIPLIER
}

/// The main idea of the static null move pruning (also called as reverse futility pruning) is to prune all nodes, which (based on lazy evaluation) are too
/// good compared to the current beta, and will very likely be a cut-node. To save time, we skip move loop entirely and return beta + some margin score.
/// The concept is very similar to null move pruning, but without performing any search.
///
/// Conditions:
///  - Only non-PV nodes
///  - Depth >= [STATIC_NULL_MOVE_PRUNING_MIN_DEPTH]
///  - Depth <= [STATIC_NULL_MOVE_PRUNING_MAX_DEPTH]
///  - Beta is not a mate score
///  - Friendly king is not checked
fn static_null_move_pruning_can_be_applied<const PV: bool>(depth: i8, beta: i16, friendly_king_checked: bool) -> bool {
    !PV && depth >= STATIC_NULL_MOVE_PRUNING_MIN_DEPTH
        && depth <= STATIC_NULL_MOVE_PRUNING_MAX_DEPTH
        && !is_score_near_checkmate(beta)
        && !friendly_king_checked
}

/// Gets the static null move pruning margin, based on `depth`. The further from the horizon we are, the more margin should we take to determine
/// if node can be pruned.
fn static_null_move_pruning_get_margin(depth: i8) -> i16 {
    STATIC_NULL_MOVE_PRUNING_DEPTH_MARGIN_BASE + ((depth - STATIC_NULL_MOVE_PRUNING_MIN_DEPTH) as i16) * STATIC_NULL_MOVE_PRUNING_DEPTH_MARGIN_MULTIPLIER
}

/// The main idea of the null move pruning is to prune all nodes, for which the search gives us score above beta even if we skip a move (which allows
/// the opposite color to make two of them in a row). This is based on the null move observation, which says that there's always a better alternative than
/// doing nothing (except zugzwang).
///
/// Conditions:
///  - Only non-PV nodes
///  - Depth >= [NULL_MOVE_MIN_DEPTH]
///  - Game phase is not indicating endgame
///  - Beta score is not a mate score
///  - Friendly king is not checked
///  - This is not the second null move in a row
fn null_move_pruning_can_be_applied<const PV: bool>(
    context: &mut SearchContext,
    depth: i8,
    beta: i16,
    allow_null_move: bool,
    friendly_king_checked: bool,
) -> bool {
    !PV && allow_null_move
        && depth >= NULL_MOVE_MIN_DEPTH
        && context.board.get_game_phase() > NULL_MOVE_MIN_GAME_PHASE
        && !is_score_near_checkmate(beta)
        && !friendly_king_checked
}

/// Gets the null move pruning depth reduction, called R, based on `depth`. It returns [NULL_MOVE_SMALL_R] if `depth` is less or equal
/// to [NULL_MOVE_R_CHANGE_DEPTH], otherwise [NULL_MOVE_BIG_R].
fn null_move_pruning_get_r(depth: i8) -> i8 {
    if depth <= NULL_MOVE_R_CHANGE_DEPTH {
        NULL_MOVE_SMALL_R
    } else {
        NULL_MOVE_BIG_R
    }
}

/// The main idea of the late move pruning is to prune all nodes, which are near horizon and were scored low by the history table.
/// We assume here, that there's a little chance that move being near the end of the list will improve score, so there's no point of spending time here.
///
/// Conditions:
///  - Only non-PV nodes
///  - Depth >= [LATE_MOVE_PRUNING_MIN_DEPTH]
///  - Depth <= [LATE_MOVE_PRUNING_MAX_DEPTH]
///  - Move index >= [LATE_MOVE_PRUNING_MOVE_INDEX_MARGIN_BASE] + some margin depending on `depth`
///  - Move score <= [LATE_MOVE_PRUNING_MAX_SCORE]
///  - Friendly king is not checked
fn late_move_pruning_can_be_applied<const PV: bool>(depth: i8, move_index: usize, move_score: i16, friendly_king_checked: bool) -> bool {
    !PV && depth >= LATE_MOVE_PRUNING_MIN_DEPTH
        && depth <= LATE_MOVE_PRUNING_MAX_DEPTH
        && move_index >= LATE_MOVE_PRUNING_MOVE_INDEX_MARGIN_BASE + (depth as usize - 1) * LATE_MOVE_PRUNING_MOVE_INDEX_MARGIN_MULTIPLIER
        && move_score <= LATE_MOVE_PRUNING_MAX_SCORE
        && !friendly_king_checked
}

/// The main idea of late move reduction is to reduce search depth of all quiet moves, which aren't promising and with high chance won't improve score.
/// This is the least risky type of pruning (used inside PVS framework which cares about re-search when the move is better than expected),
/// so it's also applied in PV nodes.
///
/// Conditions:
///  - Depth >= [LATE_MOVE_REDUCTION_MIN_DEPTH]
///  - Move index >= [LATE_MOVE_REDUCTION_MIN_MOVE_INDEX]
///  - Move score <= [LATE_MOVE_REDUCTION_MAX_SCORE]
///  - Move is quiet
///  - Friendly king is not checked
///  - Enemy king is not checked
fn late_move_reduction_can_be_applied(
    depth: i8,
    r#move: Move,
    move_index: usize,
    move_score: i16,
    friendly_king_checked: bool,
    enemy_king_checked: bool,
) -> bool {
    depth >= LATE_MOVE_REDUCTION_MIN_DEPTH
        && move_index >= LATE_MOVE_REDUCTION_MIN_MOVE_INDEX
        && move_score <= LATE_MOVE_REDUCTION_MAX_SCORE
        && r#move.is_quiet()
        && !friendly_king_checked
        && !enemy_king_checked
}

/// Gets the late move depth reduction, called R, based on `move_index`. The lower the move was scored, the larger reduction will be returned.
fn late_move_reduction_get_r(move_index: usize) -> i8 {
    cmp::min(
        LATE_MOVE_REDUCTION_MAX_REDUCTION,
        (LATE_MOVE_REDUCTION_REDUCTION_BASE + (move_index - LATE_MOVE_REDUCTION_MIN_MOVE_INDEX) / LATE_MOVE_REDUCTION_REDUCTION_STEP) as i8,
    )
}

/// The main idea of the reduction pruning is to prune all nodes, for which the calculated earlier reduction is so big, tha it's beyond regular
/// search and would fall directly into the quiescence search, or near to it.
///
/// Conditions:
///  - Only non-PV nodes
///  - Depth after reduction falls directly into quiescence search or is near to it
fn reduction_pruning_can_be_applied<const PV: bool>(depth: i8, reduction: i8) -> bool {
    !PV && depth - reduction <= REDUCTION_PRUNING_DEPTH_THRESHOLD
}
