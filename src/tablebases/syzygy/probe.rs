use super::bindings::*;
use crate::engine;
use crate::state::movescan::Move;
use crate::state::movescan::MoveFlags;
use crate::state::representation::Board;
use crate::state::*;
use crate::tablebases::WdlDtzResult;
use crate::tablebases::WdlResult;
use std::ffi::CString;
use std::mem::MaybeUninit;
use std::ptr;

/// Loads Syzygy tablebases stored in `syzygy_path` location.
pub fn init(syzygy_path: &str) {
    #[cfg(feature = "syzygy")]
    unsafe {
        tb_init(CString::new(syzygy_path).unwrap().as_ptr());
    }
}

/// Gets maximal pieces count supported by loaded Syzygy tablebases. Returns 0 if the feature is disabled.
pub fn get_max_pieces_count() -> u8 {
    #[cfg(feature = "syzygy")]
    unsafe {
        return TB_LARGEST as u8;
    }

    0
}

/// Gets WDL (Win-Draw-Loss) for the position specified in `board`. Returns [None] if data couldn't be obtained or the feature is disabled.
pub fn get_wdl(board: &Board) -> Option<WdlResult> {
    #[cfg(feature = "syzygy")]
    unsafe {
        let wdl = tb_probe_wdl(
            board.occupancy[WHITE],
            board.occupancy[BLACK],
            board.pieces[WHITE][KING] | board.pieces[BLACK][KING],
            board.pieces[WHITE][QUEEN] | board.pieces[BLACK][QUEEN],
            board.pieces[WHITE][ROOK] | board.pieces[BLACK][ROOK],
            board.pieces[WHITE][BISHOP] | board.pieces[BLACK][BISHOP],
            board.pieces[WHITE][KNIGHT] | board.pieces[BLACK][KNIGHT],
            board.pieces[WHITE][PAWN] | board.pieces[BLACK][PAWN],
            0,
            0,
            0,
            board.stm == WHITE,
        );

        return match wdl {
            TB_WIN => Some(WdlResult::Win),
            TB_LOSS => Some(WdlResult::Loss),
            TB_DRAW | TB_CURSED_WIN | TB_BLESSED_LOSS => Some(WdlResult::Draw),
            _ => None,
        };
    }

    None
}

/// Gets WDL (Win-Draw-Loss), DTZ (Distance To Zeroing) and the best move for the position specified in `board`.
/// Returns [None] if data couldn't be obtained or the feature is disabled.
pub fn get_root_wdl_dtz(board: &Board) -> Option<WdlDtzResult> {
    #[cfg(feature = "syzygy")]
    unsafe {
        let result = tb_probe_root(
            board.occupancy[WHITE],
            board.occupancy[BLACK],
            board.pieces[WHITE][KING] | board.pieces[BLACK][KING],
            board.pieces[WHITE][QUEEN] | board.pieces[BLACK][QUEEN],
            board.pieces[WHITE][ROOK] | board.pieces[BLACK][ROOK],
            board.pieces[WHITE][BISHOP] | board.pieces[BLACK][BISHOP],
            board.pieces[WHITE][KNIGHT] | board.pieces[BLACK][KNIGHT],
            board.pieces[WHITE][PAWN] | board.pieces[BLACK][PAWN],
            board.state.halfmove_clock as u32,
            0,
            0,
            board.stm == WHITE,
            ptr::null_mut(),
        );

        let wdl = ((result & TB_RESULT_WDL_MASK) >> TB_RESULT_WDL_SHIFT);
        let wdl = match wdl {
            TB_WIN => WdlResult::Win,
            TB_LOSS => WdlResult::Loss,
            _ => WdlResult::Draw,
        };
        let dtz = ((result & TB_RESULT_DTZ_MASK) >> TB_RESULT_DTZ_SHIFT);

        if result == TB_RESULT_FAILED {
            return None;
        }

        let mut moves = [MaybeUninit::uninit(); engine::MAX_MOVES_COUNT];
        let moves_count = board.get_all_moves(&mut moves, u64::MAX);

        let from = ((result & TB_RESULT_FROM_MASK) >> TB_RESULT_FROM_SHIFT) as usize;
        let to = ((result & TB_RESULT_TO_MASK) >> TB_RESULT_TO_SHIFT) as usize;
        let promotion = ((result & TB_RESULT_PROMOTES_MASK) >> TB_RESULT_PROMOTES_SHIFT);

        let promotion_flags = match promotion {
            TB_PROMOTES_QUEEN => MoveFlags::QUEEN_PROMOTION,
            TB_PROMOTES_ROOK => MoveFlags::ROOK_PROMOTION,
            TB_PROMOTES_BISHOP => MoveFlags::BISHOP_PROMOTION,
            TB_PROMOTES_KNIGHT => MoveFlags::KNIGHT_PROMOTION,
            _ => MoveFlags::SINGLE_PUSH,
        };

        for r#move in &moves[0..moves_count] {
            let r#move = unsafe { r#move.assume_init() };
            if r#move.get_from() == from && r#move.get_to() == to {
                let flags = r#move.get_flags();
                if promotion == 0 || (flags & promotion_flags) == flags {
                    return Some(WdlDtzResult::new(wdl, dtz, r#move));
                }
            }
        }

        return None;
    }

    None
}
