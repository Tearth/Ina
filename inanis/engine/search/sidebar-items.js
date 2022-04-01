initSidebarItems({"constant":[["LATE_MOVE_PRUNING_MAX_DEPTH",""],["LATE_MOVE_PRUNING_MAX_SCORE",""],["LATE_MOVE_PRUNING_MIN_DEPTH",""],["LATE_MOVE_PRUNING_MOVE_INDEX_MARGIN_BASE",""],["LATE_MOVE_PRUNING_MOVE_INDEX_MARGIN_MULTIPLIER",""],["LATE_MOVE_REDUCTION_MAX_REDUCTION",""],["LATE_MOVE_REDUCTION_MAX_SCORE",""],["LATE_MOVE_REDUCTION_MIN_DEPTH",""],["LATE_MOVE_REDUCTION_MIN_MOVE_INDEX",""],["LATE_MOVE_REDUCTION_REDUCTION_BASE",""],["LATE_MOVE_REDUCTION_REDUCTION_STEP",""],["LAZY_SMP_NOISE",""],["MOVE_ORDERING_BISHOP_PROMOTION",""],["MOVE_ORDERING_CASTLING",""],["MOVE_ORDERING_HASH_MOVE",""],["MOVE_ORDERING_HISTORY_MOVE",""],["MOVE_ORDERING_HISTORY_MOVE_OFFSET",""],["MOVE_ORDERING_KILLER_MOVE",""],["MOVE_ORDERING_KNIGHT_PROMOTION",""],["MOVE_ORDERING_LOSING_CAPTURES_OFFSET",""],["MOVE_ORDERING_QUEEN_PROMOTION",""],["MOVE_ORDERING_ROOK_PROMOTION",""],["MOVE_ORDERING_WINNING_CAPTURES_OFFSET",""],["NULL_MOVE_BIG_R",""],["NULL_MOVE_MIN_DEPTH",""],["NULL_MOVE_MIN_GAME_PHASE",""],["NULL_MOVE_R_CHANGE_DEPTH",""],["NULL_MOVE_SMALL_R",""],["RAZORING_DEPTH_MARGIN_BASE",""],["RAZORING_DEPTH_MARGIN_MULTIPLIER",""],["RAZORING_MAX_DEPTH",""],["RAZORING_MIN_DEPTH",""],["REDUCTION_PRUNING_DEPTH_THRESHOLD",""],["STATIC_NULL_MOVE_PRUNING_DEPTH_MARGIN_BASE",""],["STATIC_NULL_MOVE_PRUNING_DEPTH_MARGIN_MULTIPLIER",""],["STATIC_NULL_MOVE_PRUNING_MAX_DEPTH",""],["STATIC_NULL_MOVE_PRUNING_MIN_DEPTH",""]],"enum":[["MoveGeneratorStage",""]],"fn":[["assign_move_scores","Assigns scores for `moves` by filling `move_scores` array with `moves_count` length (starting from `start_index`), based on current `context`. If transposition table move is available, it’s passed as `tt_move` too. Moves are prioritized as follows (from most important to the less ones):"],["get_next_move","Gets a next move to analyze. This function acts as pseudo-iterator and takes care about managing move generator stages, which is basically a state machine (https://en.wikipedia.org/wiki/Finite-state_machine) with following rules:"],["late_move_pruning_can_be_applied","The main idea of the late move pruning is to prune all nodes, which are near the horizon and were scored low by the history table. We assume here, that there’s a little chance that move being near the end of the list will improve score, so there’s no point of spending time here."],["late_move_reduction_can_be_applied","The main idea of the late move reduction is to reduce search depth of all quiet moves, which aren’t promising and with high chance won’t improve score. This is the least risky type of pruning (used inside PVS framework which cares about re-search when the move is better than expected), so it’s also applied in PV nodes."],["late_move_reduction_get_r","Gets the late move depth reduction, called R, based on `move_index`. The lower the move was scored, the larger reduction will be returned."],["null_move_pruning_can_be_applied","The main idea of the null move pruning is to prune all nodes, for which the search gives us score above beta even if we skip a move (which allows the opposite color to make two of them in a row). This is based on the null move observation, which says that there’s always a better alternative than doing nothing (except zugzwang)."],["null_move_pruning_get_r","Gets the null move pruning depth reduction, called R, based on `depth`. It returns [NULL_MOVE_SMALL_R] if `depth` is less or equal to [NULL_MOVE_R_CHANGE_DEPTH], otherwise [NULL_MOVE_BIG_R]."],["razoring_can_be_applied","The main idea of the razoring is to detect and prune all nodes, which (based on lazy evaluation) are hopeless compared to the current alpha and the chance to improve the score is too small to spend time here. To make it more safe and not to skip positions where we’re somewhere in the middle of capture sequence, there’s the quiescence search performed to verify if the final score is still below alpha - margin."],["razoring_get_margin","Gets the razoring margin, based on `depth`. The further from the horizon we are, the more margin we should take to determine if node can be pruned."],["reduction_pruning_can_be_applied","The main idea of the reduction pruning is to prune all nodes, for which the calculated earlier reduction is so big, that it’s beyond regular search and would fall directly into the quiescence search, or near to it."],["run","Entry point of the regular search, with generic `PV` parameter determining if the current node is a PV (principal variation) node in the PVS framework. The implementation contains a typical alpha-beta approach, together with a bunch of reduction and prunings to optimize search. The most important parameter here, `context` contains the current state of the search, board state, statistics, and is passed by reference to all nodes. Besides obvious parameters like `depth`, `ply`, `alpha` and `beta`, there’s also `allow_null_move` which prevents two null move checks in a row, and `friendly_king_checked` which is used to share friendly king check status between nodes (it’s always calculated one depth earlier, as it’s used as one of the LMR constraints)."],["static_null_move_pruning_can_be_applied","The main idea of the static null move pruning (also called as reverse futility pruning) is to prune all nodes, which (based on lazy evaluation) are too good compared to the current beta, and will very likely be a cut-node. To save time, we skip move loop entirely and return beta + some margin score. The concept is very similar to null move pruning, but without performing any search."],["static_null_move_pruning_get_margin","Gets the static null move pruning margin, based on `depth`. The further from the horizon we are, the more margin should we take to determine if node can be pruned."]]});