# Inanis
UCI chess engine written in Rust, the successor of [Proxima b](https://github.com/Tearth/Proxima-b), [Proxima b 2.0](https://github.com/Tearth/Proxima-b-2.0) and [Cosette](https://github.com/Tearth/Cosette). The project is written after hours, with the goal to reach a strength of 3000 Elo. Perfect as a sparring partner for other chess engines, since it was heavily tested using very fast games. Supports Syzygy tablebases, MultiPV, pondering and multithreading.

**Current strength**: 3100 Elo (01-02-2025)

**Documentation**: https://tearth.dev/Inanis/

## Releases
| Version                                                       | Release date | Elo  | Main changes |
|---------------------------------------------------------------|--------------|------|--------------|
| [1.6.0](https://github.com/Tearth/Inanis/releases/tag/v1.6.0) | 01-02-2025   | 3100 | More evaluation features, improved time management |
| [1.5.0](https://github.com/Tearth/Inanis/releases/tag/v1.5.0) | 01-11-2024   | 3000 | Aspiration windows, improved performance and multithreading |
| [1.4.0](https://github.com/Tearth/Inanis/releases/tag/v1.4.0) | 03-08-2024   | 2950 | Check extensions, relative PST, countermove heuristic |
| [1.3.0](https://github.com/Tearth/Inanis/releases/tag/v1.3.0) | 14-06-2024   | 2900 | Gradient descent tuner, improved SEE and evaluation |
| [1.2.1](https://github.com/Tearth/Inanis/releases/tag/v1.2.1) | 04-09-2023   | 2850 | Commands executed directly from a command line, perft in UCI mode |
| [1.2.0](https://github.com/Tearth/Inanis/releases/tag/v1.2.0) | 15-01-2023   | 2850 | Improved Syzygy support, general performance and stability improvement |
| [1.1.1](https://github.com/Tearth/Inanis/releases/tag/v1.1.1) | 14-08-2022   | 2800 | A bunch of fixes for reported issues, stability improvement |
| [1.1.0](https://github.com/Tearth/Inanis/releases/tag/v1.1.0) | 31-07-2022   | 2800 | Syzygy tablebases, MultiPV, adjusted evaluation |
| [1.0.1](https://github.com/Tearth/Inanis/releases/tag/v1.0.1) | 05-04-2022   | 2750 | A bunch of fixes for reported issues, stability improvement |
| [1.0.0](https://github.com/Tearth/Inanis/releases/tag/v1.0.0) | 02-04-2022   | 2750 | Initial release |

Each release contains a set of binaries for various platforms: Linux (x86, x86-64, ARM, AArch64) and Windows (x86, x86-64). Both Linux x86-64 and Windows x86-64 were also compiled with two additional instruction set variants: POPCNT and POPCNT + BMI1 + BMI2 - to get the best performance, please try to run the `benchmark` command using different engine's variants and choose the one which didn't return an error and has the most advanced instructions.

## Rating lists
 - [CCRL Blitz](https://computerchess.org.uk/ccrl/404/index.html)
 - [CCRL 40/15](https://computerchess.org.uk/ccrl/4040/index.html)
 - [CEGT 40/4](http://www.cegt.net/40_4_Ratinglist/40_4_single/rangliste.html)
 - [CEDR Blitz](https://chessengines.blogspot.com/p/rating-jcer.html)
 - [MCERL Blitz](https://www.chessengeria.eu/mcerl)
 - [UBC Bullet](https://e4e6.com/)

Big thanks to all testers and their effort! Please note that Elo on the lists above can slightly differ from the author's estimations due to different engine pools and the time control used for tests. 

## How to play online
Inanis has an official lichess account, where you can try to challenge the engine: https://lichess.org/@/InanisBot. Please note that ratings there are very understated and not comparable to CCRL ones. Accepts standard chess with a bullet, blitz, rapid and classical time control.

## UCI options
 - `Hash` *(default: 2 MB)* - a total size (in megabytes) for the transposition table and pawn hashtable
 - `Move Overhead` *(default: 100 ms)* - the amount of time (in milliseconds) that should be reserved during a search for some unexpected delays (like the slowness of GUI or network lags)
 - `MultiPV` *(default: 1 PV line)* - number of PV lines which should be displayed during search
 - `Threads` *(default: 1 thread)* - number of threads to use during search (should be less than a number of processor cores to get the best performance)
 - `SyzygyPath` *(default: &lt;empty&gt;)* - location of the optional Syzygy tablebases
 - `SyzygyProbeLimit` *(default: 8 pieces)* - maximal number of pieces for which the tablebase probe should be executed
 - `SyzygyProbeDepth` *(default: 6)* - minimal depth at which the tablebase probe should be executed
 - `Ponder` *(default: false)* - allows the engine to think during the opponent's time
 - `Crash Files` *(default: false)* - when enabled, saves crash messages in the ./crash directory
 - `Search Noise` *(default: false)* - when enabled, a small random noise is added to make search different every time
 - `Soft Nodes` *(default: false)* - when enabled, nodes limit is enforced only after search iteration is done instead of aborting it in the middle

## How to build
By default, calling `cargo build` or `cargo build --release` will build the engine without support for Syzygy tablebases (but still fully functional). To include it, please add `--features syzygy,bindgen` and make sure you have installed [clang](https://clang.llvm.org/) when working on Windows (MSVC doesn't support some C11 elements, so can't be used).

## Algorithms
 - **Board representation**: bitboards (a hybrid of make/undo scheme and storing data on stacks)
 - **Move generator**: staged (captures, quiet moves), magic bitboards, precalculated arrays for knight and king
 - **Move ordering**: hash move, good captures (SEE with support for x-ray attacks), killers, countermoves, castling and promotions, butterfly history, bad captures
 - **Search**: negamax, alpha-beta pruning, quiescence search, aspiration windows, null-move pruning, static null move pruning, razoring, late move reduction, late move pruning, lazy SMP, internal iterative reductions, check extensions
 - **Cache**: transposition table, pawn hashtable, history heuristic, killer heuristic, countermove heuristic
 - **Evaluation**: material, piece-square tables, pawn structure, pawn threats, mobility, king safety, tempo

## Tuner
Inanis has a built-in tuner, which allows for optimizing all evaluation parameters using a well-known [Texel's tuning method](https://www.chessprogramming.org/Texel%27s_Tuning_Method). As an output, there are Rust source files generated in a way that allows them to be directly pasted into the engine's source code. 

Example input file:
```
r2qkr2/p1pp1ppp/1pn1pn2/2P5/3Pb3/2N1P3/PP3PPP/R1B1KB1R b KQq - c9 "0-1";
r4rk1/3bppb1/p3q1p1/1p1p3p/2pPn3/P1P1PN1P/1PB1QPPB/1R3RK1 b - - c9 "1/2-1/2";
4Q3/8/8/8/6k1/4K2p/3N4/5q2 b - - c9 "0-1";
r4rk1/1Qpbq1bp/p1n2np1/3p1p2/3P1P2/P1NBPN1P/1P1B2P1/R4RK1 b - - c9 "0-1";
```

Examples of running the tuner:

 - `tuner ./input/quiet.epd ./output/ true 0.007 0.75 1` - run single-threaded tuning for positions stored in `quiet.epd`, starting from the random values, with scaling constant 0.007, WDL ratio 0.75 and saving the result in the `output` directory

 - `tuner ./input/quiet.epd ./output/ false None 1.0 4` - run tuning with 4 threads for positions stored in `quiet.epd`, starting from the values already set in the engine, with scaling constant determined before tuning, WDL ratio 1.0 and saving the result in the `output` directory

Since version 1.1.0, Inanis also has a command to generate epd files with quiet positions, based on provided PGN input:
 - `dataset ./input/games.pgn ./output/quiet.epd 16 250 50 3 0.5` -  generate a new `quiet.epd` file, by parsing `games.pgn` and taking 3 random positions from each of the game, ignoring these with a ply less than 16, evaluation score bigger than 250, and the difference between evaluation score and quiescence search score bigger than 50. The average game phase 0.5 means that the positions will be balanced (> 0.5 = near opening, < 0.5 = near ending)

## Test suites 
Testing of strategic evaluation performance can be done by using the `test` command, which performs a fixed-depth search for positions stored in the EPD file.

Example test suite file:
```
1k2r2r/1bq2p2/pn4p1/3pP3/pbpN1P1p/4QN1B/1P4PP/2RR3K b - - bm Nd7; c0 "Nd7=10, Bc5=8, Bc6=2, Be7=7"; id "STS: Knight Outposts/Repositioning/Centralization.001";
1q2bn2/6pk/2p1pr1p/2Q2p1P/1PP5/5N2/5PP1/4RBK1 w - - bm Ne5; c0 "Ne5=10, Nd4=8, Ra1=6, b5=9"; id "STS: Knight Outposts/Repositioning/Centralization.002";
1r1q1rk1/1b1n1p1p/p2b1np1/3pN3/3P1P2/P1N5/3BB1PP/1R1Q1RK1 b - - bm Ne4; c0 "Ne4=10, Bxa3=6, Nb6=6"; id "STS: Knight Outposts/Repositioning/Centralization.003";
1k2r2r/1bq2p2/pn4p1/3pP3/pbpN1P1p/4QN1B/1P4PP/2RR3K b - - bm Nd7; c0 "Nd7=10, Bc5=8, Bc6=2, Be7=7"; id "STS: Knight Outposts/Repositioning/Centralization.001";
1q2bn2/6pk/2p1pr1p/2Q2p1P/1PP5/5N2/5PP1/4RBK1 w - - bm Ne5; c0 "Ne5=10, Nd4=8, Ra1=6, b5=9"; id "STS: Knight Outposts/Repositioning/Centralization.002";
1r1q1rk1/1b1n1p1p/p2b1np1/3pN3/3P1P2/P1N5/3BB1PP/1R1Q1RK1 b - - bm Ne4; c0 "Ne4=10, Bxa3=6, Nb6=6"; id "STS: Knight Outposts/Repositioning/Centralization.003";
```

Examples of running the tests:

 - `testset ./input/STS1.epd 16 64 4` - run a fixed-depth (16 in this case) search for all positions stored in the `STS1.epd` file, using 64 MB transposition table and 4 threads. To classify the test as successful, the last iteration has to return the correct best move.

## Dependencies
**Build dependencies**
 - [cc](https://github.com/rust-lang/cc-rs) - a compilation of C sources into a Rust application
 - [bindgen](https://github.com/rust-lang/rust-bindgen) - generation of Rust FFI bindings to C libraries
 
**Dev dependencies**
 - [criterion](https://github.com/bheisler/criterion.rs) - statistics-driven benchmarking framework

**External libraries**
 - [Fathom](https://github.com/jdart1/Fathom) - support for Syzygy tablebases

## Contributing
Because Inanis is a pet project, pull requests are not currently accepted - this may or may not change in the future, depending on the way the project will go. However, feel free to make issues or suggestions, they are greatly appreciated. 

## Commands

All commands listed below can be executed both in interactive mode and directly from a command line, on example `inanis.exe perft 5`. Entries marked as [DEV] are available only when the engine is compiled with `dev` feature.

```
=== General ===
 benchmark - run test for a set of positions
 evaluate [fen] - show score for the position
 uci - run Universal Chess Interface
 quit - close the application

=== Development ===
 [DEV] dataset [pgn] [output] [min_ply] [max_score] [max_diff] [density] - dataset generator
 [DEV] magic - generate magic numbers
 [DEV] testset [epd] [depth] [ttable_size] [threads_count] - run test of positions
 [DEV] tuner [epd] [output] [randomize] [k] [wdl_ratio] [threads_count] - run tuning

=== Perft ===
 perft [depth]
 perft [depth] fen [fen]
 perft [depth] moves [moves]

=== Divided Perft ===
 dperft [depth]
 dperft [depth] fen [fen]
 dperft [depth] moves [moves]

=== Quick Perft ===
 qperft [depth] [threads_count] [hashtable_size_mb]
 qperft [depth] [threads_count] [hashtable_size_mb] fen [fen]
 qperft [depth] [threads_count] [hashtable_size_mb] moves [moves]
```