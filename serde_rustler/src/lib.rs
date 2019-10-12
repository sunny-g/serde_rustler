//! Serde support for [rustler](https://docs.rs/rustler).

#![recursion_limit = "196"]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rustler;

pub mod atoms;
mod de;
mod error;
mod ser;
mod util;

pub use de::{from_term, Deserializer};
pub use error::Error;
pub use ser::{to_term, Serializer};
