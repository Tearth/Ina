#![allow(
    clippy::needless_range_loop,
    clippy::identity_op,
    clippy::let_and_return,
    clippy::uninit_assumed_init,
    clippy::nonminimal_bool,
    clippy::collapsible_if
)]

#[macro_use]
extern crate bitflags;

pub mod bit;
pub mod board;
pub mod common;
pub mod fen;
pub mod movegen;
pub mod movescan;
pub mod patterns;
pub mod perft;
pub mod terminal;
pub mod zobrist;
