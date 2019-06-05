//!
//!

#![recursion_limit = "196"]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rustler;
extern crate rustler_codegen;

pub mod atoms;
pub mod de;
pub mod error;
pub mod ser;
mod util;
