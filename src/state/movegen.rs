use super::*;
use crate::utils::assert_fast;
use crate::utils::bithelpers::BitHelpers;
use crate::utils::rand;
use std::sync::OnceLock;

#[rustfmt::skip]
const ROOK_SHIFTS: [u8; 64] =
[
    12, 11, 11, 11, 11, 11, 11, 12,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    12, 11, 11, 11, 11, 11, 11, 12
];

#[rustfmt::skip]
const BISHOP_SHIFTS: [u8; 64] =
[
    6, 5, 5, 5, 5, 5, 5, 6,
    5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5,
    6, 5, 5, 5, 5, 5, 5, 6
];

const ROOK_MAGIC_NUMBERS: [u64; 64] = [
    2413929538783315984,
    1170945936180973568,
    36063983556362632,
    72092815185739788,
    10520444128667370504,
    216173881759761408,
    144117387214979332,
    36035675409096960,
    9147939964463232,
    9241527447739047936,
    4900198006996009024,
    9271504309680349440,
    290904697693279232,
    18577391417034800,
    6557382345249260032,
    576601508053778688,
    289394209213907079,
    9259827994678018049,
    9227875773923722240,
    3458800797972758600,
    5260345653422196736,
    1153485004358042624,
    144401061158326274,
    108088590096941196,
    9835864061152075800,
    1738671485194616848,
    54184079046820352,
    4611703612763537536,
    4917935201873035392,
    72061994232053888,
    72642538535649792,
    144186664922138796,
    9223408595893223562,
    36662253159714817,
    9148830171799556,
    13907258654559569920,
    4644904059815936,
    1153491053785908224,
    37525196376080,
    576742263804133474,
    4611721477685805056,
    5046847994054443074,
    81347780144070704,
    4652218965232287760,
    613641840008167440,
    1208090608632365184,
    1444811064045797444,
    144115480410849301,
    36031270989333568,
    9024809304326464,
    6918690112188612736,
    11681520772186240,
    4612249003011083776,
    1153205180755083392,
    4657900772905853952,
    578724114186912256,
    16648963476391231489,
    504544995536095746,
    4036492191583969290,
    9264080424097810437,
    18577417855566882,
    562984515535106,
    3206563493302044676,
    2323860433544422402,
];

const BISHOP_MAGIC_NUMBERS: [u64; 64] = [
    18018831217729569,
    7566619154406457857,
    5769265629886676992,
    11836590258269718532,
    1130711343955976,
    5188711992789051600,
    2595223508957332480,
    4613977417873621504,
    52914355962521,
    720584771016851496,
    81672282392551681,
    1443689575104709154,
    2315497890614674434,
    4504720752246850,
    4683757923315231234,
    36030472065647684,
    869195003027850256,
    2533343613162048,
    9077585216569856,
    4613938370177974404,
    1819740157378038914,
    3026981903850120192,
    150100551208960,
    9250534398112121088,
    13983151348368280608,
    2453353491453707520,
    1189838711584489508,
    5188437052588310784,
    73192707253608448,
    1171008470900605200,
    2304869587485219,
    72216611327312384,
    1416312765554720,
    4902326593860936194,
    7494553009214460168,
    4535619944704,
    289464028735078528,
    9572356859152384,
    24771999121551488,
    4611968773312806976,
    594765529323978768,
    290309994972256,
    73747544500340740,
    4504287224791296,
    18109643855331840,
    9242583324889711104,
    302884692999082816,
    2315421958013010688,
    2306969493907701858,
    1153521859699409280,
    2254007511875588,
    12687767152414427136,
    9008713537683586,
    164399116042125312,
    1315103887499739200,
    24772082878251017,
    1188989888384471040,
    144124276262832640,
    9801521853783640080,
    1153489127519306752,
    5783255261986030084,
    9259968324145381506,
    9513590469632983681,
    9377059641948700736,
];

static ROOK_SQUARES: OnceLock<[MagicSquare; 64]> = OnceLock::new();
static BISHOP_SQUARES: OnceLock<[MagicSquare; 64]> = OnceLock::new();

#[derive(Clone, Default)]
pub struct MagicSquare {
    pub mask: u64,
    pub shift: u8,
    pub magic: u64,
    pub attacks: Vec<u64>,
}

/// Initialize movegen for rooks and bishops.
pub fn init() {
    const INIT: MagicSquare = MagicSquare::new();
    let mut rook_squares = [INIT; 64];
    let mut bishop_squares = [INIT; 64];

    for index in ALL_SQUARES {
        apply_rook_magic(&mut rook_squares, index);
        apply_bishop_magic(&mut bishop_squares, index);
    }

    let _ = ROOK_SQUARES.set(rook_squares);
    let _ = BISHOP_SQUARES.set(bishop_squares);
}

