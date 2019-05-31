//! when we want to deserialize a binary
//!     - we pass in the term, decode it to get the underlying binary
//!     - serde deserializes it into the serde data model, returning rust data
//!     - *** *** map rust data to elixir term
//!         - by calling `encode(env)` to return a Term
//!         - (aka implementing a Serializer for serde -> elixir term)
//!         - (or Deserialize )
//! when we want to serialize an elixir type
//!     - we pass in the terms
//!     - *** *** map elixir term to serde data model
//!         - by calling `decode()` on them and casting them to a type
//!         - (aka implementing a Deserializer for elixir term -> serde)
//!         - (or Serialize?)
//!     - then serde returns a binary
//!
//! name of the game (for Serialize):
//!     - study `Term.decode` to find out each term's associated native T
//!     - associate each T with a `serialize_` method
//!
//! name of the game (for Serializer):
//!     - maps data model into the output representation
//!     - aka, maps serde types to Rustler Terms
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
