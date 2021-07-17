use std::mem::MaybeUninit;

use crate::{board::*, common::*, helpers::*, movegen::*};

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
    }
}

#[derive(Copy, Clone)]
pub struct Move {
    pub data: u16,
}

impl Move {
    pub fn new(from: u8, to: u8, flags: MoveFlags) -> Move {
        Move {
            data: ((flags.bits as u16) << 12) | ((to as u16) << 6) | (from as u16),
        }
    }

    pub fn from_text(text: &str, board: &Bitboard) -> Move {
        let mut chars = text.chars();
        let from = (7 - ((chars.next().unwrap() as u8) - b'a')) + 8 * ((chars.next().unwrap() as u8) - b'1');
        let to = (7 - ((chars.next().unwrap() as u8) - b'a')) + 8 * ((chars.next().unwrap() as u8) - b'1');

        let mut moves: [Move; 218] = unsafe { MaybeUninit::uninit().assume_init() };
        let moves_count = match board.color_to_move {
            WHITE => board.get_moves::<WHITE>(&mut moves),
            BLACK => board.get_moves::<BLACK>(&mut moves),
            _ => panic!("Invalid value: board.color_to_move={}", board.color_to_move),
        };

        for r#move in &moves[0..moves_count] {
            if r#move.get_from() == from && r#move.get_to() == to {
                return *r#move;
            }
        }

        panic!("Invalid move");
    }

    pub fn to_text(self) -> String {
        let from = self.get_from();
        let to = self.get_to();

        let result = vec![
            char::from(b'a' + (7 - from % 8)),
            char::from(b'1' + from / 8),
            char::from(b'a' + (7 - to % 8)),
            char::from(b'1' + to / 8),
        ];

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
            _ => panic!("Invalid promotion piece"),
        }
    }
}

pub fn scan_piece_moves<const COLOR: u8, const PIECE: u8>(board: &Bitboard, moves: &mut [Move], mut index: usize) -> usize {
    let mut pieces = board.pieces[COLOR as usize][PIECE as usize];
    let enemy_color = COLOR ^ 1;

    while pieces != 0 {
        let from_field = get_lsb(pieces);
        let from_field_index = bit_scan(from_field);
        pieces = pop_lsb(pieces);

        let occupancy = board.occupancy[WHITE as usize] | board.occupancy[BLACK as usize];
        let mut piece_moves = match PIECE {
            KNIGHT => get_knight_moves(from_field_index as usize),
            BISHOP => get_bishop_moves(occupancy, from_field_index as usize),
            ROOK => get_rook_moves(occupancy, from_field_index as usize),
            QUEEN => get_queen_moves(occupancy, from_field_index as usize),
            KING => get_king_moves(from_field_index as usize),
            _ => panic!("Invalid value: PIECE={}", PIECE),
        } & !board.occupancy[COLOR as usize];

        while piece_moves != 0 {
            let to_field = get_lsb(piece_moves);
            let to_field_index = bit_scan(to_field);
            piece_moves = pop_lsb(piece_moves);

            let capture = (to_field & board.occupancy[enemy_color as usize]) != 0;
            let flags = if capture { MoveFlags::CAPTURE } else { MoveFlags::QUIET };

            moves[index] = Move::new(from_field_index, to_field_index, flags);
            index += 1;
        }

        if PIECE == KING {
            match COLOR {
                WHITE => {
                    let king_side_castling_rights = board.castling_rights.contains(CastlingRights::WHITE_SHORT_CASTLING);
                    let king_side_rook_present = (board.pieces[COLOR as usize][ROOK as usize] & 0x1) != 0;

                    if king_side_castling_rights && king_side_rook_present && (occupancy & 0x6) == 0 {
                        if !board.are_fields_attacked::<COLOR>(&[3, 2, 1]) {
                            moves[index] = Move::new(3, 1, MoveFlags::SHORT_CASTLING);
                            index += 1;
                        }
                    }

                    let queen_side_castling_rights = board.castling_rights.contains(CastlingRights::WHITE_LONG_CASTLING);
                    let queen_side_rook_present = (board.pieces[COLOR as usize][ROOK as usize] & 0x80) != 0;

                    if queen_side_castling_rights && queen_side_rook_present && (occupancy & 0x70) == 0 {
                        if !board.are_fields_attacked::<COLOR>(&[3, 4, 5]) {
                            moves[index] = Move::new(3, 5, MoveFlags::LONG_CASTLING);
                            index += 1;
                        }
                    }
                }
                BLACK => {
                    let king_side_castling_rights = board.castling_rights.contains(CastlingRights::BLACK_SHORT_CASTLING);
                    let king_side_rook_present = (board.pieces[COLOR as usize][ROOK as usize] & 0x100000000000000) != 0;

                    if king_side_castling_rights && king_side_rook_present && (occupancy & 0x600000000000000) == 0 {
                        if !board.are_fields_attacked::<COLOR>(&[59, 58, 57]) {
                            moves[index] = Move::new(59, 57, MoveFlags::SHORT_CASTLING);
                            index += 1;
                        }
                    }

                    let queen_side_castling_rights = board.castling_rights.contains(CastlingRights::BLACK_LONG_CASTLING);
                    let queen_side_rook_present = (board.pieces[COLOR as usize][ROOK as usize] & 0x8000000000000000) != 0;

                    if queen_side_castling_rights && queen_side_rook_present && (occupancy & 0x7000000000000000) == 0 {
                        if !board.are_fields_attacked::<COLOR>(&[59, 60, 61]) {
                            moves[index] = Move::new(59, 61, MoveFlags::LONG_CASTLING);
                            index += 1;
                        }
                    }
                }
                _ => panic!("Invalid value: COLOR={}", COLOR),
            }
        }
    }

    index
}

