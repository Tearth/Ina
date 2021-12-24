use super::board::Bitboard;
use super::board::CastlingRights;
use super::*;
use crate::engine::*;
use crate::evaluation::parameters::*;
use std::mem::MaybeUninit;

bitflags! {
    pub struct MoveFlags: u8 {
        const QUIET = 0;
        const DOUBLE_PUSH = 1;
        const SHORT_CASTLING = 2;
        const LONG_CASTLING = 3;
        const CAPTURE = 4;
        const EN_PASSANT = 5;
        const UNDEFINED1 = 6;
        const UNDEFINED2 = 7;
        const KNIGHT_PROMOTION = 8;
        const BISHOP_PROMOTION = 9;
        const ROOK_PROMOTION = 10;
        const QUEEN_PROMOTION = 11;
        const KNIGHT_PROMOTION_CAPTURE = 12;
        const BISHOP_PROMOTION_CAPTURE = 13;
        const ROOK_PROMOTION_CAPTURE = 14;
        const QUEEN_PROMOTION_CAPTURE = 15;

        const FIELD_SPECIAL_0 = 1;
        const FIELD_SPECIAL_1 = 2;
        const FIELD_CAPTURE = 4;
        const FIELD_PROMOTION = 8;
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Move {
    pub data: u16,
}

impl Move {
    pub fn new(from: u8, to: u8, flags: MoveFlags) -> Move {
        Move {
            data: ((flags.bits as u16) << 12) | ((to as u16) << 6) | (from as u16),
        }
    }

    pub fn from_short_notation(mut text: &str, board: &Bitboard) -> Result<Move, &'static str> {
        let mut moves: [Move; MAX_MOVES_COUNT] = unsafe { MaybeUninit::uninit().assume_init() };
        let moves_count = board.get_moves::<false>(&mut moves, u64::MAX);

        let mut desired_to: Option<u8> = None;
        let mut desired_file: Option<u8> = None;
        let mut desired_rank: Option<u8> = None;
        let mut desired_piece: Option<u8> = None;
        let mut desired_flags: Option<MoveFlags> = None;
        let mut desired_capture: Option<bool> = None;
        let mut desired_promotion: Option<u8> = None;

        if text.contains('=') {
            desired_promotion = match &text[text.len() - 2..] {
                "=Q" => Some(QUEEN),
                "=R" => Some(ROOK),
                "=B" => Some(BISHOP),
                "=N" => Some(KNIGHT),
                _ => return Err("Invalid promotion piece"),
            }
        }

        text = text.trim_matches('#');
        text = text.trim_matches('+');
        text = text.trim_matches('=');
        text = text.trim_matches('?');
        text = text.trim_matches('!');
        text = text.trim_end_matches('Q');
        text = text.trim_end_matches('R');
        text = text.trim_end_matches('B');
        text = text.trim_end_matches('N');

        if text == "0-0" {
            desired_piece = Some(KING);
            desired_flags = Some(MoveFlags::SHORT_CASTLING);
        } else if text == "0-0-0" {
            desired_piece = Some(KING);
            desired_flags = Some(MoveFlags::LONG_CASTLING);
        } else {
            let mut chars = text.chars();
            match text.len() {
                // e4
                2 => {
                    let file = chars.next().ok_or("Invalid move: bad source file")? as u8;
                    let rank = chars.next().ok_or("Invalid move: bad source rank")? as u8;
                    let to = (7 - (file - b'a')) + 8 * (rank - b'1');

                    desired_to = Some(to);
                    desired_piece = Some(PAWN);
                }
                // Nd5
                3 => {
                    let piece = chars.next().ok_or("Invalid move: bad source file")?;
                    let file = chars.next().ok_or("Invalid move: bad source file")? as u8;
                    let rank = chars.next().ok_or("Invalid move: bad source rank")? as u8;
                    let to = (7 - (file - b'a')) + 8 * (rank - b'1');
                    let piece_type = symbol_to_piece(piece)?;

                    desired_to = Some(to);
                    desired_piece = Some(piece_type);
                }
                // exf5, Rxf5, N3e4, Nde4
                4 => {
                    let piece_or_file = chars.next().ok_or("Invalid move: bad source file")?;
                    let capture_or_file_rank = chars.next().ok_or("Invalid move: symbol")?;
                    let file = chars.next().ok_or("Invalid move: bad source file")? as u8;
                    let rank = chars.next().ok_or("Invalid move: bad source rank")? as u8;
                    let to = (7 - (file - b'a')) + 8 * (rank - b'1');

                    // exf5, Rxf5
                    if capture_or_file_rank == 'x' {
                        // exf5
                        if piece_or_file.is_lowercase() {
                            let file_from = 7 - ((piece_or_file as u8) - b'a');

                            desired_to = Some(to);
                            desired_file = Some(file_from);
                            desired_piece = Some(PAWN);
                            desired_capture = Some(true);
                        // Rxf5
                        } else {
                            let piece_type = symbol_to_piece(piece_or_file)?;

                            desired_to = Some(to);
                            desired_piece = Some(piece_type);
                            desired_capture = Some(true);
                        }
                    // N3e4
                    } else if capture_or_file_rank.is_digit(10) {
                        let piece_type = symbol_to_piece(piece_or_file)?;
                        let rank_from = (capture_or_file_rank as u8) - b'1';

                        desired_to = Some(to);
                        desired_piece = Some(piece_type);
                        desired_rank = Some(rank_from);
                    }
                    // Nde4
                    else {
                        let file_from = 7 - ((capture_or_file_rank as u8) - b'a');
                        let piece_type = symbol_to_piece(piece_or_file)?;

                        desired_to = Some(to);
                        desired_piece = Some(piece_type);
                        desired_file = Some(file_from);
                    }
                }
                // R2xc2, Rexc2
                5 => {
                    let piece = chars.next().ok_or("Invalid move: bad source file")?;
                    let file_rank = chars.next().ok_or("Invalid move: symbol")?;
                    let _ = chars.next().ok_or("Invalid move: symbol")?;
                    let file = chars.next().ok_or("Invalid move: bad source file")? as u8;
                    let rank = chars.next().ok_or("Invalid move: bad source rank")? as u8;
                    let to = (7 - (file - b'a')) + 8 * (rank - b'1');
                    let piece_type = symbol_to_piece(piece)?;

                    // R2xc2
                    if file_rank.is_digit(10) {
                        let rank_from = (file_rank as u8) - b'1';

                        desired_to = Some(to);
                        desired_rank = Some(rank_from);
                        desired_piece = Some(PAWN);
                        desired_capture = Some(true);
                    // Rexc2
                    } else {
                        let file_from = 7 - ((file_rank as u8) - b'a');

                        desired_to = Some(to);
                        desired_file = Some(file_from);
                        desired_piece = Some(piece_type);
                        desired_capture = Some(true);
                    }
                }
                _ => return Err("Invalid move: unknown length"),
            }
        }

        for move_index in 0..moves_count {
            let r#move = moves[move_index];
            if (desired_to.is_none() || desired_to.unwrap() == r#move.get_to())
                && (desired_file.is_none() || (r#move.get_from() % 8) == desired_file.unwrap())
                && (desired_rank.is_none() || (r#move.get_from() / 8) == desired_rank.unwrap())
                && (desired_piece.is_none() || board.get_piece(r#move.get_from()) == desired_piece.unwrap())
                && (desired_flags.is_none() || r#move.get_flags() == desired_flags.unwrap())
                && (desired_capture.is_none() || r#move.is_capture() == desired_capture.unwrap())
                && (desired_promotion.is_none() || (r#move.is_promotion() && r#move.get_promotion_piece() == desired_promotion.unwrap()))
            {
                return Ok(r#move);
            }
        }

        Err("Invalid move")
    }

    pub fn from_long_notation(text: &str, board: &Bitboard) -> Result<Move, &'static str> {
        let mut chars = text.chars();
        let from_file = chars.next().ok_or("Invalid move: bad source file")? as u8;
        let from_rank = chars.next().ok_or("Invalid move: bad source rank")? as u8;
        let to_file = chars.next().ok_or("Invalid move: bad destination file")? as u8;
        let to_rank = chars.next().ok_or("Invalid move: bad destination rank")? as u8;
        let promotion = chars.next();

        if !(b'a'..=b'h').contains(&from_file) || !(b'a'..=b'h').contains(&to_file) {
            return Err("Invalid move: bad source field");
        }

        if !(b'1'..=b'8').contains(&from_rank) || !(b'1'..=b'8').contains(&to_rank) {
            return Err("Invalid move: bad destination field");
        }

        if let Some(promotion_piece) = promotion {
            if !['n', 'b', 'r', 'q'].contains(&promotion_piece) {
                return Err("Invalid move: bad promotion piece");
            }
        }

        let from = (7 - (from_file - b'a')) + 8 * (from_rank - b'1');
        let to = (7 - (to_file - b'a')) + 8 * (to_rank - b'1');
        let promotion_flags = match promotion {
            Some(promotion_piece) => {
                let mut flags = match promotion_piece {
                    'n' => MoveFlags::KNIGHT_PROMOTION,
                    'b' => MoveFlags::BISHOP_PROMOTION,
                    'r' => MoveFlags::ROOK_PROMOTION,
                    'q' => MoveFlags::QUEEN_PROMOTION,
                    _ => panic!("Invalid value: promotion_piece={}", promotion_piece),
                };

                if ((from as i8) - (to as i8)).abs() != 8 {
                    flags |= MoveFlags::CAPTURE;
                };

                flags
            }
            None => MoveFlags::QUIET,
        };

        let mut moves: [Move; MAX_MOVES_COUNT] = unsafe { MaybeUninit::uninit().assume_init() };
        let moves_count = board.get_moves::<false>(&mut moves, u64::MAX);

        for r#move in &moves[0..moves_count] {
            if r#move.get_from() == from && r#move.get_to() == to {
                let flags = r#move.get_flags();
                if promotion_flags == MoveFlags::QUIET || (flags & promotion_flags).bits == flags.bits {
                    return Ok(*r#move);
                }
            }
        }

        Err("Invalid move: not found")
    }

    pub fn to_long_notation(self) -> String {
        let from = self.get_from();
        let to = self.get_to();

        let mut result = vec![
            char::from(b'a' + (7 - from % 8)),
            char::from(b'1' + from / 8),
            char::from(b'a' + (7 - to % 8)),
            char::from(b'1' + to / 8),
        ];

        let flags = self.get_flags();
        if flags.contains(MoveFlags::KNIGHT_PROMOTION) {
            result.push(match flags {
                MoveFlags::KNIGHT_PROMOTION | MoveFlags::KNIGHT_PROMOTION_CAPTURE => 'n',
                MoveFlags::BISHOP_PROMOTION | MoveFlags::BISHOP_PROMOTION_CAPTURE => 'b',
                MoveFlags::ROOK_PROMOTION | MoveFlags::ROOK_PROMOTION_CAPTURE => 'r',
                MoveFlags::QUEEN_PROMOTION | MoveFlags::QUEEN_PROMOTION_CAPTURE => 'q',
                _ => panic!("Invalid value: flags={:?}", flags),
            });
        }

        result.into_iter().collect()
    }

    pub fn get_from(&self) -> u8 {
        (self.data & 0x3f) as u8
    }

    pub fn get_to(&self) -> u8 {
        ((self.data >> 6) & 0x3f) as u8
    }

    pub fn get_flags(&self) -> MoveFlags {
        unsafe { MoveFlags::from_bits_unchecked((self.data >> 12) as u8) }
    }

    pub fn get_promotion_piece(&self) -> u8 {
        match self.get_flags() {
            MoveFlags::KNIGHT_PROMOTION | MoveFlags::KNIGHT_PROMOTION_CAPTURE => KNIGHT,
            MoveFlags::BISHOP_PROMOTION | MoveFlags::BISHOP_PROMOTION_CAPTURE => BISHOP,
            MoveFlags::ROOK_PROMOTION | MoveFlags::ROOK_PROMOTION_CAPTURE => ROOK,
            MoveFlags::QUEEN_PROMOTION | MoveFlags::QUEEN_PROMOTION_CAPTURE => QUEEN,
            _ => panic!("Invalid value: self.get_flags()={:?}", self.get_flags()),
        }
    }

    pub fn is_quiet(&self) -> bool {
        self.get_flags() == MoveFlags::QUIET || self.get_flags() == MoveFlags::DOUBLE_PUSH
    }

    pub fn is_capture(&self) -> bool {
        self.get_flags().contains(MoveFlags::CAPTURE)
    }

    pub fn is_en_passant(&self) -> bool {
        self.get_flags() == MoveFlags::EN_PASSANT
    }

    pub fn is_promotion(&self) -> bool {
        self.get_flags().contains(MoveFlags::FIELD_PROMOTION)
    }

    pub fn is_castling(&self) -> bool {
        self.get_flags() == MoveFlags::SHORT_CASTLING || self.get_flags() == MoveFlags::LONG_CASTLING
    }
}

impl Default for Move {
    fn default() -> Self {
        Move::new(0, 0, MoveFlags::QUIET)
    }
}

pub fn scan_piece_moves<const PIECE: u8, const CAPTURES_ONLY: bool>(board: &Bitboard, moves: &mut [Move], mut index: usize, evasion_mask: u64) -> usize {
    let enemy_color = board.active_color ^ 1;
    let mut pieces = board.pieces[board.active_color as usize][PIECE as usize];

    while pieces != 0 {
        let from_field = get_lsb(pieces);
        let from_field_index = bit_scan(from_field);
        pieces = pop_lsb(pieces);

        let occupancy = board.occupancy[WHITE as usize] | board.occupancy[BLACK as usize];
        let mut piece_moves = match PIECE {
            KNIGHT => movegen::get_knight_moves(from_field_index as usize),
            BISHOP => movegen::get_bishop_moves(occupancy, from_field_index as usize),
            ROOK => movegen::get_rook_moves(occupancy, from_field_index as usize),
            QUEEN => movegen::get_queen_moves(occupancy, from_field_index as usize),
            KING => movegen::get_king_moves(from_field_index as usize),
            _ => panic!("Invalid value: PIECE={}", PIECE),
        } & !board.occupancy[board.active_color as usize]
            & evasion_mask;

        if CAPTURES_ONLY {
            piece_moves &= board.occupancy[enemy_color as usize];
        }

        while piece_moves != 0 {
            let to_field = get_lsb(piece_moves);
            let to_field_index = bit_scan(to_field);
            piece_moves = pop_lsb(piece_moves);

            let capture = (to_field & board.occupancy[enemy_color as usize]) != 0;
            let flags = if CAPTURES_ONLY || capture { MoveFlags::CAPTURE } else { MoveFlags::QUIET };

            moves[index] = Move::new(from_field_index, to_field_index, flags);
            index += 1;
        }

        if PIECE == KING && !CAPTURES_ONLY {
            match board.active_color {
                WHITE => {
                    let king_side_castling_rights = board.castling_rights.contains(CastlingRights::WHITE_SHORT_CASTLING);
                    let king_side_rook_present = (board.pieces[board.active_color as usize][ROOK as usize] & 0x1) != 0;

                    if king_side_castling_rights && king_side_rook_present && (occupancy & 0x6) == 0 {
                        if !board.are_fields_attacked(board.active_color, &[3, 2, 1]) {
                            moves[index] = Move::new(3, 1, MoveFlags::SHORT_CASTLING);
                            index += 1;
                        }
                    }

                    let queen_side_castling_rights = board.castling_rights.contains(CastlingRights::WHITE_LONG_CASTLING);
                    let queen_side_rook_present = (board.pieces[board.active_color as usize][ROOK as usize] & 0x80) != 0;

                    if queen_side_castling_rights && queen_side_rook_present && (occupancy & 0x70) == 0 {
                        if !board.are_fields_attacked(board.active_color, &[3, 4, 5]) {
                            moves[index] = Move::new(3, 5, MoveFlags::LONG_CASTLING);
                            index += 1;
                        }
                    }
                }
                BLACK => {
                    let king_side_castling_rights = board.castling_rights.contains(CastlingRights::BLACK_SHORT_CASTLING);
                    let king_side_rook_present = (board.pieces[board.active_color as usize][ROOK as usize] & 0x100000000000000) != 0;

                    if king_side_castling_rights && king_side_rook_present && (occupancy & 0x600000000000000) == 0 {
                        if !board.are_fields_attacked(board.active_color, &[59, 58, 57]) {
                            moves[index] = Move::new(59, 57, MoveFlags::SHORT_CASTLING);
                            index += 1;
                        }
                    }

                    let queen_side_castling_rights = board.castling_rights.contains(CastlingRights::BLACK_LONG_CASTLING);
                    let queen_side_rook_present = (board.pieces[board.active_color as usize][ROOK as usize] & 0x8000000000000000) != 0;

                    if queen_side_castling_rights && queen_side_rook_present && (occupancy & 0x7000000000000000) == 0 {
                        if !board.are_fields_attacked(board.active_color, &[59, 60, 61]) {
                            moves[index] = Move::new(59, 61, MoveFlags::LONG_CASTLING);
                            index += 1;
                        }
                    }
                }
                _ => panic!("Invalid value: board.active_color={}", board.active_color),
            }
        }
    }

    index
}

pub fn get_piece_mobility<const PIECE: u8>(board: &Bitboard, color: u8, attack_mask: &mut u64) -> i16 {
    let mut pieces = board.pieces[color as usize][PIECE as usize];
    let mut mobility = 0;

    while pieces != 0 {
        let from_field = get_lsb(pieces);
        let from_field_index = bit_scan(from_field);
        pieces = pop_lsb(pieces);

        let occupancy = board.occupancy[WHITE as usize] | board.occupancy[BLACK as usize];
        let piece_moves = match PIECE {
            KNIGHT => movegen::get_knight_moves(from_field_index as usize),
            BISHOP => movegen::get_bishop_moves(occupancy, from_field_index as usize),
            ROOK => movegen::get_rook_moves(occupancy, from_field_index as usize),
            QUEEN => movegen::get_queen_moves(occupancy, from_field_index as usize),
            KING => movegen::get_king_moves(from_field_index as usize),
            _ => panic!("Invalid value: PIECE={}", PIECE),
        } & !board.occupancy[color as usize];

        let center_mobility = unsafe { MOBILITY_CENTER_MULTIPLIER } * bit_count(piece_moves & CENTER) as i16;
        let outside_mobility = bit_count(piece_moves & OUTSIDE) as i16;

        mobility += center_mobility + outside_mobility;
        *attack_mask |= piece_moves;
    }

    mobility
}

pub fn scan_pawn_moves<const CAPTURES_ONLY: bool>(board: &Bitboard, moves: &mut [Move], mut index: usize, evasion_mask: u64) -> usize {
    if !CAPTURES_ONLY {
        index = scan_pawn_moves_single_push(board, moves, index, evasion_mask);
        index = scan_pawn_moves_double_push(board, moves, index, evasion_mask);
    }

    index = scan_pawn_moves_diagonal_attacks::<LEFT>(board, moves, index, evasion_mask);
    index = scan_pawn_moves_diagonal_attacks::<RIGHT>(board, moves, index, evasion_mask);
    index
}

fn scan_pawn_moves_single_push(board: &Bitboard, moves: &mut [Move], mut index: usize, evasion_mask: u64) -> usize {
    let pieces = board.pieces[board.active_color as usize][PAWN as usize];
    let occupancy = board.occupancy[WHITE as usize] | board.occupancy[BLACK as usize];

    let shift = 8 - 16 * (board.active_color as i8);
    let promotion_line = RANK_H >> (56 * (board.active_color as u8));
    let mut target_fields = match board.active_color {
        WHITE => (pieces << 8),
        BLACK => (pieces >> 8),
        _ => {
            panic!("Invalid value: board.active_color={}", board.active_color);
        }
    } & !occupancy
        & evasion_mask;

    while target_fields != 0 {
        let to_field = get_lsb(target_fields);
        let to_field_index = bit_scan(to_field);
        let from_field_index = ((to_field_index as i8) - shift) as u8;
        target_fields = pop_lsb(target_fields);

        if (to_field & promotion_line) != 0 {
            moves[index + 0] = Move::new(from_field_index, to_field_index, MoveFlags::QUEEN_PROMOTION);
            moves[index + 1] = Move::new(from_field_index, to_field_index, MoveFlags::ROOK_PROMOTION);
            moves[index + 2] = Move::new(from_field_index, to_field_index, MoveFlags::BISHOP_PROMOTION);
            moves[index + 3] = Move::new(from_field_index, to_field_index, MoveFlags::KNIGHT_PROMOTION);
            index += 4;
        } else {
            moves[index] = Move::new(from_field_index, to_field_index, MoveFlags::QUIET);
            index += 1;
        }
    }

    index
}

fn scan_pawn_moves_double_push(board: &Bitboard, moves: &mut [Move], mut index: usize, evasion_mask: u64) -> usize {
    let pieces = board.pieces[board.active_color as usize][PAWN as usize];
    let occupancy = board.occupancy[WHITE as usize] | board.occupancy[BLACK as usize];

    let shift = 16 - 32 * (board.active_color as i8);
    let mut target_fields = match board.active_color {
        WHITE => ((((pieces & RANK_B) << 8) & !occupancy) << 8),
        BLACK => ((((pieces & RANK_G) >> 8) & !occupancy) >> 8),
        _ => {
            panic!("Invalid value: board.active_color={}", board.active_color);
        }
    } & !occupancy
        & evasion_mask;

    while target_fields != 0 {
        let to_field = get_lsb(target_fields);
        let to_field_index = bit_scan(to_field);
        let from_field_index = ((to_field_index as i8) - shift) as u8;
        target_fields = pop_lsb(target_fields);

        moves[index] = Move::new(from_field_index, to_field_index, MoveFlags::DOUBLE_PUSH);
        index += 1;
    }

    index
}

fn scan_pawn_moves_diagonal_attacks<const DIR: u8>(board: &Bitboard, moves: &mut [Move], mut index: usize, evasion_mask: u64) -> usize {
    let enemy_color = board.active_color ^ 1;
    let pieces = board.pieces[board.active_color as usize][PAWN as usize];

    let forbidden_file = FILE_A >> (DIR * 7);
    let shift = 9 - (board.active_color ^ DIR) * 2;
    let signed_shift = (shift as i8) - ((board.active_color as i8) * 2 * (shift as i8));
    let promotion_line = RANK_H >> (56 * (board.active_color as u8));

    let mut target_fields = match board.active_color {
        WHITE => ((pieces & !forbidden_file) << shift),
        BLACK => ((pieces & !forbidden_file) >> shift),
        _ => {
            panic!("Invalid value: board.active_color={}", board.active_color);
        }
    } & (board.occupancy[enemy_color as usize] | board.en_passant)
        & evasion_mask;

    while target_fields != 0 {
        let to_field = get_lsb(target_fields);
        let to_field_index = bit_scan(to_field);
        let from_field_index = ((to_field_index as i8) - signed_shift) as u8;
        target_fields = pop_lsb(target_fields);

        if (to_field & promotion_line) != 0 {
            moves[index + 0] = Move::new(from_field_index, to_field_index, MoveFlags::QUEEN_PROMOTION_CAPTURE);
            moves[index + 1] = Move::new(from_field_index, to_field_index, MoveFlags::ROOK_PROMOTION_CAPTURE);
            moves[index + 2] = Move::new(from_field_index, to_field_index, MoveFlags::BISHOP_PROMOTION_CAPTURE);
            moves[index + 3] = Move::new(from_field_index, to_field_index, MoveFlags::KNIGHT_PROMOTION_CAPTURE);
            index += 4;
        } else {
            let en_passant = (to_field & board.en_passant) != 0;
            let flags = if en_passant { MoveFlags::EN_PASSANT } else { MoveFlags::CAPTURE };

            moves[index] = Move::new(from_field_index, to_field_index, flags);
            index += 1;
        }
    }

    index
}
