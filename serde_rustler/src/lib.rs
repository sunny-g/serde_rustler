//!
//!

#![recursion_limit = "100"]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rustler;
#[macro_use]
extern crate rustler_codegen;

pub mod atoms;
// pub mod de;
pub mod error;
pub mod ser;
