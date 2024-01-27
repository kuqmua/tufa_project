#![deny(
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::float_arithmetic
)]
#![allow(clippy::too_many_arguments)]

pub mod bot;
pub mod global_variables;
pub mod helpers;

fn main() {
    crate::bot::start_bot();
}
