#![warn(
    clippy::all,
    clippy::missing_errors_doc,
    clippy::style,
    clippy::unseparated_literal_suffix,
    clippy::pedantic,
    clippy::nursery
)]

mod lichess;
pub use lichess::*;
mod chesscom;
pub use chesscom::*;
