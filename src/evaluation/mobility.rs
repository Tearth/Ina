use super::*;
use crate::state::movescan;
use crate::state::representation::Board;

#[cfg(feature = "dev")]
use crate::tuning::tuner::TunerCoefficient;

pub struct MobilityData {
    knight_mobility: PieceMobility,
    bishop_mobility: PieceMobility,
    rook_mobility: PieceMobility,
    queen_mobility: PieceMobility,
}

pub struct PieceMobility {
    pub inner: i8,
    pub outer: i8,
}

/// Evaluates mobility and part of the king safety on the `board` and returns score from the white color perspective (more than 0 when advantage,
/// less than 0 when disadvantage). This evaluator does two things at once: first, counts all possible moves of knight, bishop, rook, queen
/// (pawns and king are too slow and not very important), and second, sums how many squares around both kings are dangered by enemy side
/// (`dangered_white_king_squares` and `dangered_black_king_squares`). This is used in the safety evaluator, to prevent calculating the same thing twice.
pub fn evaluate(board: &Board, dangered_white_king_squares: &mut u32, dangered_black_king_squares: &mut u32) -> PackedEval {
    let white_mobility_data = get_mobility_data(board, WHITE, dangered_black_king_squares);
    let black_mobility_data = get_mobility_data(board, BLACK, dangered_white_king_squares);

    let knight_mobility_inner = (white_mobility_data.knight_mobility.inner - black_mobility_data.knight_mobility.inner) as i16;
    let bishop_mobility_inner = (white_mobility_data.bishop_mobility.inner - black_mobility_data.bishop_mobility.inner) as i16;
    let rook_mobility_inner = (white_mobility_data.rook_mobility.inner - black_mobility_data.rook_mobility.inner) as i16;
    let queen_mobility_inner = (white_mobility_data.queen_mobility.inner - black_mobility_data.queen_mobility.inner) as i16;

    let knight_mobility_outer = (white_mobility_data.knight_mobility.outer - black_mobility_data.knight_mobility.outer) as i16;
    let bishop_mobility_outer = (white_mobility_data.bishop_mobility.outer - black_mobility_data.bishop_mobility.outer) as i16;
    let rook_mobility_outer = (white_mobility_data.rook_mobility.outer - black_mobility_data.rook_mobility.outer) as i16;
    let queen_mobility_outer = (white_mobility_data.queen_mobility.outer - black_mobility_data.queen_mobility.outer) as i16;

    let knight_mobility_inner = knight_mobility_inner * params::MOBILITY_INNER[KNIGHT];
    let bishop_mobility_inner = bishop_mobility_inner * params::MOBILITY_INNER[BISHOP];
    let rook_mobility_inner = rook_mobility_inner * params::MOBILITY_INNER[ROOK];
    let queen_mobility_inner = queen_mobility_inner * params::MOBILITY_INNER[QUEEN];

    let knight_mobility_outer = knight_mobility_outer * params::MOBILITY_OUTER[KNIGHT];
    let bishop_mobility_outer = bishop_mobility_outer * params::MOBILITY_OUTER[BISHOP];
    let rook_mobility_outer = rook_mobility_outer * params::MOBILITY_OUTER[ROOK];
    let queen_mobility_outer = queen_mobility_outer * params::MOBILITY_OUTER[QUEEN];

    knight_mobility_inner
        + bishop_mobility_inner
        + rook_mobility_inner
        + queen_mobility_inner
        + knight_mobility_outer
        + bishop_mobility_outer
        + rook_mobility_outer
        + queen_mobility_outer
}

fn get_mobility_data(board: &Board, color: usize, dangered_king_squares: &mut u32) -> MobilityData {
    MobilityData {
        knight_mobility: movescan::get_piece_mobility::<KNIGHT>(board, color, dangered_king_squares),
        bishop_mobility: movescan::get_piece_mobility::<BISHOP>(board, color, dangered_king_squares),
        rook_mobility: movescan::get_piece_mobility::<ROOK>(board, color, dangered_king_squares),
        queen_mobility: movescan::get_piece_mobility::<QUEEN>(board, color, dangered_king_squares),
    }
}

/// Gets coefficients of mobility on `board` and assigns indexes starting from `index`. Similarly to [evaluate], both `dangered_white_king_squares` and
/// `dangered_black_king_squares` are accordingly updated.
#[cfg(feature = "dev")]
pub fn get_coefficients(
    board: &Board,
    dangered_white_king_squares: &mut u32,
    dangered_black_king_squares: &mut u32,
    index: &mut u16,
    coefficients: &mut Vec<TunerCoefficient>,
    indices: &mut Vec<u16>,
) {
    let white_mobility_data = get_mobility_data(board, WHITE, dangered_black_king_squares);
    let black_mobility_data = get_mobility_data(board, BLACK, dangered_white_king_squares);

    let mut data = [
        TunerCoefficient::new(0, OPENING),
        TunerCoefficient::new(0, ENDING),
        TunerCoefficient::new(white_mobility_data.knight_mobility.inner - black_mobility_data.knight_mobility.inner, OPENING),
        TunerCoefficient::new(white_mobility_data.knight_mobility.inner - black_mobility_data.knight_mobility.inner, ENDING),
        TunerCoefficient::new(white_mobility_data.bishop_mobility.inner - black_mobility_data.bishop_mobility.inner, OPENING),
        TunerCoefficient::new(white_mobility_data.bishop_mobility.inner - black_mobility_data.bishop_mobility.inner, ENDING),
        TunerCoefficient::new(white_mobility_data.rook_mobility.inner - black_mobility_data.rook_mobility.inner, OPENING),
        TunerCoefficient::new(white_mobility_data.rook_mobility.inner - black_mobility_data.rook_mobility.inner, ENDING),
        TunerCoefficient::new(white_mobility_data.queen_mobility.inner - black_mobility_data.queen_mobility.inner, OPENING),
        TunerCoefficient::new(white_mobility_data.queen_mobility.inner - black_mobility_data.queen_mobility.inner, ENDING),
        TunerCoefficient::new(0, OPENING),
        TunerCoefficient::new(0, ENDING),
        //
        TunerCoefficient::new(0, OPENING),
        TunerCoefficient::new(0, ENDING),
        TunerCoefficient::new(white_mobility_data.knight_mobility.outer - black_mobility_data.knight_mobility.outer, OPENING),
        TunerCoefficient::new(white_mobility_data.knight_mobility.outer - black_mobility_data.knight_mobility.outer, ENDING),
        TunerCoefficient::new(white_mobility_data.bishop_mobility.outer - black_mobility_data.bishop_mobility.outer, OPENING),
        TunerCoefficient::new(white_mobility_data.bishop_mobility.outer - black_mobility_data.bishop_mobility.outer, ENDING),
        TunerCoefficient::new(white_mobility_data.rook_mobility.outer - black_mobility_data.rook_mobility.outer, OPENING),
        TunerCoefficient::new(white_mobility_data.rook_mobility.outer - black_mobility_data.rook_mobility.outer, ENDING),
        TunerCoefficient::new(white_mobility_data.queen_mobility.outer - black_mobility_data.queen_mobility.outer, OPENING),
        TunerCoefficient::new(white_mobility_data.queen_mobility.outer - black_mobility_data.queen_mobility.outer, ENDING),
        TunerCoefficient::new(0, OPENING),
        TunerCoefficient::new(0, ENDING),
    ];

    for coefficient in &mut data {
        let (value, _) = coefficient.get_data();
        if value != 0 {
            indices.push(*index);
            coefficients.push(coefficient.clone());
        }

        *index += 1;
    }
}
