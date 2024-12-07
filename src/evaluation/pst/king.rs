// ------------------------------------------------------------------------- //
// Generated at 06-12-2024 18:23:06 UTC (e = 0.130063, k = 0.0077, r = 1.00) //
// ------------------------------------------------------------------------- //

use super::*;

#[rustfmt::skip]
pub const KING_PST_PATTERN: [[[PackedEval; 64]; KING_BUCKETS_COUNT]; 2] =
[
    [
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!( -57,   43),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   5,   33),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(  45,   18),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(  56,  -17),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!( -12,   32), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(  -6,   33), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(  14,   20), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(  23,   -8), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!( -62,   38), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!( -56,   34), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!( -41,   20), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!( -64,   -6), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!( -84,   34), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!( -74,   25), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!( -76,   16), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!( -43,  -35), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!( -87,   32), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!( -64,   21), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!( -60,   13), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!( -73,  -18), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!( -16,   27), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!( -28,   24), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!( -17,   17), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   2,  -19), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   9,   19), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(  25,   10), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(  40,    4), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(  63,  -31), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( -37,    9), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   5,   -1), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(  67,  -23), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(  54,  -58), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!( -12,  -56),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(  -9,   -5),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(  -9,   10),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!( -46,    3),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(  32,  -24), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(  25,   10), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(  16,   26), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!( -21,   17), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(  34,  -19), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(  42,    6), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(  -3,   28), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!( -48,   23), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   8,  -21), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(  58,  -12), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!( -16,   17), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!( -88,   26), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(  34,  -38), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(  82,  -14), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(  -3,   14), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!( -58,   18), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(  80,  -42), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(  47,   -8), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(  25,   13), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!( -18,    5), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(  31,  -47), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(  60,  -19), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(  39,  -10), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!( -17,  -10), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!( -43, -101), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(  70,  -54), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(  23,  -32), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( -27,  -37), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
    ],
    [
        [
            s!(   2,    5), s!(   1,   -0), s!(  -6,  -25), s!(   1,   -2), s!(   2,   -1), s!(  -0,   -6), s!(  -3,   -9), s!(   1,    9),
            s!(  -1,  -11), s!(  -1,   -6), s!(  -2,   -6), s!(  -3,   -6), s!(  -1,   -0), s!(   3,   -1), s!(  -4,  -18), s!(  -3,   -9),
            s!(   3,   -0), s!(   0,    1), s!(  -1,   -4), s!(  -3,    7), s!(   3,    5), s!(   4,   -1), s!(   3,  -12), s!(   4,   -4),
            s!(   3,   12), s!(   0,    4), s!(  -2,    3), s!(  -4,    5), s!(  -7,   11), s!(  -3,    1), s!(  11,  -33), s!( -16,  -28),
            s!(   1,   13), s!(   1,    8), s!(  -8,   13), s!( -22,   14), s!(  -4,    8), s!(  -2,   11), s!( -18,   10), s!( -16,    2),
            s!(   6,    8), s!(   4,    7), s!(  -7,   10), s!(  -3,    9), s!(  12,    7), s!(  -3,    6), s!( -13,    7), s!(   5,    8),
            s!(  -8,   -4), s!(  -1,    1), s!(  -3,   -3), s!(  -0,  -10), s!(  -4,    3), s!(  -2,    2), s!(  11,    4), s!(   5,    6),
            s!(   0,  -24), s!(  -3,  -13), s!(  10,  -16), s!( -10,  -15), s!(  -5,  -13), s!(  -3,    5), s!(  11,    0), s!(  11,   -8),
        ],
        [
            s!(   1,   -0), s!(   4,  -10), s!(   4,    3), s!(  -2,   -6), s!(   2,  -11), s!(  -2,  -24), s!(   5,   13), s!(  -1,   -7),
            s!(   3,   -2), s!(   8,   -6), s!(   2,   -6), s!(  -2,   -5), s!(  -5,   -7), s!(  -3,  -25), s!(   0,   -5), s!(  -1,  -10),
            s!(   4,    3), s!(  -5,   -3), s!(  -2,   -3), s!(  -9,   10), s!( -16,   -1), s!(  17,    5), s!(  13,    5), s!(   3,  -17),
            s!(  -6,    6), s!(  -4,    3), s!( -19,    9), s!(  -3,    9), s!( -13,   -6), s!(  -6,  -15), s!(  -8,  -31), s!(  -5,  -32),
            s!(  -4,   13), s!( -16,   21), s!(  -4,   11), s!(  -4,   14), s!( -11,   13), s!(  -9,   12), s!( -30,   12), s!( -27,    9),
            s!(   4,    8), s!( -16,    9), s!( -21,    7), s!( -24,   14), s!(  -7,   14), s!( -10,    4), s!(  -3,    1), s!(   3,   13),
            s!(  -2,  -10), s!(   5,   -4), s!(  15,  -13), s!(  11,  -10), s!( -11,   10), s!(   9,   -2), s!(  16,    2), s!(  22,    5),
            s!(  12,  -21), s!(  19,  -25), s!(  16,  -15), s!(   0,   -9), s!(  22,  -20), s!(   5,    4), s!(  19,    2), s!(  22,   12),
        ],
        [
            s!(   3,    7), s!(   3,    6), s!(   1,    5), s!(   1,   -5), s!(   1,   -3), s!(   5,    5), s!(  -0,    4), s!(  -7,  -21),
            s!(   5,   -2), s!(  -3,   -3), s!(  -4,    2), s!(  -3,    3), s!(   1,   -8), s!(   7,    4), s!(   2,   -0), s!(  -7,   -4),
            s!(   2,    0), s!(  -9,   -4), s!( -14,    4), s!(  -9,    8), s!(   4,    6), s!(   5,   10), s!(   9,  -18), s!(  -9,   -9),
            s!(   5,   15), s!( -12,    6), s!(  -8,    2), s!( -14,    4), s!(   4,  -18), s!(   7,  -24), s!(  14,  -22), s!(   9,   -9),
            s!(  10,   12), s!(   7,    6), s!(   6,    1), s!(  14,    8), s!(   4,    1), s!(  11,   -5), s!(  15,    8), s!(  11,    7),
            s!(  -3,    4), s!( -16,   -2), s!(   8,   -2), s!(   9,    0), s!(   6,   -4), s!(   0,   -6), s!(   5,   -3), s!(   8,    1),
            s!(  -3,  -12), s!(   2,  -16), s!(   3,  -16), s!(  -7,   -3), s!(  -8,   -5), s!( -11,   -7), s!(  -4,    3), s!(   1,    1),
            s!(  -6,   -1), s!(  -4,   -6), s!( -10,    3), s!( -12,   -2), s!(  -7,   -0), s!(  -4,    4), s!( -14,   13), s!(  -1,   22),
        ],
        [
            s!(   1,    2), s!(   3,   13), s!(   3,    5), s!(   1,    3), s!(   2,    1), s!(   4,   10), s!(   5,   17), s!(  -3,  -13),
            s!(   2,    5), s!(  -0,    0), s!(   0,    7), s!(   2,    8), s!(   4,    3), s!(   7,   20), s!(  -1,    1), s!(  -1,   16),
            s!(  -2,    6), s!(  -9,    1), s!(  -3,   -5), s!(   2,  -11), s!(   5,   -3), s!(   2,   -5), s!(  -9,   -3), s!(  -7,    2),
            s!(   1,    2), s!(  -6,   -6), s!(  -9,   -6), s!(   3,  -20), s!(  18,  -22), s!(  -7,  -22), s!(   6,   -7), s!(  10,   -3),
            s!(   1,   11), s!(   2,    9), s!(   3,    0), s!(  -7,    6), s!(  -7,    7), s!(   4,    2), s!(  16,    5), s!(  11,    0),
            s!(   9,   -9), s!(   6,   -6), s!( -15,   -1), s!( -21,    2), s!( -18,    4), s!(  -9,   -6), s!(  -8,    1), s!(   6,   -7),
            s!(   3,  -13), s!(  -6,  -12), s!(   2,  -17), s!(  -4,   -7), s!(  -4,   -6), s!(  -9,   -6), s!(  -2,   -2), s!(  -9,   -6),
            s!(   0,   -5), s!(  -0,  -16), s!(   9,   -9), s!(  10,   -4), s!(  24,  -10), s!(  18,  -10), s!(  11,   -4), s!(  -1,   15),
        ],
        [
            s!(  -1,    0), s!(   3,    5), s!(   2,   -1), s!(   3,    7), s!(  -1,   -6), s!(   3,   17), s!(   0,  -10), s!(   1,    4),
            s!(   1,    7), s!(   3,    3), s!(   2,   -9), s!(   3,   15), s!(   2,   13), s!(  -1,   -2), s!(   2,    9), s!(  -2,    3),
            s!(  -4,    1), s!(  -1,   -7), s!(   2,    7), s!(   6,    9), s!(   4,    5), s!(  -9,    6), s!(  -9,   -2), s!(  -0,    7),
            s!(  -3,  -10), s!(  -9,  -11), s!(   1,  -24), s!(   1,  -13), s!(  -1,  -16), s!( -12,   -4), s!(  -2,   -6), s!(   5,    1),
            s!(   4,   -7), s!(   9,   -6), s!(  -7,   -3), s!(  -6,   -1), s!(   8,    4), s!(  -1,    2), s!(  11,    3), s!(   8,   -4),
            s!(   3,   -2), s!(   0,   -1), s!(  10,   -7), s!(  12,  -10), s!(  -1,   -3), s!(  -7,   -3), s!(  14,   -5), s!(   2,   -8),
            s!(  18,   -5), s!(   2,   -7), s!(  -1,  -15), s!( -20,   -6), s!(   8,  -11), s!(   5,  -12), s!(   5,   -6), s!(  11,   -7),
            s!(  -1,    2), s!(  -7,    5), s!( -10,    8), s!(  11,   -8), s!(   6,   -2), s!(  14,   -2), s!(  -0,   -4), s!(   9,   -6),
        ],
        [
            s!(  -3,  -26), s!(  -0,   -7), s!(   0,    0), s!(  -2,   -5), s!(   1,   -8), s!(  -2,   -5), s!(   0,   -7), s!(   4,   15),
            s!(   0,   -1), s!(   2,  -14), s!(   1,   -4), s!(  -0,   -8), s!(  -1,    6), s!(   1,    2), s!(   9,    6), s!(   4,   -1),
            s!(  -3,  -23), s!(   2,  -13), s!(   1,   -0), s!(   2,   -2), s!(  -4,    3), s!( -13,    4), s!(  -3,    9), s!(   1,    6),
            s!( -11,  -22), s!(  -1,  -30), s!(   4,  -19), s!(   2,  -21), s!(  -7,    2), s!(  -1,    2), s!(  -7,    7), s!(   4,   15),
            s!(  -1,   -2), s!(  -6,   -4), s!(  -1,    3), s!(  -1,    7), s!(   6,    2), s!(  -1,   -2), s!(  13,   -2), s!(   3,    2),
            s!(   8,   -4), s!(   6,    1), s!(  -4,   -1), s!(  -9,    3), s!( -12,    3), s!(  -9,   -1), s!(  -8,   -2), s!(  -8,    2),
            s!(   9,    2), s!(   3,   -0), s!(   6,   -0), s!(   3,   -2), s!(   3,   -4), s!(  -5,   -7), s!( -20,    1), s!(  10,   -7),
            s!(  -3,   11), s!(   1,    6), s!(  -2,    3), s!(  13,    2), s!(   6,  -14), s!(  10,   -6), s!(   6,  -10), s!(   2,   -1),
        ],
        [
            s!(  -0,   -5), s!(  -0,   -9), s!(  -0,   -8), s!(   0,   -7), s!(   3,   -3), s!(  -2,  -22), s!(   1,  -15), s!(  -0,   -3),
            s!(  -0,   -1), s!(   0,   -3), s!(   0,  -12), s!(  -0,  -12), s!(  -1,   -7), s!(   8,   -2), s!(  10,  -14), s!(   2,  -13),
            s!(  -1,  -15), s!(   2,   -6), s!(   0,  -11), s!( -10,    0), s!(  -7,    1), s!(  -4,   -4), s!(  -3,    1), s!(   5,    1),
            s!(  -7,  -35), s!(  -3,  -31), s!(   3,  -19), s!(  -6,    1), s!(  -4,    6), s!(  -1,   -2), s!(  -2,   13), s!(   1,    3),
            s!(  -7,   -8), s!( -10,   -8), s!(   2,    9), s!(   6,    7), s!(  -1,    5), s!(  -7,    6), s!(   9,   -3), s!(   7,    9),
            s!(  -4,    5), s!(   5,    4), s!(   4,    7), s!(   8,    8), s!(   8,   -1), s!(  -0,    2), s!(  -4,   -1), s!(   1,    1),
            s!(   4,    3), s!(   4,    8), s!(  -8,   10), s!( -11,    8), s!(   5,   -6), s!(  -2,   -8), s!( -17,    4), s!(  -3,  -13),
            s!(   2,   21), s!(   2,   11), s!(  -3,    1), s!(  -4,   -1), s!( -13,   -0), s!(  12,  -13), s!(   9,  -22), s!( -10,  -16),
        ],
        [
            s!(   0,    0), s!(   0,   -2), s!(  -0,   -1), s!(   0,   -2), s!(   2,    2), s!(  -2,   -8), s!(  -2,   -4), s!(  -2,   -3),
            s!(   0,   -1), s!(  -0,   -2), s!(  -0,   -2), s!(   3,    4), s!(   2,   -1), s!(  -1,  -18), s!(  -2,  -15), s!(  -0,    1),
            s!(   1,   -2), s!(  -1,  -14), s!(   1,    1), s!(  -3,    1), s!(   0,    2), s!( -10,   -4), s!(   2,   -6), s!(   4,   -2),
            s!(  -4,  -32), s!(   0,  -16), s!(  -1,    6), s!(  -3,    4), s!(   3,    6), s!(   7,   -8), s!(   0,    5), s!(   3,    7),
            s!(  -2,  -14), s!(   8,  -11), s!(  -8,    6), s!(  -3,    6), s!(  -2,   -1), s!(   5,   13), s!(   5,    8), s!(  16,   24),
            s!( -11,   11), s!(  15,   -6), s!(   8,   -2), s!(   4,   -3), s!(   4,   -1), s!(  -1,   -0), s!(  -1,    9), s!(   2,    3),
            s!(  -2,   18), s!( -21,   17), s!( -11,   17), s!(  -6,    9), s!(   5,   -6), s!( -19,    1), s!(  -7,   -8), s!(  -7,   -9),
            s!(  17,   -1), s!(   4,   -8), s!(   5,   -6), s!( -15,    2), s!( -27,   -4), s!(   9,  -25), s!( -10,   -9), s!( -14,  -28),
        ],
        [
            s!(  -1,   -5), s!(  -2,   -3), s!(  -2,  -10), s!(   2,    0), s!(   1,    2), s!(   1,    1), s!(  -2,   -2), s!(   0,    2),
            s!(  -2,   -6), s!(  -5,   -5), s!(  -3,   -1), s!(   0,    5), s!(  -4,  -18), s!(  -2,  -20), s!(  -3,  -17), s!(  -2,   -7),
            s!(   2,    5), s!(  -3,    3), s!(  -5,    5), s!(  -2,   -1), s!(  -2,  -11), s!(  -4,  -11), s!(  -4,   -2), s!(   1,   -3),
            s!(  -0,   10), s!(  -2,   19), s!(  -0,   -1), s!(  -1,    5), s!(  -2,   11), s!(  -5,    7), s!(   3,    6), s!(  -4,   -3),
            s!(  -0,   14), s!(  -4,   13), s!(  -3,   -3), s!(  -6,    2), s!(  -2,   -3), s!(  -7,   -6), s!(  -8,    4), s!(  -3,    1),
            s!(   4,    4), s!(   1,    9), s!(   2,    2), s!(   3,    4), s!(   8,   -1), s!(  18,   14), s!(  -4,   17), s!(   3,    4),
            s!(  -7,   -2), s!(   1,    6), s!(  -3,    2), s!(   0,    7), s!(  -2,   12), s!(   3,   22), s!(   9,   22), s!(  16,   14),
            s!(   2,    0), s!(   5,   -2), s!(  -2,   -7), s!(  -4,    2), s!(  -5,   15), s!(  -9,   20), s!(  -5,   33), s!(  -2,   14),
        ],
        [
            s!(  -1,   -4), s!(   4,    6), s!(   3,   -7), s!(  -1,   -1), s!(   2,   -7), s!(  -1,  -13), s!(   3,   -4), s!(  -2,   -9),
            s!(   2,    1), s!(   3,   -6), s!(   2,    6), s!(   3,   -3), s!(  -2,  -23), s!(   1,  -18), s!(  -1,   -8), s!(  -1,  -14),
            s!(  -2,    1), s!(   5,   13), s!(   3,    1), s!(  -6,   -8), s!(  -9,   -8), s!(   3,   -2), s!(  -1,   -7), s!(   1,  -14),
            s!(   3,   16), s!(   3,    7), s!(   0,   12), s!(   4,    5), s!(  -6,    6), s!(  -1,    9), s!(  -3,    2), s!(   0,   -5),
            s!(   2,   13), s!(  -8,   10), s!(  -3,   -6), s!(   3,    2), s!(   1,   -7), s!(  -2,   -2), s!( -12,   -6), s!(  -5,    1),
            s!(  -4,    3), s!( -12,   15), s!( -11,   10), s!(  -7,    7), s!(  -5,    6), s!(  -4,    8), s!(   0,   -4), s!(  -1,   19),
            s!( -10,    4), s!(   5,    5), s!(   3,    5), s!(   6,   -1), s!(  -3,   20), s!(  -2,   35), s!(  15,    8), s!(   5,   18),
            s!(  11,   -5), s!(   8,   -5), s!(   7,  -17), s!(  -5,    8), s!(   6,   20), s!(  -9,   24), s!( -13,   17), s!(   5,   26),
        ],
        [
            s!(   3,   18), s!(   3,   12), s!(   1,    8), s!(   3,    4), s!(   1,   -7), s!(   4,    6), s!(   0,   -8), s!(  -1,   -3),
            s!(   2,   -1), s!(  -0,    6), s!(  -0,    6), s!(   2,   -8), s!(  -0,   -7), s!(  10,   -2), s!(   2,   -6), s!(   0,  -12),
            s!(  -0,   -1), s!(  -5,   10), s!(   2,    5), s!(  -6,   -2), s!(   2,    2), s!(  -9,    2), s!(  -2,  -10), s!(  -8,   -8),
            s!(   2,   16), s!(   3,   17), s!(   6,    7), s!(   2,    5), s!(  -3,    5), s!(   0,    8), s!(  -1,   11), s!(   8,   -7),
            s!(   3,   13), s!(   3,    8), s!(   9,   -7), s!(  13,  -11), s!(   0,    5), s!(  -2,   -9), s!(   2,   -1), s!(  11,  -15),
            s!(   0,    4), s!( -12,    4), s!(  11,   -2), s!(  10,   -1), s!(   0,   -8), s!(  -3,    2), s!(   4,   13), s!(   3,   -2),
            s!(  -5,   -3), s!(  11,    1), s!(   1,   -1), s!(   6,   -1), s!(   8,    8), s!(  -0,   -4), s!(  -1,    2), s!(   0,   11),
            s!(  -0,    5), s!(  -1,    7), s!(  -3,    1), s!(  -1,    8), s!(  -9,   21), s!( -19,   15), s!( -18,   17), s!(  -8,    7),
        ],
        [
            s!(   0,    5), s!(   1,    1), s!(   1,    3), s!(  -1,   -7), s!(  -0,    2), s!(   3,    8), s!(   4,   14), s!(   0,    5),
            s!(   1,   -0), s!(   2,    9), s!(   2,   12), s!(   1,    4), s!(  11,   23), s!(  10,   26), s!(   3,   13), s!(   3,    7),
            s!(  -0,    9), s!(  -1,    8), s!(   0,    8), s!(   2,   -3), s!(  -3,  -11), s!(  -3,   -7), s!(  -4,   -2), s!(  -3,    4),
            s!(   1,   13), s!(  -2,   12), s!(   0,    5), s!(   1,   -3), s!(  -3,   -7), s!(  -5,    9), s!(  -1,   -7), s!(   3,   -7),
            s!(  -3,   -4), s!(   0,   -8), s!(   4,   -5), s!(  -5,   -8), s!(  -5,    4), s!(  -1,   -4), s!(  15,   -8), s!(  11,  -11),
            s!(   4,    3), s!(  11,    7), s!(   4,   -0), s!( -11,    1), s!( -11,   -4), s!(  -6,   -2), s!(  10,  -11), s!(  10,   -8),
            s!(  -6,   -8), s!(   8,   -4), s!(   8,   -0), s!(  -3,   11), s!(  -9,    4), s!(   0,   -2), s!(  11,   -2), s!( -12,   -6),
            s!(  -1,  -12), s!(  -6,   -6), s!(  -4,    4), s!(   2,   15), s!(  -5,   15), s!(  -6,    3), s!(  -7,    4), s!(  -5,  -10),
        ],
        [
            s!(   0,    3), s!(   1,    0), s!(   2,    5), s!(   2,    3), s!(   0,   -3), s!(   2,    7), s!(  -0,  -11), s!(   2,    8),
            s!(   2,    4), s!(   5,   12), s!(   1,   -7), s!(   0,    2), s!(  -0,    8), s!(  -2,    1), s!(   1,    4), s!(   0,    6),
            s!(  -4,    3), s!(  -1,    3), s!(   2,    6), s!(   2,    4), s!(   2,    5), s!( -10,   -1), s!(  -5,   10), s!(   1,    3),
            s!(  -0,   -1), s!(  -2,   -1), s!(  -1,    2), s!(   0,   13), s!(  -2,   10), s!(  -3,    8), s!(  -6,    2), s!(  -0,    4),
            s!(   1,   -6), s!(   2,   -8), s!(  -2,    5), s!(  -7,  -16), s!(  -0,   -4), s!(   1,  -16), s!(   8,   -9), s!(   2,  -16),
            s!(   2,   -3), s!(   6,   -5), s!(   3,   -2), s!(   3,  -15), s!(  -1,   -6), s!(   5,   -6), s!(  17,   -7), s!(   6,  -14),
            s!(  10,   -1), s!(   6,    6), s!(  -2,   11), s!(  -4,   13), s!(   5,    4), s!(  22,   -5), s!(   4,   -2), s!(  -1,    0),
            s!(   2,    6), s!(   1,   20), s!(  -5,   23), s!(  -5,    1), s!(  -2,    7), s!( -12,    4), s!(   3,    3), s!(   5,   -5),
        ],
        [
            s!(   0,   -5), s!(  -1,   -5), s!(   0,   -1), s!(  -1,   -6), s!(   1,   -2), s!(  -3,   -8), s!(   0,   -4), s!(   4,   15),
            s!(  -0,   -0), s!(   1,    1), s!(   1,   -9), s!(  -2,   -5), s!(  -2,    6), s!(  -3,   -0), s!(   2,   -2), s!(   3,  -10),
            s!(  -0,  -11), s!(  -2,  -14), s!(  -1,   -5), s!(   2,    8), s!(  -1,    0), s!( -11,    0), s!(  -1,   10), s!(  -0,   -4),
            s!(  -5,   -7), s!(  -0,    5), s!(  -1,    7), s!(  -1,    4), s!(  -2,    2), s!(   2,   11), s!(  -7,   12), s!(   3,    8),
            s!(  -1,  -15), s!(  -4,    4), s!(  -2,    2), s!(  -3,    2), s!(  -4,  -10), s!(   3,   -8), s!(   8,  -12), s!(   5,    1),
            s!(   3,  -10), s!(  -1,    8), s!(  -1,   -1), s!(  -3,    4), s!(   1,   -0), s!(   5,   -0), s!(   8,    9), s!(   4,   12),
            s!(  13,   14), s!(  -7,   10), s!(   1,    6), s!(   6,    6), s!(   2,    1), s!(   8,   -0), s!(  -8,    1), s!( -10,   -3),
            s!(  -7,    8), s!(   6,   28), s!(  -5,   14), s!(  -3,   18), s!(   8,    6), s!(   7,   -3), s!(   9,   -5), s!(   6,   -4),
        ],
        [
            s!(  -0,    1), s!(  -0,   -4), s!(   1,    6), s!(  -0,   -4), s!(   3,    3), s!(  -2,  -11), s!(  -1,   -9), s!(  -0,    1),
            s!(   0,   -2), s!(  -1,   -6), s!(   1,   -8), s!(   1,   -8), s!(  -1,  -14), s!(   2,    0), s!(   4,    1), s!(   2,   -6),
            s!(   1,   -3), s!(  -1,   -3), s!(  -1,  -10), s!(  -5,  -13), s!(  -2,    4), s!(  -3,   -0), s!(  -4,    2), s!(   7,   -4),
            s!(  -1,   -6), s!(   1,    1), s!(   0,   -1), s!(  -3,   -1), s!(   1,    6), s!(   7,   -1), s!(  -7,   10), s!(   5,    9),
            s!(  -2,   -2), s!(  -3,    4), s!(  -5,    4), s!(  -3,  -10), s!(   0,   -4), s!(   6,    1), s!(   3,    2), s!(   4,    7),
            s!(  -1,    3), s!(   7,   -0), s!(  -1,    6), s!(   7,    3), s!(  12,    3), s!(  13,    5), s!(   0,   12), s!(  -3,   -9),
            s!(  13,   17), s!(  -0,   18), s!(   8,   29), s!(  -2,   16), s!(   6,    1), s!(   9,   -1), s!(  -7,    8), s!( -11,    3),
            s!(  -1,   24), s!(  -2,   27), s!(  -4,   28), s!(   3,    3), s!(  -8,    4), s!(  -7,   -3), s!(  -3,  -18), s!(  -3,  -12),
        ],
        [
            s!(   0,    1), s!(   0,   -1), s!(   0,    1), s!(  -0,   -4), s!(   4,    5), s!(  -2,   -9), s!(  -3,  -12), s!(  -2,   -2),
            s!(   0,   -1), s!(  -1,   -4), s!(  -1,    1), s!(   1,   -6), s!(   1,    2), s!(  -2,  -12), s!(   1,   -1), s!(   1,   -0),
            s!(   0,   -4), s!(  -1,   -1), s!(   0,   -7), s!(  -1,   -5), s!(  -1,   -4), s!(  -3,   -6), s!(   1,   -3), s!(   2,    3),
            s!(  -1,   -7), s!(  -0,    0), s!(  -0,    5), s!(   0,    7), s!(   2,    3), s!(   6,    2), s!(  -1,   12), s!(   3,    8),
            s!(  -1,   -5), s!(   1,   -6), s!(  -6,    3), s!(  -3,   -2), s!(   1,   -4), s!(   1,    9), s!(   4,    6), s!(   4,   -5),
            s!(  -3,    5), s!(   4,    5), s!(   7,   20), s!(   3,   -3), s!(   1,    7), s!(  -1,   -0), s!(  -4,   -2), s!(  -5,   -5),
            s!(   4,   28), s!(  -4,   27), s!(  -7,   24), s!(  -7,   -0), s!(   5,   -4), s!(  -9,    8), s!(  -4,   -7), s!( -11,   -0),
            s!(   4,    9), s!(  -1,   22), s!(  15,   23), s!(   1,    5), s!( -10,   -4), s!(  -1,  -16), s!(   5,   -1), s!(  -0,  -13),
        ],
    ],
];
