#![warn(
    clippy::all,
    clippy::missing_errors_doc,
    clippy::style,
    clippy::unseparated_literal_suffix
)]
#![allow(clippy::future_not_send, clippy::missing_docs_in_private_items)]

mod components;
pub mod contexts;
mod models;
mod pages;
mod router;

pub use components::*;
pub use contexts::*;
pub use models::*;
pub use pages::*;
pub use router::*;