pub fn scan_pawn_moves<const COLOR: u8>(board: &Bitboard, moves: &mut [Move], mut index: usize) -> usize {
    index = scan_pawn_moves_single_push::<COLOR>(board, moves, index);
    index = scan_pawn_moves_double_push::<COLOR>(board, moves, index);
    index = scan_pawn_moves_diagonal_attacks::<COLOR, LEFT>(board, moves, index);
    index = scan_pawn_moves_diagonal_attacks::<COLOR, RIGHT>(board, moves, index);

    index
}

fn scan_pawn_moves_single_push<const COLOR: u8>(board: &Bitboard, moves: &mut [Move], mut index: usize) -> usize {
    let pieces = board.pieces[COLOR as usize][PAWN as usize];
    let occupancy = board.occupancy[WHITE as usize] | board.occupancy[BLACK as usize];

    let shift = 8 - 16 * (COLOR as i8);
    let promotion_line = 0xff00000000000000 >> (56 * (COLOR as u8));
    let mut target_fields = match COLOR {
        WHITE => (pieces << 8),
        BLACK => (pieces >> 8),
        _ => {
            panic!("Invalid value: COLOR={}", COLOR);
        }
    } & !occupancy;

    while target_fields != 0 {
        let to_field = get_lsb(target_fields);
        let to_field_index = bit_scan(to_field);
        let from_field_index = ((to_field_index as i8) - shift) as u8;
        target_fields = pop_lsb(target_fields);

        if (to_field & promotion_line) != 0 {
            moves[index + 0] = Move::new(from_field_index, to_field_index, MoveFlags::KNIGHT_PROMOTION);
            moves[index + 1] = Move::new(from_field_index, to_field_index, MoveFlags::BISHOP_PROMOTION);
            moves[index + 2] = Move::new(from_field_index, to_field_index, MoveFlags::ROOK_PROMOTION);
            moves[index + 3] = Move::new(from_field_index, to_field_index, MoveFlags::QUEEN_PROMOTION);
            index += 4;
        } else {
            moves[index] = Move::new(from_field_index, to_field_index, MoveFlags::QUIET);
            index += 1;
        }
    }

    index
}

fn scan_pawn_moves_double_push<const COLOR: u8>(board: &Bitboard, moves: &mut [Move], mut index: usize) -> usize {
    let pieces = board.pieces[COLOR as usize][PAWN as usize];
    let occupancy = board.occupancy[WHITE as usize] | board.occupancy[BLACK as usize];

    let shift = 16 - 32 * (COLOR as i8);
    let mut target_fields = match COLOR {
        WHITE => ((((pieces & RANK_B) << 8) & !occupancy) << 8),
        BLACK => ((((pieces & RANK_G) >> 8) & !occupancy) >> 8),
        _ => {
            panic!("Invalid value: COLOR={}", COLOR);
        }
    } & !occupancy;

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

fn scan_pawn_moves_diagonal_attacks<const COLOR: u8, const DIR: u8>(board: &Bitboard, moves: &mut [Move], mut index: usize) -> usize {
    let pieces = board.pieces[COLOR as usize][PAWN as usize];
    let enemy_color = COLOR ^ 1;

    let forbidden_file = FILE_A >> (DIR * 7);
    let shift = 9 - (COLOR ^ DIR) * 2;
    let signed_shift = (shift as i8) - ((COLOR as i8) * 2 * (shift as i8));
    let promotion_line = 0xff00000000000000 >> (56 * (COLOR as u8));

    let mut target_fields = match COLOR {
        WHITE => ((pieces & !forbidden_file) << shift),
        BLACK => ((pieces & !forbidden_file) >> shift),
        _ => {
            panic!("Invalid value: COLOR={}", COLOR);
        }
    } & (board.occupancy[enemy_color as usize] | board.en_passant);

    while target_fields != 0 {
        let to_field = get_lsb(target_fields);
        let to_field_index = bit_scan(to_field);
        let from_field_index = ((to_field_index as i8) - signed_shift) as u8;
        target_fields = pop_lsb(target_fields);

        if (to_field & promotion_line) != 0 {
            moves[index + 0] = Move::new(from_field_index, to_field_index, MoveFlags::KNIGHT_PROMOTION_CAPTURE);
            moves[index + 1] = Move::new(from_field_index, to_field_index, MoveFlags::BISHOP_PROMOTION_CAPTURE);
            moves[index + 2] = Move::new(from_field_index, to_field_index, MoveFlags::ROOK_PROMOTION_CAPTURE);
            moves[index + 3] = Move::new(from_field_index, to_field_index, MoveFlags::QUEEN_PROMOTION_CAPTURE);
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
