// ------------------------------------------------------------------------- //
// Generated at 28-12-2024 13:59:28 UTC (e = 0.067562, k = 0.0077, r = 0.70) //
// ------------------------------------------------------------------------- //

use super::*;

#[rustfmt::skip]
pub const BISHOP_PST_PATTERN: [[[PackedEval; 64]; KING_BUCKETS_COUNT]; 2] =
[
    [
        [
            s!( 285,  301), s!( 347,  288), s!( 291,  316), s!( 303,  324), s!( 302,  330), s!( 316,  312), s!( 331,  292), s!( 333,  290),
            s!( 294,  287), s!( 318,  302), s!( 359,  307), s!( 342,  296), s!( 369,  292), s!( 330,  298), s!( 319,  316), s!( 280,  296),
            s!( 330,  296), s!( 346,  318), s!( 355,  305), s!( 391,  301), s!( 365,  305), s!( 421,  308), s!( 363,  319), s!( 359,  313),
            s!( 334,  301), s!( 350,  299), s!( 372,  300), s!( 382,  297), s!( 403,  285), s!( 385,  312), s!( 362,  322), s!( 340,  310),
            s!( 324,  295), s!( 339,  291), s!( 357,  305), s!( 374,  297), s!( 369,  293), s!( 368,  314), s!( 350,  305), s!( 362,  296),
            s!( 327,  292), s!( 345,  291), s!( 359,  306), s!( 357,  302), s!( 363,  324), s!( 372,  307), s!( 368,  298), s!( 373,  287),
            s!( 343,  274), s!( 335,  267), s!( 354,  273), s!( 346,  294), s!( 336,  292), s!( 370,  276), s!( 378,  277), s!( 374,  262),
            s!( 306,  270), s!( 347,  270), s!( 352,  238), s!( 339,  284), s!( 357,  287), s!( 354,  275), s!( 363,  243), s!( 332,  250),
        ],
        [
            s!( 305,  303), s!( 314,  305), s!( 306,  322), s!( 301,  329), s!( 315,  339), s!( 281,  327), s!( 335,  336), s!( 305,  291),
            s!( 285,  306), s!( 336,  306), s!( 332,  309), s!( 343,  313), s!( 344,  308), s!( 304,  341), s!( 282,  323), s!( 279,  302),
            s!( 343,  319), s!( 354,  323), s!( 367,  320), s!( 378,  307), s!( 376,  315), s!( 405,  324), s!( 353,  343), s!( 336,  336),
            s!( 326,  314), s!( 351,  324), s!( 355,  329), s!( 375,  318), s!( 369,  307), s!( 362,  334), s!( 342,  331), s!( 338,  328),
            s!( 350,  301), s!( 337,  314), s!( 354,  332), s!( 354,  322), s!( 372,  302), s!( 349,  331), s!( 351,  315), s!( 356,  289),
            s!( 328,  311), s!( 351,  314), s!( 351,  321), s!( 354,  323), s!( 353,  340), s!( 376,  315), s!( 363,  305), s!( 374,  294),
            s!( 349,  298), s!( 336,  295), s!( 356,  294), s!( 334,  315), s!( 350,  309), s!( 368,  295), s!( 393,  289), s!( 364,  240),
            s!( 319,  269), s!( 350,  275), s!( 334,  296), s!( 351,  300), s!( 342,  300), s!( 373,  287), s!( 360,  277), s!( 376,  244),
        ],
        [
            s!( 288,  300), s!( 332,  311), s!( 300,  331), s!( 309,  334), s!( 292,  336), s!( 312,  318), s!( 330,  321), s!( 318,  275),
            s!( 288,  301), s!( 338,  314), s!( 357,  322), s!( 334,  321), s!( 349,  313), s!( 331,  320), s!( 314,  330), s!( 295,  302),
            s!( 329,  317), s!( 374,  317), s!( 376,  321), s!( 397,  310), s!( 387,  318), s!( 443,  320), s!( 407,  316), s!( 362,  326),
            s!( 335,  316), s!( 382,  311), s!( 369,  326), s!( 407,  303), s!( 391,  305), s!( 391,  318), s!( 377,  319), s!( 364,  319),
            s!( 367,  313), s!( 353,  313), s!( 384,  314), s!( 400,  304), s!( 408,  289), s!( 380,  319), s!( 362,  308), s!( 384,  296),
            s!( 328,  313), s!( 385,  302), s!( 377,  320), s!( 383,  318), s!( 377,  323), s!( 415,  301), s!( 390,  297), s!( 365,  290),
            s!( 370,  275), s!( 353,  295), s!( 370,  304), s!( 364,  313), s!( 374,  299), s!( 382,  305), s!( 383,  293), s!( 356,  270),
            s!( 311,  299), s!( 360,  292), s!( 352,  293), s!( 341,  316), s!( 340,  315), s!( 358,  306), s!( 332,  295), s!( 335,  281),
        ],
        [
            s!( 272,  308), s!( 308,  316), s!( 278,  331), s!( 303,  333), s!( 277,  345), s!( 302,  331), s!( 331,  294), s!( 298,  288),
            s!( 287,  300), s!( 322,  318), s!( 310,  336), s!( 321,  339), s!( 327,  329), s!( 287,  337), s!( 317,  320), s!( 274,  300),
            s!( 342,  308), s!( 369,  328), s!( 348,  332), s!( 361,  327), s!( 349,  333), s!( 381,  336), s!( 357,  335), s!( 330,  334),
            s!( 332,  324), s!( 346,  323), s!( 326,  341), s!( 354,  318), s!( 347,  321), s!( 359,  329), s!( 339,  327), s!( 340,  312),
            s!( 340,  316), s!( 316,  324), s!( 350,  333), s!( 355,  322), s!( 357,  311), s!( 345,  333), s!( 350,  317), s!( 356,  293),
            s!( 324,  318), s!( 353,  323), s!( 356,  335), s!( 354,  330), s!( 355,  338), s!( 372,  325), s!( 356,  305), s!( 346,  309),
            s!( 351,  303), s!( 341,  308), s!( 360,  314), s!( 340,  319), s!( 353,  310), s!( 369,  298), s!( 370,  293), s!( 356,  272),
            s!( 323,  301), s!( 350,  302), s!( 324,  300), s!( 323,  326), s!( 336,  320), s!( 331,  292), s!( 335,  288), s!( 302,  301),
        ],
        [
            s!( 287,  299), s!( 338,  298), s!( 294,  333), s!( 311,  336), s!( 292,  345), s!( 318,  334), s!( 336,  305), s!( 318,  290),
            s!( 303,  294), s!( 331,  323), s!( 321,  331), s!( 333,  335), s!( 355,  323), s!( 353,  328), s!( 333,  321), s!( 287,  316),
            s!( 348,  325), s!( 372,  328), s!( 407,  326), s!( 379,  328), s!( 396,  322), s!( 425,  326), s!( 355,  328), s!( 389,  307),
            s!( 334,  326), s!( 377,  318), s!( 394,  330), s!( 382,  317), s!( 394,  313), s!( 395,  318), s!( 374,  320), s!( 355,  320),
            s!( 371,  320), s!( 349,  318), s!( 393,  313), s!( 407,  304), s!( 381,  307), s!( 388,  316), s!( 384,  315), s!( 363,  298),
            s!( 359,  314), s!( 376,  315), s!( 410,  314), s!( 401,  317), s!( 384,  331), s!( 393,  314), s!( 355,  303), s!( 347,  307),
            s!( 374,  291), s!( 355,  290), s!( 360,  315), s!( 382,  305), s!( 366,  318), s!( 373,  302), s!( 339,  314), s!( 355,  276),
            s!( 323,  294), s!( 376,  308), s!( 334,  313), s!( 348,  333), s!( 344,  309), s!( 346,  301), s!( 351,  296), s!( 318,  291),
        ],
        [
            s!( 303,  305), s!( 357,  320), s!( 293,  316), s!( 302,  320), s!( 294,  326), s!( 307,  319), s!( 340,  308), s!( 310,  260),
            s!( 301,  305), s!( 319,  328), s!( 326,  329), s!( 333,  329), s!( 336,  321), s!( 342,  326), s!( 333,  302), s!( 302,  299),
            s!( 325,  339), s!( 374,  327), s!( 363,  336), s!( 385,  315), s!( 392,  318), s!( 404,  323), s!( 361,  326), s!( 379,  301),
            s!( 349,  329), s!( 348,  335), s!( 358,  337), s!( 359,  326), s!( 389,  298), s!( 377,  320), s!( 363,  317), s!( 341,  316),
            s!( 363,  313), s!( 359,  324), s!( 356,  332), s!( 397,  302), s!( 365,  301), s!( 363,  326), s!( 361,  312), s!( 365,  303),
            s!( 362,  307), s!( 372,  311), s!( 385,  319), s!( 369,  325), s!( 373,  326), s!( 362,  328), s!( 360,  309), s!( 344,  315),
            s!( 363,  288), s!( 404,  281), s!( 399,  297), s!( 367,  312), s!( 347,  316), s!( 368,  294), s!( 344,  313), s!( 339,  301),
            s!( 342,  284), s!( 341,  304), s!( 373,  310), s!( 339,  319), s!( 369,  304), s!( 348,  295), s!( 347,  294), s!( 299,  295),
        ],
        [
            s!( 294,  313), s!( 344,  309), s!( 286,  318), s!( 313,  327), s!( 292,  332), s!( 308,  306), s!( 325,  275), s!( 326,  281),
            s!( 288,  298), s!( 320,  336), s!( 351,  312), s!( 335,  300), s!( 354,  316), s!( 359,  304), s!( 347,  297), s!( 292,  304),
            s!( 316,  337), s!( 363,  327), s!( 369,  342), s!( 387,  315), s!( 380,  303), s!( 413,  303), s!( 363,  325), s!( 384,  294),
            s!( 361,  320), s!( 355,  333), s!( 386,  323), s!( 389,  314), s!( 404,  292), s!( 388,  309), s!( 365,  319), s!( 338,  317),
            s!( 351,  317), s!( 367,  306), s!( 359,  333), s!( 391,  304), s!( 372,  309), s!( 362,  328), s!( 362,  305), s!( 358,  303),
            s!( 363,  288), s!( 377,  312), s!( 379,  322), s!( 372,  325), s!( 367,  335), s!( 350,  332), s!( 361,  294), s!( 331,  299),
            s!( 363,  258), s!( 408,  289), s!( 392,  283), s!( 362,  307), s!( 340,  319), s!( 340,  299), s!( 324,  306), s!( 319,  305),
            s!( 344,  271), s!( 361,  282), s!( 383,  288), s!( 336,  306), s!( 367,  293), s!( 342,  282), s!( 345,  291), s!( 336,  278),
        ],
        [
            s!( 296,  294), s!( 343,  291), s!( 299,  315), s!( 307,  317), s!( 301,  338), s!( 308,  306), s!( 333,  296), s!( 321,  277),
            s!( 295,  294), s!( 345,  319), s!( 335,  316), s!( 328,  303), s!( 366,  321), s!( 367,  319), s!( 316,  289), s!( 284,  292),
            s!( 336,  327), s!( 351,  310), s!( 386,  316), s!( 387,  298), s!( 381,  296), s!( 426,  301), s!( 370,  306), s!( 356,  311),
            s!( 350,  324), s!( 371,  321), s!( 392,  312), s!( 441,  287), s!( 398,  277), s!( 383,  303), s!( 354,  309), s!( 356,  311),
            s!( 379,  304), s!( 370,  297), s!( 399,  307), s!( 411,  288), s!( 383,  287), s!( 369,  306), s!( 372,  288), s!( 318,  302),
            s!( 389,  309), s!( 386,  299), s!( 389,  316), s!( 375,  311), s!( 368,  311), s!( 340,  316), s!( 346,  297), s!( 334,  289),
            s!( 356,  271), s!( 403,  292), s!( 386,  292), s!( 351,  290), s!( 352,  299), s!( 330,  293), s!( 308,  300), s!( 343,  274),
            s!( 322,  276), s!( 362,  267), s!( 361,  297), s!( 370,  299), s!( 349,  286), s!( 338,  274), s!( 339,  297), s!( 326,  282),
        ],
        [
            s!( 289,  289), s!( 341,  273), s!( 286,  301), s!( 308,  318), s!( 299,  314), s!( 306,  304), s!( 331,  292), s!( 321,  277),
            s!( 304,  296), s!( 327,  285), s!( 356,  287), s!( 342,  291), s!( 362,  299), s!( 361,  297), s!( 327,  314), s!( 279,  296),
            s!( 340,  300), s!( 354,  298), s!( 357,  293), s!( 380,  306), s!( 379,  314), s!( 435,  311), s!( 370,  319), s!( 367,  315),
            s!( 332,  301), s!( 354,  282), s!( 367,  297), s!( 393,  277), s!( 404,  286), s!( 385,  308), s!( 372,  322), s!( 349,  307),
            s!( 336,  282), s!( 350,  298), s!( 357,  311), s!( 371,  282), s!( 380,  284), s!( 371,  299), s!( 362,  288), s!( 370,  304),
            s!( 337,  291), s!( 349,  292), s!( 361,  317), s!( 365,  298), s!( 368,  322), s!( 367,  305), s!( 375,  290), s!( 370,  275),
            s!( 353,  282), s!( 342,  273), s!( 354,  281), s!( 347,  298), s!( 333,  288), s!( 357,  281), s!( 379,  265), s!( 379,  262),
            s!( 322,  284), s!( 359,  281), s!( 352,  264), s!( 343,  294), s!( 363,  302), s!( 350,  277), s!( 375,  252), s!( 326,  260),
        ],
        [
            s!( 317,  289), s!( 327,  290), s!( 310,  308), s!( 306,  318), s!( 303,  329), s!( 298,  329), s!( 339,  319), s!( 310,  294),
            s!( 300,  290), s!( 331,  279), s!( 344,  308), s!( 342,  306), s!( 358,  310), s!( 318,  309), s!( 301,  344), s!( 290,  306),
            s!( 346,  301), s!( 355,  305), s!( 372,  298), s!( 400,  294), s!( 390,  316), s!( 419,  329), s!( 370,  330), s!( 354,  339),
            s!( 344,  301), s!( 351,  306), s!( 362,  304), s!( 383,  293), s!( 385,  311), s!( 372,  325), s!( 356,  334), s!( 349,  320),
            s!( 355,  291), s!( 339,  317), s!( 365,  313), s!( 368,  315), s!( 385,  298), s!( 363,  322), s!( 359,  328), s!( 370,  294),
            s!( 344,  312), s!( 362,  296), s!( 364,  321), s!( 368,  317), s!( 364,  307), s!( 376,  294), s!( 375,  291), s!( 373,  297),
            s!( 372,  292), s!( 346,  304), s!( 363,  298), s!( 338,  309), s!( 356,  294), s!( 375,  291), s!( 398,  282), s!( 379,  244),
            s!( 339,  295), s!( 371,  292), s!( 335,  292), s!( 357,  306), s!( 349,  300), s!( 374,  279), s!( 348,  270), s!( 359,  256),
        ],
        [
            s!( 295,  285), s!( 338,  292), s!( 298,  321), s!( 308,  321), s!( 295,  329), s!( 313,  307), s!( 334,  319), s!( 324,  280),
            s!( 294,  292), s!( 335,  289), s!( 356,  309), s!( 333,  314), s!( 359,  325), s!( 341,  324), s!( 322,  321), s!( 282,  306),
            s!( 324,  312), s!( 369,  305), s!( 377,  305), s!( 393,  299), s!( 384,  302), s!( 444,  308), s!( 401,  318), s!( 359,  318),
            s!( 331,  322), s!( 382,  312), s!( 365,  325), s!( 405,  299), s!( 397,  295), s!( 397,  286), s!( 375,  319), s!( 375,  313),
            s!( 365,  300), s!( 363,  316), s!( 391,  308), s!( 404,  297), s!( 403,  300), s!( 375,  301), s!( 358,  322), s!( 381,  299),
            s!( 335,  314), s!( 380,  298), s!( 388,  306), s!( 380,  306), s!( 376,  312), s!( 411,  300), s!( 378,  302), s!( 363,  295),
            s!( 389,  302), s!( 362,  289), s!( 367,  296), s!( 360,  302), s!( 361,  291), s!( 371,  299), s!( 376,  282), s!( 359,  277),
            s!( 315,  299), s!( 368,  300), s!( 353,  295), s!( 345,  316), s!( 344,  311), s!( 362,  283), s!( 327,  273), s!( 333,  275),
        ],
        [
            s!( 275,  295), s!( 319,  307), s!( 281,  330), s!( 304,  325), s!( 281,  330), s!( 306,  321), s!( 334,  303), s!( 309,  288),
            s!( 289,  296), s!( 330,  296), s!( 316,  314), s!( 325,  329), s!( 335,  319), s!( 297,  326), s!( 337,  312), s!( 290,  310),
            s!( 353,  320), s!( 368,  302), s!( 355,  314), s!( 372,  311), s!( 358,  332), s!( 395,  319), s!( 358,  323), s!( 349,  321),
            s!( 327,  320), s!( 348,  312), s!( 359,  317), s!( 356,  291), s!( 359,  290), s!( 357,  308), s!( 348,  321), s!( 344,  321),
            s!( 359,  299), s!( 323,  314), s!( 362,  313), s!( 362,  315), s!( 373,  294), s!( 355,  339), s!( 353,  324), s!( 368,  298),
            s!( 328,  293), s!( 366,  309), s!( 367,  310), s!( 371,  303), s!( 352,  323), s!( 379,  306), s!( 366,  316), s!( 346,  307),
            s!( 380,  298), s!( 348,  310), s!( 372,  290), s!( 338,  301), s!( 364,  293), s!( 372,  296), s!( 373,  278), s!( 369,  296),
            s!( 354,  297), s!( 374,  299), s!( 325,  309), s!( 333,  309), s!( 355,  301), s!( 335,  279), s!( 359,  281), s!( 307,  285),
        ],
        [
            s!( 291,  281), s!( 341,  295), s!( 296,  324), s!( 309,  333), s!( 295,  329), s!( 317,  335), s!( 335,  288), s!( 320,  292),
            s!( 304,  282), s!( 337,  318), s!( 325,  333), s!( 333,  332), s!( 356,  311), s!( 354,  293), s!( 326,  304), s!( 288,  306),
            s!( 342,  309), s!( 373,  313), s!( 397,  320), s!( 374,  335), s!( 387,  307), s!( 426,  307), s!( 362,  309), s!( 382,  306),
            s!( 336,  319), s!( 386,  319), s!( 388,  324), s!( 384,  310), s!( 388,  304), s!( 394,  303), s!( 375,  324), s!( 357,  312),
            s!( 368,  303), s!( 355,  314), s!( 394,  326), s!( 397,  301), s!( 368,  309), s!( 385,  306), s!( 382,  323), s!( 370,  303),
            s!( 359,  316), s!( 373,  310), s!( 396,  308), s!( 406,  296), s!( 396,  321), s!( 400,  311), s!( 362,  303), s!( 362,  301),
            s!( 379,  296), s!( 365,  293), s!( 364,  292), s!( 386,  303), s!( 370,  291), s!( 373,  287), s!( 347,  296), s!( 364,  283),
            s!( 330,  298), s!( 373,  298), s!( 341,  299), s!( 350,  316), s!( 352,  304), s!( 341,  295), s!( 343,  297), s!( 326,  305),
        ],
        [
            s!( 294,  293), s!( 353,  306), s!( 294,  320), s!( 302,  315), s!( 294,  317), s!( 306,  311), s!( 335,  288), s!( 318,  257),
            s!( 290,  297), s!( 332,  323), s!( 327,  308), s!( 336,  320), s!( 343,  299), s!( 357,  302), s!( 328,  280), s!( 314,  293),
            s!( 315,  319), s!( 375,  328), s!( 375,  352), s!( 389,  306), s!( 391,  302), s!( 411,  309), s!( 380,  319), s!( 389,  293),
            s!( 346,  329), s!( 356,  347), s!( 350,  336), s!( 391,  326), s!( 385,  288), s!( 388,  307), s!( 367,  312), s!( 341,  315),
            s!( 360,  304), s!( 354,  333), s!( 362,  330), s!( 391,  307), s!( 375,  304), s!( 367,  306), s!( 373,  305), s!( 357,  299),
            s!( 373,  300), s!( 384,  302), s!( 391,  308), s!( 381,  317), s!( 371,  318), s!( 360,  321), s!( 357,  316), s!( 344,  316),
            s!( 360,  279), s!( 398,  278), s!( 408,  276), s!( 372,  298), s!( 342,  295), s!( 363,  279), s!( 339,  315), s!( 316,  307),
            s!( 326,  295), s!( 336,  289), s!( 378,  285), s!( 348,  323), s!( 365,  309), s!( 347,  296), s!( 346,  307), s!( 315,  305),
        ],
        [
            s!( 295,  303), s!( 344,  302), s!( 283,  301), s!( 312,  320), s!( 291,  307), s!( 307,  297), s!( 328,  277), s!( 323,  277),
            s!( 279,  295), s!( 334,  329), s!( 354,  324), s!( 336,  301), s!( 355,  308), s!( 366,  287), s!( 334,  274), s!( 294,  296),
            s!( 318,  333), s!( 365,  330), s!( 366,  332), s!( 386,  313), s!( 384,  276), s!( 416,  286), s!( 376,  308), s!( 382,  305),
            s!( 342,  308), s!( 374,  336), s!( 377,  323), s!( 400,  300), s!( 400,  275), s!( 395,  286), s!( 360,  307), s!( 344,  305),
            s!( 364,  311), s!( 364,  315), s!( 371,  296), s!( 392,  282), s!( 378,  295), s!( 348,  314), s!( 357,  307), s!( 351,  293),
            s!( 367,  290), s!( 382,  307), s!( 382,  294), s!( 379,  301), s!( 359,  310), s!( 350,  317), s!( 348,  300), s!( 340,  304),
            s!( 351,  260), s!( 407,  264), s!( 399,  275), s!( 363,  301), s!( 341,  316), s!( 339,  303), s!( 323,  299), s!( 316,  282),
            s!( 334,  269), s!( 361,  271), s!( 375,  292), s!( 346,  317), s!( 366,  296), s!( 348,  306), s!( 331,  299), s!( 335,  300),
        ],
        [
            s!( 297,  294), s!( 342,  288), s!( 298,  311), s!( 308,  313), s!( 299,  334), s!( 310,  303), s!( 332,  288), s!( 324,  277),
            s!( 303,  293), s!( 351,  324), s!( 338,  318), s!( 332,  312), s!( 372,  318), s!( 371,  300), s!( 318,  285), s!( 281,  292),
            s!( 337,  323), s!( 358,  318), s!( 383,  305), s!( 387,  304), s!( 387,  286), s!( 431,  298), s!( 368,  304), s!( 352,  314),
            s!( 339,  320), s!( 378,  329), s!( 393,  306), s!( 424,  303), s!( 382,  274), s!( 384,  290), s!( 354,  307), s!( 354,  316),
            s!( 376,  307), s!( 372,  296), s!( 383,  290), s!( 400,  287), s!( 372,  290), s!( 360,  306), s!( 373,  298), s!( 332,  293),
            s!( 370,  304), s!( 382,  281), s!( 385,  304), s!( 372,  315), s!( 370,  306), s!( 357,  315), s!( 340,  302), s!( 342,  289),
            s!( 358,  271), s!( 390,  273), s!( 385,  296), s!( 355,  289), s!( 343,  298), s!( 334,  283), s!( 317,  276), s!( 345,  273),
            s!( 324,  273), s!( 366,  283), s!( 352,  286), s!( 374,  309), s!( 366,  288), s!( 330,  287), s!( 336,  301), s!( 330,  286),
        ],
    ],
    [
        [
            s!(   2,    8), s!(  -4,    6), s!(   8,   10), s!(  -1,   -4), s!(  13,    8), s!(  15,    3), s!(  13,   20), s!(  -2,    1),
            s!(   7,   13), s!( -23,    2), s!(  -3,    2), s!(  14,   -6), s!(  -6,   -6), s!(   4,   16), s!( -18,   -7), s!( -19,   14),
            s!( -19,    6), s!( -14,    5), s!( -26,   16), s!( -10,  -11), s!( -34,   28), s!(   8,   20), s!(  -9,   21), s!(  -9,   30),
            s!(  -3,    6), s!(  -8,   -5), s!( -11,   -9), s!( -16,   14), s!(  11,    7), s!(  -7,    4), s!(   5,   -8), s!( -13,   11),
            s!( -13,    4), s!(  -5,  -13), s!( -10,   11), s!(  25,   -5), s!(  -7,   16), s!(  -3,  -23), s!(   1,   -4), s!( -12,    5),
            s!(  -0,  -17), s!(  -6,   14), s!(   1,   16), s!(   2,   22), s!(  -3,   -3), s!(  -9,    4), s!(  -7,   -3), s!( -14,    3),
            s!( -12,    3), s!(  -3,  -15), s!(  -7,    2), s!(  -7,   -5), s!(  -5,   -1), s!( -12,   -9), s!( -17,    3), s!( -10,  -19),
            s!(  -5,  -26), s!(  -0,   -0), s!(  -5,  -33), s!(   0,   -4), s!(  -5,   -9), s!(  -8,  -12), s!( -10,   -4), s!( -11,    6),
        ],
        [
            s!( -15,   11), s!( -29,   10), s!( -17,   12), s!(   3,   13), s!(  -4,    3), s!( -42,   -3), s!(   4,  -10), s!(   8,    4),
            s!(  -9,   38), s!( -26,   14), s!( -11,    5), s!( -27,   11), s!( -13,   10), s!( -51,  -13), s!( -22,   -6), s!( -60,  -29),
            s!( -16,    4), s!( -23,   12), s!( -27,   14), s!( -25,    8), s!(  13,   10), s!(   3,   22), s!( -21,   18), s!(  10,   16),
            s!(   8,    3), s!( -17,    9), s!( -20,   11), s!(  -5,   15), s!( -25,   14), s!( -17,   14), s!( -11,   -2), s!(  -8,    4),
            s!( -18,   11), s!(  -8,   12), s!( -10,   26), s!(  -9,   10), s!( -18,    9), s!( -18,   -2), s!( -22,   11), s!( -12,    2),
            s!(  -2,   -0), s!(  -2,   15), s!(  -6,   18), s!( -10,   11), s!( -15,   11), s!( -23,   13), s!( -18,    2), s!( -18,    8),
            s!(   1,    2), s!(  -2,   -3), s!(  -8,   -1), s!( -14,    4), s!( -16,    2), s!( -18,   11), s!( -23,    2), s!( -15,    4),
            s!(  15,  -12), s!(   6,  -25), s!(  -8,  -14), s!( -16,    4), s!( -14,    6), s!( -24,    5), s!(  -1,    8), s!( -29,    2),
        ],
        [
            s!(   3,   12), s!(   3,    5), s!(   3,    3), s!(   0,    9), s!( -12,    3), s!(   4,   -7), s!(  -8,   -9), s!(   5,   -3),
            s!(  17,   12), s!( -10,  -13), s!(  35,   -7), s!(  11,  -14), s!( -17,  -19), s!( -71,   -4), s!( -17,  -21), s!(  15,  -24),
            s!(   3,  -12), s!(   4,   -3), s!(  -8,   -4), s!(  20,  -10), s!(  17,  -16), s!(  29,   -4), s!(   8,   -8), s!(   8,  -26),
            s!(   6,   11), s!( -23,   -1), s!(  37,  -16), s!(   3,  -11), s!(  34,  -19), s!(   9,  -13), s!(  12,   -8), s!(  -6,   -3),
            s!( -13,    3), s!(  23,   -9), s!(  10,   -7), s!(  24,  -11), s!(  -7,  -10), s!(  13,  -14), s!(  11,  -13), s!(  -1,    0),
            s!(  10,   -7), s!(  18,  -10), s!(   7,    0), s!(   1,   -6), s!(  10,   -8), s!(   5,  -15), s!(   3,   -3), s!( -22,   -6),
            s!(  -9,   -4), s!(  15,   -7), s!(  -5,   -6), s!(  13,  -14), s!(  -4,  -10), s!(  12,  -12), s!(  -6,  -12), s!(   6,   -7),
            s!( -19,   -9), s!( -10,   -7), s!(  -3,   -0), s!( -13,   -4), s!(   3,   -6), s!( -19,   -2), s!(  -9,  -15), s!(  -4,    2),
        ],
        [
            s!(  -4,   -3), s!(  -9,    5), s!(   6,  -10), s!(  -3,    4), s!(  -2,   -7), s!(   2,   -2), s!(  -3,  -19), s!( -13,   -4),
            s!(  15,   15), s!( -15,   -8), s!(   7,  -11), s!( -11,  -15), s!( -15,   -2), s!( -40,  -18), s!(  11,   -2), s!(  13,  -27),
            s!(  -9,    2), s!(  -3,  -10), s!(  14,  -18), s!(  26,   -8), s!(  10,  -21), s!( -12,   -6), s!(   5,  -30), s!(  20,  -34),
            s!(   9,    5), s!( -14,    1), s!(  17,   -6), s!( -11,  -14), s!( -15,   -7), s!( -10,   -4), s!( -16,    1), s!(  -3,  -16),
            s!(  -7,   10), s!(   8,    3), s!( -17,   -1), s!( -11,   -6), s!( -28,   -2), s!( -22,    5), s!( -21,   -5), s!( -13,    5),
            s!(  10,    6), s!(  -4,   -5), s!( -12,    2), s!( -21,   -9), s!( -25,    1), s!( -29,   -6), s!( -13,   -2), s!( -12,   -1),
            s!(  -4,  -11), s!(  -4,   -1), s!( -14,  -11), s!( -19,   -1), s!( -27,   -6), s!( -22,    3), s!( -27,    3), s!(  -8,   -2),
            s!(  -2,  -14), s!(  -4,  -19), s!( -16,    4), s!( -12,   -9), s!( -25,   -3), s!( -26,   -4), s!( -11,   -6), s!( -20,    7),
        ],
        [
            s!( -11,   -6), s!(  -6,  -14), s!(  -2,  -13), s!(   4,    3), s!(  -6,    1), s!(   2,   -5), s!(  -2,   -4), s!( -14,    5),
            s!(  -4,    8), s!(   4,  -14), s!( -24,  -18), s!(  -7,  -11), s!( -13,  -19), s!(  10,   -0), s!(  19,   -8), s!(  -7,    4),
            s!(  -5,   -8), s!( -11,   -7), s!(   6,  -12), s!( -13,   -3), s!(   4,   -4), s!( -22,   -2), s!(  -6,  -10), s!( -19,  -16),
            s!(   3,    8), s!(  13,   -7), s!(   7,  -23), s!(   4,  -15), s!(  31,  -10), s!(   1,   -5), s!(  -3,   -9), s!(  -2,   -2),
            s!(  18,    4), s!(   7,   -1), s!( -13,   -3), s!(  -3,  -12), s!(  12,  -10), s!(   3,   -8), s!(  -6,   -6), s!( -23,   10),
            s!(  -1,  -10), s!(   7,   -6), s!(   3,  -11), s!(  -7,    0), s!(  -8,   -5), s!(  -3,   -5), s!(   6,    7), s!(   9,   10),
            s!( -16,   -3), s!(  -3,    3), s!(   6,   -3), s!(  -8,   -4), s!(  -2,   -6), s!(  -5,   -8), s!(  -0,   -5), s!(  -4,    6),
            s!(  -9,   -1), s!(  -6,   -1), s!( -20,    2), s!(   3,    1), s!(  -2,   -1), s!(  -8,   -4), s!( -13,   -2), s!(   9,    3),
        ],
        [
            s!(  -3,  -13), s!(   4,    6), s!(  -4,    0), s!( -12,  -11), s!(   5,    4), s!(   0,   19), s!(   1,   14), s!(  -8,   10),
            s!(  17,  -64), s!( -12,  -27), s!( -23,   -9), s!(  -5,  -12), s!(  -6,   -4), s!(   2,    1), s!(  -9,    7), s!(  16,    3),
            s!(  17,   -8), s!(  43,   -4), s!(  10,   -1), s!(  15,   -6), s!(  17,  -10), s!( -20,  -12), s!( -17,   -7), s!( -27,    8),
            s!(   0,   -3), s!(   6,   -9), s!( -18,   -1), s!(  10,  -10), s!(  -3,    3), s!(  38,  -11), s!(   2,   -1), s!(  -6,   -2),
            s!(   3,   15), s!(  -6,   -8), s!(  -3,   -3), s!( -19,   -1), s!(  21,  -14), s!(   9,    3), s!(  35,  -15), s!(  14,    4),
            s!( -17,    5), s!( -16,    6), s!(  -7,   -2), s!(  -1,   -5), s!(  -5,   -7), s!(  14,   -2), s!(  14,   -4), s!(  28,   10),
            s!( -24,    9), s!( -15,    6), s!(  -9,   -3), s!( -10,  -10), s!(   4,   -3), s!(   3,   -3), s!(  15,   -3), s!(  14,   12),
            s!(  -0,    3), s!( -22,   13), s!( -14,   -2), s!(   7,    7), s!(  -8,    5), s!(  -0,    6), s!(  14,   -7), s!(  22,  -20),
        ],
        [
            s!(  -0,   -3), s!(  -0,   -7), s!(  -2,  -13), s!(  -9,   -1), s!(   6,   16), s!(   6,   28), s!(  -4,   32), s!(  -7,    0),
            s!(-107,  -83), s!(  -3,   -5), s!(  -2,  -10), s!(   2,   -8), s!(   6,   10), s!(   7,   10), s!(   4,   25), s!(  19,   25),
            s!(  20,   -5), s!(  19,    3), s!(  29,   11), s!(   9,    7), s!( -27,   -4), s!( -31,   13), s!(  -3,    8), s!( -21,    6),
            s!(  -2,  -21), s!(  14,   -5), s!(  -6,   -3), s!(  -1,    2), s!(  36,    9), s!( -28,   15), s!(   8,   -0), s!(  -7,    7),
            s!( -21,    7), s!( -12,    5), s!(  -6,   -7), s!(   8,   -9), s!(  19,    1), s!(  40,    4), s!(  -6,   -4), s!(  15,    5),
            s!( -21,   34), s!( -33,   23), s!( -12,   12), s!(  -6,    2), s!(  16,   -7), s!(  19,    7), s!(  32,    5), s!(  -1,    4),
            s!( -24,   17), s!( -27,   27), s!( -26,   20), s!(  -4,    2), s!(   5,   -6), s!(  31,  -13), s!(  17,    2), s!(  17,   20),
            s!(   0,   24), s!( -18,   28), s!( -10,   -2), s!( -14,    6), s!(   8,   16), s!(   0,    3), s!(  18,    6), s!(  13,   -0),
        ],
        [
            s!(  -0,    1), s!(   7,    7), s!(   4,    6), s!(  10,   19), s!(   3,   16), s!(   5,   14), s!(   4,    7), s!(   2,   -0),
            s!(   6,   -1), s!(   6,    1), s!(   2,    9), s!(   6,    7), s!(   6,    1), s!(   6,   19), s!(  -1,   15), s!(   5,    0),
            s!(  20,    3), s!( -21,  -17), s!(  -7,   -0), s!(   8,   10), s!(   3,   19), s!(  -9,   14), s!( -15,   17), s!( -19,   -3),
            s!(  -2,   -8), s!(   9,  -10), s!(  10,   18), s!(   7,   15), s!(   4,   26), s!( -12,    1), s!(  19,   31), s!(   3,   -3),
            s!(  19,    4), s!(  -7,   -3), s!(   1,   -5), s!(  14,   13), s!(  32,   15), s!(  24,   18), s!(  16,   -9), s!( -10,   -3),
            s!( -13,   24), s!(  -8,    7), s!( -25,   16), s!( -21,    6), s!(  37,  -11), s!(   5,   13), s!(   3,    8), s!( -12,   -7),
            s!(   7,    4), s!( -28,   32), s!( -18,    4), s!(   5,   -9), s!(  21,  -21), s!(  34,   20), s!(  20,   11), s!( -10,   -7),
            s!( -24,   10), s!( -25,    8), s!( -21,   16), s!( -16,  -11), s!( -15,   12), s!(  11,  -11), s!(  18,    1), s!(  -4,  -13),
        ],
        [
            s!(   1,    5), s!(  -1,   10), s!(   6,   21), s!(   2,    2), s!(   7,   10), s!(   4,   -1), s!(   4,    7), s!(   1,   -0),
            s!(   3,   -0), s!(  -2,   21), s!(  -1,   17), s!(  10,   16), s!(  -6,   -2), s!(   3,   11), s!(  -5,   -4), s!( -13,    4),
            s!(   5,    5), s!(   6,   -1), s!( -16,   12), s!(   3,   -1), s!( -17,   11), s!(  -1,   12), s!(  -3,    5), s!(  -5,   14),
            s!( -10,   -1), s!(  -1,   -6), s!(  -7,   -6), s!( -18,    3), s!(   7,    1), s!(   2,    1), s!(   5,    6), s!(  -7,   -1),
            s!(  -8,   -3), s!(  -7,  -15), s!( -12,  -20), s!(   5,  -17), s!(  -6,    5), s!(   2,   12), s!(   3,    8), s!( -12,    4),
            s!(  -8,  -11), s!(  -8,  -13), s!(  -0,   -9), s!(   5,  -17), s!(  -1,   -5), s!(   1,   12), s!( -11,   14), s!(  -3,    5),
            s!( -13,   -6), s!(   1,   -7), s!(   1,   -5), s!(  -3,   -3), s!(   7,   13), s!( -11,    8), s!(  -4,   29), s!(  -8,  -19),
            s!(   2,   -9), s!(  10,    6), s!(  -1,  -10), s!(   4,    1), s!(  -4,   -1), s!(  -5,   13), s!(  -5,    6), s!(  -7,    3),
        ],
        [
            s!(  -5,    6), s!(  -2,   25), s!(   2,   30), s!(   4,    8), s!(  -1,   -7), s!( -11,  -18), s!(   1,  -11), s!(   3,   -3),
            s!( -10,   21), s!( -11,   39), s!(   3,   25), s!(  -8,   18), s!( -10,    9), s!( -14,  -25), s!(  -8,  -16), s!( -17,  -13),
            s!( -10,    6), s!( -15,   25), s!(  -9,   33), s!( -14,   36), s!(  13,   18), s!(   6,   -4), s!(  -8,    7), s!(   5,   -1),
            s!(   7,    5), s!(  -4,   14), s!( -10,    8), s!(  -3,   16), s!( -13,    3), s!(  -5,   39), s!(  -7,   20), s!(   1,   12),
            s!(  -8,    1), s!(  -2,   -3), s!(  -2,    2), s!(  -6,  -21), s!(  -4,   21), s!( -10,   13), s!(  -3,   42), s!(  -7,    6),
            s!(  -0,  -11), s!(   2,  -20), s!(  -7,  -22), s!(   5,   11), s!(  -9,    3), s!(  -9,   19), s!( -11,   15), s!(  -8,   16),
            s!(   1,  -18), s!(   2,  -19), s!(   2,  -22), s!(   2,    2), s!(  -2,   27), s!(  -9,    9), s!( -12,   22), s!(  -8,   21),
            s!(   1,  -15), s!(   6,  -27), s!(   4,  -18), s!(  -3,   10), s!(   2,   24), s!(  -7,   13), s!(  -0,   -5), s!(   0,   38),
        ],
        [
            s!(   5,   23), s!(   3,   15), s!(   4,   14), s!(   1,    6), s!(   2,    8), s!(   3,    3), s!(  -3,  -14), s!(  -0,   -6),
            s!(   9,   13), s!(  -7,   27), s!(  15,   25), s!(   2,   -1), s!(  -2,   -3), s!( -11,   -8), s!(  -6,  -21), s!(   3,   -3),
            s!(   5,  -14), s!(  -1,   12), s!(  -3,   16), s!(   3,   13), s!( -11,  -17), s!(   8,  -14), s!(  10,  -11), s!(   2,  -10),
            s!(  -6,   11), s!( -18,    3), s!(  16,    6), s!(  -2,    3), s!(   3,  -28), s!(   3,   -4), s!(   7,   -2), s!(  -7,    7),
            s!(   3,   -2), s!(   2,   11), s!(   0,  -18), s!(  18,   -7), s!( -18,   -1), s!(  12,    8), s!(   3,   26), s!(   3,   10),
            s!(  12,   -8), s!(   8,  -25), s!(   5,  -20), s!(   1,   -7), s!(   9,    1), s!(   6,   18), s!(  -6,   29), s!( -12,   -2),
            s!(  -6,  -20), s!(   3,  -17), s!(   4,   -9), s!(   7,    5), s!( -15,   -2), s!(   6,   10), s!( -11,    2), s!(   6,   13),
            s!( -12,  -21), s!( -10,  -18), s!(   2,   -7), s!(  -7,   10), s!(  -2,   18), s!( -12,    8), s!(  -3,   11), s!(  -6,   29),
        ],
        [
            s!(   6,    7), s!(  -3,   -3), s!(   1,    1), s!(  -0,    6), s!(   1,    5), s!(   1,    5), s!(  -1,  -11), s!(  -7,  -12),
            s!(  13,   20), s!(  -2,    8), s!(   7,    2), s!(  -1,    2), s!(  -6,    9), s!( -13,  -10), s!(  -2,   -5), s!(  12,   -7),
            s!(  -7,    9), s!(  -9,   10), s!(   5,    3), s!(  17,    0), s!(   4,  -12), s!(  -3,  -14), s!(  10,   -2), s!(  21,    6),
            s!(   1,   -5), s!(  -5,    9), s!(  25,    2), s!(  -2,  -19), s!( -15,  -42), s!(   1,  -14), s!( -11,   20), s!(   9,   10),
            s!(   2,  -18), s!(   2,    6), s!( -13,    2), s!(  -9,   -3), s!( -27,   -7), s!( -14,    7), s!( -10,   14), s!(  -1,   12),
            s!(  11,  -18), s!(   1,  -22), s!( -13,  -27), s!( -18,   -3), s!( -20,   -1), s!( -17,    6), s!( -11,   -4), s!(   3,    8),
            s!(   9,  -17), s!(   0,  -32), s!(  -2,   -1), s!(  -6,    2), s!( -22,   -3), s!( -10,   -3), s!( -16,   13), s!(  -2,    5),
            s!(   2,  -12), s!(   3,  -12), s!(  -0,    8), s!(   1,    2), s!( -11,    0), s!(  -8,   10), s!(   9,   22), s!( -12,   10),
        ],
        [
            s!(  -3,    3), s!(  -2,   -2), s!(  -1,   -5), s!(   1,   -2), s!(  -1,    1), s!(  -1,    2), s!(  -2,   -8), s!(  -5,   -1),
            s!(   2,   16), s!(   4,   13), s!(  -6,   -3), s!(  -2,   -3), s!(  -3,    7), s!(   6,   11), s!(   8,    8), s!(   2,    6),
            s!(   1,   26), s!(  -3,    6), s!(  -2,    2), s!(  -5,    3), s!(  -6,   -4), s!(  -7,    5), s!(  -7,   -1), s!(   7,   17),
            s!(   3,   29), s!(  11,   13), s!(   4,   -0), s!( -12,   -6), s!(   6,   -7), s!(  -1,    4), s!(   3,   18), s!(   6,    9),
            s!(   4,   -1), s!(   2,   29), s!(   2,   13), s!(  -2,   14), s!(   3,   -5), s!(  12,   10), s!(  -3,   -3), s!(  -4,   12),
            s!(  -7,   -5), s!(   1,  -14), s!(  -2,   -8), s!(  -2,   -4), s!(   4,    4), s!( -10,  -10), s!(   7,   -1), s!(  -3,    1),
            s!(  -6,   -9), s!( -12,   -5), s!(  -4,    4), s!(  -1,    1), s!(  -0,    8), s!(  -2,    1), s!(  11,   -4), s!(   1,    1),
            s!(   1,   16), s!(   2,   21), s!( -13,   30), s!(  -6,    9), s!(  -4,    3), s!(  -6,   15), s!(  -0,    9), s!(  -1,   -4),
        ],
        [
            s!(  -1,   -5), s!(   1,   -0), s!(  -1,    2), s!(  -5,   -7), s!(   2,   12), s!(   1,   25), s!(   2,   21), s!(  -1,   15),
            s!(  -4,  -11), s!(  -1,   -6), s!(  -6,   -2), s!(   1,   11), s!(   1,   15), s!(   7,   33), s!(   4,   35), s!(  11,    7),
            s!(   4,    2), s!(  17,   -0), s!(   5,    6), s!(   7,    8), s!(   7,   16), s!(  -9,   16), s!(  -3,   19), s!( -20,    6),
            s!(   1,   15), s!(  16,   18), s!( -14,   23), s!(   9,   18), s!(  -3,    6), s!(   8,   23), s!(   9,    2), s!( -10,   -4),
            s!(   4,   19), s!(  -1,   28), s!(   9,   32), s!( -14,   24), s!(  23,    3), s!(  20,   -9), s!(  19,    7), s!(  29,   12),
            s!( -22,    0), s!( -20,   24), s!( -13,    3), s!(   5,    4), s!(  -1,   -3), s!(  22,  -10), s!(  22,  -10), s!(  23,    7),
            s!(   1,    8), s!( -10,   18), s!( -14,   13), s!(  -9,   13), s!(  12,   14), s!(   7,   -4), s!(  18,   -4), s!(  19,   -1),
            s!(   6,   35), s!(  -8,   40), s!( -10,   19), s!(  -2,   11), s!(  -4,   16), s!(   9,   13), s!(   6,    2), s!(  12,   -6),
        ],
        [
            s!(  -1,   -1), s!(  -0,   -3), s!(   1,    1), s!(  -1,    5), s!(   3,   18), s!(   2,   21), s!(   4,   37), s!(  -1,   11),
            s!( -23,  -15), s!(  -0,   -2), s!(   1,   -0), s!(   2,   10), s!(  11,   38), s!(  15,   40), s!(   7,   43), s!(   8,   18),
            s!(   4,   -1), s!(   6,    3), s!(   7,   17), s!(   4,   35), s!(   1,   35), s!( -20,   29), s!(  -1,   16), s!(  -6,    3),
            s!(   1,   -3), s!(   9,   38), s!(  -2,   30), s!(  -0,   31), s!(  13,   30), s!( -10,   14), s!(  18,   12), s!(  -4,   -6),
            s!(  -0,   12), s!(  14,   50), s!(   8,   17), s!(  -1,   23), s!(   4,   -2), s!(  25,   13), s!(   1,   -1), s!(  -1,    1),
            s!(  -7,   18), s!( -21,   24), s!(  -7,   32), s!( -16,    3), s!(  23,    3), s!(   6,   -5), s!(  31,    3), s!(  -2,   -8),
            s!(   5,   31), s!( -23,   51), s!( -23,   24), s!(  -2,   23), s!(   3,    3), s!(  24,    1), s!(   5,  -19), s!(  -0,    7),
            s!(   8,   60), s!(  -9,   11), s!(  -3,   29), s!(  -7,   21), s!(   7,   26), s!(  -2,    1), s!(   7,    3), s!(   7,   -0),
        ],
        [
            s!(   0,    2), s!(   2,    3), s!(   2,    4), s!(   3,    7), s!(   2,   10), s!(   2,    5), s!(   2,    7), s!(   2,    3),
            s!(   3,   -0), s!(   1,   -2), s!(   3,    7), s!(   3,    7), s!(   1,    3), s!(   7,   37), s!(   1,   14), s!(   0,    2),
            s!(   7,    2), s!(  -8,   -9), s!(  -2,   -3), s!(   3,    9), s!(   4,    8), s!(   1,   15), s!(  -2,   16), s!( -12,   -2),
            s!(  -1,   -2), s!(   4,    9), s!(   6,   10), s!(   1,    4), s!(   9,   26), s!(  -5,   10), s!(  13,    6), s!(   2,   -2),
            s!(   6,    7), s!(  -6,   -1), s!(   2,   12), s!(   0,   11), s!(  11,    8), s!(   4,    9), s!(  11,   -0), s!(  -1,    3),
            s!(  -3,    9), s!(   0,    1), s!( -15,   22), s!( -15,    8), s!(  28,  -11), s!(   4,    7), s!(   1,   -6), s!( -14,   -6),
            s!(   2,    5), s!( -27,   20), s!(  -1,   29), s!(  12,   16), s!(   9,  -12), s!(  16,    8), s!(   8,    9), s!(   0,   -1),
            s!(  -3,   10), s!(  -6,   17), s!(  -3,   27), s!(  -5,    6), s!(  -5,    8), s!(  14,   -2), s!(   5,    3), s!(  -4,   -4),
        ],
    ],
];
