// notcurses::lib
//
//! A rusty, high level notcurses wrapper.
//

#![warn(clippy::all)]
#![allow(
    clippy::float_arithmetic,
    clippy::implicit_return,
    clippy::needless_return,
    clippy::blanket_clippy_restriction_lints,
    clippy::pattern_type_mismatch
)]

/// Reexport of [`libnotcurses-sys`](https://crates.io/crates/libnotcurses-sys).
///
/// ---
///
#[doc(inline)]
pub use libnotcurses_sys as sys;
pub use sys::NcAlign as Align;

mod error;
mod notcurses;
mod plane;
mod visual;

pub use self::notcurses::{Capabilities, Notcurses};
pub use error::{Error, Result};
pub use plane::{Plane, PlaneBuilder};
pub use visual::Visual;