/// Gets a rook moves for the square specified by `square`, considering `occupancy_bb`.
pub fn get_rook_moves(mut occupancy_bb: u64, square: usize) -> u64 {
    assert_fast!(square < 64);

    let data = unsafe { ROOK_SQUARES.get().unwrap_unchecked() };
    occupancy_bb &= data[square].mask;
    occupancy_bb = occupancy_bb.wrapping_mul(data[square].magic);
    occupancy_bb >>= data[square].shift;

    assert_fast!(occupancy_bb < data[square].attacks.len() as u64);
    data[square].attacks[occupancy_bb as usize]
}

/// Gets a bishop moves for the square specified by `square`, considering `occupancy_bb`.
pub fn get_bishop_moves(mut occupancy_bb: u64, square: usize) -> u64 {
    assert_fast!(square < 64);

    let data = unsafe { BISHOP_SQUARES.get().unwrap_unchecked() };
    occupancy_bb &= data[square].mask;
    occupancy_bb = occupancy_bb.wrapping_mul(data[square].magic);
    occupancy_bb >>= data[square].shift;

    assert_fast!(occupancy_bb < data[square].attacks.len() as u64);
    data[square].attacks[occupancy_bb as usize]
}

/// Gets a queen moves for the square specified by `square`, considering `occupancy_bb`.
pub fn get_queen_moves(occupancy_bb: u64, square: usize) -> u64 {
    movegen::get_rook_moves(occupancy_bb, square) | movegen::get_bishop_moves(occupancy_bb, square)
}

/// Gets a knight moves for the square specified by `square`, without considering an occupancy.
pub fn get_knight_moves(square: usize) -> u64 {
    patterns::get_jumps(square)
}

/// Gets a king moves for the square specified by `square`, without considering an occupancy.
pub fn get_king_moves(square: usize) -> u64 {
    patterns::get_box(square)
}

/// Generates a rook magic number for the square specified by `square`.
pub fn generate_rook_magic_number(square: usize) -> u64 {
    assert_fast!(square < 64);

    let shift = ROOK_SHIFTS[square];
    let mask = get_rook_mask(square);
    let count = 1 << shift;

    let mut permutations = Vec::with_capacity(count as usize);
    let mut attacks = Vec::with_capacity(count as usize);

    for index in 0..count {
        let permutation = get_permutation(mask, index as u64);
        let permutation_attacks = get_rook_attacks(permutation, square);

        permutations.push(permutation);
        attacks.push(permutation_attacks);
    }

    generate_magic_number(shift, &permutations, &attacks)
}

/// Generates a bishop magic number for the square specified by `square`.
pub fn generate_bishop_magic_number(square: usize) -> u64 {
    assert_fast!(square < 64);

    let shift = BISHOP_SHIFTS[square];
    let mask = get_bishop_mask(square);
    let count = 1 << shift;

    let mut permutations = Vec::with_capacity(count as usize);
    let mut attacks = Vec::with_capacity(count as usize);

    for index in 0..count {
        let permutation = get_permutation(mask, index as u64);
        let permutation_attacks = get_bishop_attacks(permutation, square);

        permutations.push(permutation);
        attacks.push(permutation_attacks);
    }

    generate_magic_number(shift, &permutations, &attacks)
}

/// Generates a magic number for a set of `permutations` and `attacks`, using `shift` proper for the specified square.
fn generate_magic_number(shift: u8, permutations: &[u64], attacks: &[u64]) -> u64 {
    let count = 1 << shift;
    let mut hashed_attacks = vec![0; count];
    let mut magic_number: u64;

    loop {
        magic_number = rand::u64(..) & rand::u64(..) & rand::u64(..);

        for index in 0..count {
            let hash = (permutations[index].wrapping_mul(magic_number) >> (64 - shift)) as usize;

            if hashed_attacks[hash] == 0 || hashed_attacks[hash] == attacks[index] {
                hashed_attacks[hash] = attacks[index];
            } else {
                magic_number = 0;
                break;
            }
        }

        if magic_number != 0 {
            break;
        }

        for index in &mut hashed_attacks {
            *index = 0;
        }
    }

    magic_number
}

/// Applies rook magic for the square specified by `square`, using built-in magic number from [ROOK_MAGIC_NUMBERS].
fn apply_rook_magic(rook_squares: &mut [MagicSquare; 64], square: usize) {
    assert_fast!(square < 64);

    let shift = ROOK_SHIFTS[square];
    let mask = get_rook_mask(square);
    let count = 1 << shift;

    let mut permutations = Vec::with_capacity(count as usize);
    let mut attacks = Vec::with_capacity(count as usize);

    for index in 0..count {
        let permutation = get_permutation(mask, index as u64);

        permutations.push(permutation);
        attacks.push(get_rook_attacks(permutation, square));
    }

    let magic = ROOK_MAGIC_NUMBERS[square];
    rook_squares[square].shift = 64 - shift;
    rook_squares[square].mask = mask;
    rook_squares[square].magic = magic;
    rook_squares[square].attacks = apply_magic_for_square(&permutations, &attacks, magic, shift);
}

