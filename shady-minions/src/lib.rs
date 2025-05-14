#![warn(
    clippy::all,
    clippy::missing_errors_doc,
    clippy::style,
    clippy::unseparated_literal_suffix,
    clippy::pedantic,
    clippy::nursery
)]

mod button;
mod card;
mod checkbox;
mod drawer;
mod dropdown;
mod form;
mod input;
mod label;
mod modal;
mod popover;
mod select;
mod slider;
mod switch;
mod tabs;
mod textarea;

pub mod ui {
    pub use super::button::*;
    pub use super::card::*;
    pub use super::checkbox::*;
    pub use super::drawer::*;
    pub use super::dropdown::*;
    pub use super::form::*;
    pub use super::input::*;
    pub use super::label::*;
    pub use super::modal::*;
    pub use super::popover::*;
    pub use super::select::*;
    pub use super::slider::*;
    pub use super::switch::*;
    pub use super::tabs::*;
    pub use super::textarea::*;
}
