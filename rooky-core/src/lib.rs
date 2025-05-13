#![warn(
    clippy::all,
    clippy::missing_errors_doc,
    clippy::style,
    clippy::unseparated_literal_suffix,
    clippy::pedantic,
    clippy::nursery
)]

pub mod errors;
mod game;
pub mod headers;
pub mod idb;
pub mod openings;
pub mod pgn_standards;
pub use game::*;
