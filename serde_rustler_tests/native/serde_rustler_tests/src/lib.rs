//! Library implementing tests to be called from ExUnit.
//!
//! See `run_ser_test` and `run_de_test` for details about how to use `serde_rustler::Serializer` and `serde_rustler::Deserializer`.

#[macro_use]
extern crate rustler;

mod json;
mod test;
mod types;

use crate::types::Animal;
use rustler::{types::tuple, Encoder, Env, NifResult, SchedulerFlags, Term};
use serde_rustler::{atoms, from_term, to_term, Deserializer, Error, Serializer};
use std::error::Error as StdError;

rustler_export_nifs! {
    "Elixir.SerdeRustlerTests",
    [
        // json
        ("decode_json", 1, json::decode),
        ("decode_json_dirty", 1, json::decode, SchedulerFlags::DirtyCpu),
        ("encode_json_compact", 1, json::encode_compact),
        ("encode_json_compact_dirty", 1, json::encode_compact, SchedulerFlags::DirtyCpu),
        ("encode_json_pretty", 1, json::encode_pretty),
        ("encode_json_pretty_dirty", 1, json::encode_pretty, SchedulerFlags::DirtyCpu),

        // tests
        ("readme", 1, readme),
        ("test", 3, test::test),
        ("transcode", 1, transcode),
        ("transcode_dirty", 1, transcode, SchedulerFlags::DirtyCpu),
    ],
    None
}

/// Implements the README example.
#[inline]
pub fn readme<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let animal: Animal = from_term(args[0])?;
    println!("\n deserialized animal from README example: {:?}", animal);
    to_term(env, animal).map_err(|err| err.into())
}

/// Deserializes anything from an Elixir term and subsequently serializes the result abck to an Elixir term, returning it.
#[inline]
pub fn transcode<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    tag_tuple(env, || {
        let de = Deserializer::from(args[0]);
        let ser = Serializer::from(env);
        serde_transcode::transcode(de, ser)
    })
}

fn tag_tuple<'a, F>(env: Env<'a>, func: F) -> NifResult<Term<'a>>
where
    F: FnOnce() -> Result<Term<'a>, Error>,
{
    match func() {
        Ok(term) => Ok(ok_tuple(env, term)),
        Err(reason) => {
            let reason_term = reason.description().encode(env);
            Ok(error_tuple(env, reason_term))
        }
    }
}

fn ok_tuple<'a>(env: Env<'a>, term: Term<'a>) -> Term<'a> {
    let ok_atom_term = atoms::ok().encode(env);
    tuple::make_tuple(env, &vec![ok_atom_term, term])
}

fn error_tuple<'a>(env: Env<'a>, reason_term: Term<'a>) -> Term<'a> {
    let err_atom_term = atoms::error().encode(env);
    tuple::make_tuple(env, &vec![err_atom_term, reason_term])
}