/// Applies bishop magic for the square specified by `square`, using built-in magic number from [BISHOP_MAGIC_NUMBERS].
fn apply_bishop_magic(bishop_squares: &mut [MagicSquare; 64], square: usize) {
    assert_fast!(square < 64);

    let shift = BISHOP_SHIFTS[square];
    let mask = get_bishop_mask(square);
    let count = 1 << shift;

    let mut permutations = Vec::with_capacity(count as usize);
    let mut attacks = Vec::with_capacity(count as usize);

    for index in 0..count {
        let permutation = get_permutation(mask, index as u64);

        permutations.push(permutation);
        attacks.push(get_bishop_attacks(permutation, square));
    }

    let magic = BISHOP_MAGIC_NUMBERS[square];
    bishop_squares[square].shift = 64 - shift;
    bishop_squares[square].mask = mask;
    bishop_squares[square].magic = magic;
    bishop_squares[square].attacks = apply_magic_for_square(&permutations, &attacks, magic, shift);
}

/// Applies a magic number for a set of `permutations`, `attacks` and `square`.
fn apply_magic_for_square(permutations: &[u64], attacks: &[u64], magic: u64, shift: u8) -> Vec<u64> {
    let count = 1 << shift;
    let mut result = vec![0; count as usize];

    for index in 0..count {
        let permutation = permutations[index as usize];
        let permutation_attacks = attacks[index as usize];
        let hash = permutation.wrapping_mul(magic) >> (64 - shift);

        assert_fast!(result[hash as usize] == 0);
        result[hash as usize] = permutation_attacks;
    }

    result
}

/// Gets `index`-th permutation of the `mask`.  
fn get_permutation(mut mask: u64, mut index: u64) -> u64 {
    let mut result = 0u64;

    while mask != 0 {
        let lsb = mask.get_lsb();
        let lsb_index = lsb.bit_scan();
        mask = mask.pop_lsb();

        result |= (index & 1) << lsb_index;
        index >>= 1;
    }

    result
}

/// Gets a rook mask for the square specified by `square`, without considering occupancy.
fn get_rook_mask(square: usize) -> u64 {
    (patterns::get_file(square) & !(1u64 << square) & !RANK_1_BB & !RANK_8_BB) | (patterns::get_rank(square) & !(1u64 << square) & !FILE_A_BB & !FILE_H_BB)
}

/// Gets a bishop mask for the square specified by `square`, without considering occupancy.
fn get_bishop_mask(square: usize) -> u64 {
    patterns::get_diagonals(square) & !(1u64 << square) & !EDGE_BB
}

/// Gets a rook attacks for the square specified by `square`, considering `occupancy_bb`.
fn get_rook_attacks(occupancy_bb: u64, square: usize) -> u64 {
    assert_fast!(square < 64);

    let left = get_attacks(occupancy_bb, square, (-1, 0));
    let right = get_attacks(occupancy_bb, square, (1, 0));
    let top = get_attacks(occupancy_bb, square, (0, 1));
    let down = get_attacks(occupancy_bb, square, (0, -1));

    left | right | top | down
}

/// Gets a bishop attacks for the square specified by `square`, occupancy `occupancy_bb`.
fn get_bishop_attacks(occupancy_bb: u64, square: usize) -> u64 {
    assert_fast!(square < 64);

    let top_right = get_attacks(occupancy_bb, square, (1, 1));
    let top_left = get_attacks(occupancy_bb, square, (1, -1));
    let down_right = get_attacks(occupancy_bb, square, (-1, 1));
    let down_left = get_attacks(occupancy_bb, square, (-1, -1));

    top_right | top_left | down_right | down_left
}

/// Helper function to get all possible to move squares, considering `occupancy_bb`, starting from the square
/// specified by `square` and going into the `direction`.
fn get_attacks(occupancy_bb: u64, square: usize, direction: (isize, isize)) -> u64 {
    assert_fast!(square < 64);
    assert_fast!(direction.0 >= -1 && direction.0 <= 1);
    assert_fast!(direction.1 >= -1 && direction.1 <= 1);

    let mut result = 0u64;
    let mut current = ((square as isize) % 8 + direction.0, (square as isize) / 8 + direction.1);

    while current.0 >= 0 && current.0 <= 7 && current.1 >= 0 && current.1 <= 7 {
        result |= 1u64 << (current.0 + current.1 * 8);

        if (occupancy_bb & result) != 0 {
            break;
        }

        current = (current.0 + direction.0, current.1 + direction.1);
    }

    result
}

impl MagicSquare {
    /// Constructs a new instance of [MagicSquare] with zeroed values.
    pub const fn new() -> Self {
        Self { mask: 0, shift: 0, magic: 0, attacks: Vec::new() }
    }
}
