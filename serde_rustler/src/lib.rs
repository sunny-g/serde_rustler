//!
//!

#![recursion_limit = "196"]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rustler;
extern crate rustler_codegen;

pub mod atoms;
mod de;
pub mod error;
mod ser;
pub mod util;

pub use de::Deserializer;
pub use ser::Serializer;
