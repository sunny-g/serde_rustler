//! Library implementing tests to be called from ExUnit.
//!
//! See `run_ser_test` and `run_de_test` for details about how to use `serde_rustler::Serializer` and `serde_rustler::Deserializer`.

#[macro_use]
extern crate rustler;

mod types;

rustler_export_nifs! {
    "Elixir.SerdeRustlerTests",
    [   ("readme", 1, readme),
        ("test", 3, test),
        ("transcode", 1, transcode),
    ],
    None
}

use crate::types::{
    Animal, NewtypeStruct, NewtypeVariant, Struct, StructVariant, TupleStruct, TupleVariant, Unit,
    UnitVariant,
};
use rustler::{types::tuple, Encoder, Env, NifResult, Term};
use serde::{Deserialize, Serialize};
use serde_bytes::Bytes;
use serde_rustler::{atoms, from_term, to_term, Deserializer, Error, Serializer};
use std::{collections::HashMap, error::Error as StdError};

/// Implements the README example.
#[inline]
pub fn readme<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    tag_tuple(env, || {
        let animal: Animal = from_term(args[0])?;
        println!("\n deserialized animal from README: {:?}", animal);
        to_term(env, animal)
    })
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

/// Serializes or deserializes a known Elixir term to/from a known Rust value, asserts that the resulting is equivalent to known term/value.
#[inline]
pub fn test<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let test_type: &str = args[0].decode()?;
    let test_name: &str = args[1].decode()?;
    let expected_term = args[2];

    macro_rules! run_test {
        ($actual:expr) => {
            run_test(env, test_type, $actual, expected_term)
        };
    }

    match test_name {
        "none" => run_test!(None as Option<u8>),
        "some" => run_test!(Some(100)),
        "true" => run_test!(true),
        "false" => run_test!(false),

        // Signed Integers
        "i8 (min)" => run_test!(i8::min_value()),
        "i8 (0)" => run_test!(0 as i8),
        "i8 (max)" => run_test!(i8::max_value()),
        "i16 (min)" => run_test!(i16::min_value()),
        "i16 (0)" => run_test!(0 as i16),
        "i16 (max)" => run_test!(i16::max_value()),
        "i32 (min)" => run_test!(i32::min_value()),
        "i32 (0)" => run_test!(0 as i32),
        "i32 (max)" => run_test!(i32::max_value()),
        "i64 (min)" => run_test!(i64::min_value()),
        "i64 (0)" => run_test!(0 as i64),
        "i64 (max)" => run_test!(i64::max_value()),
        "i128 (min)" => run_test!(i128::min_value()),
        "i128 (0)" => run_test!(0 as i128),
        "i128 (max)" => run_test!(i128::max_value()),

        // Unsigned Integers
        "u8 (min)" => run_test!(u8::min_value()),
        "u8 (max)" => run_test!(u8::max_value()),
        "u16 (min)" => run_test!(u16::min_value()),
        "u16 (max)" => run_test!(u16::max_value()),
        "u32 (min)" => run_test!(u32::min_value()),
        "u32 (max)" => run_test!(u32::max_value()),
        "u64 (min)" => run_test!(u64::min_value()),
        "u64 (max)" => run_test!(u64::max_value()),
        "u128 (min)" => run_test!(u128::min_value()),
        "u128 (max)" => run_test!(u128::max_value()),

        // Float32
        "f32 (0)" => run_test!(f32::from_bits(0x00000000)),
        "f32 (-0)" => run_test!(f32::from_bits(0x80000000)),
        "f32 (one)" => run_test!(f32::from_bits(0x3f800000)),
        "f32 (smallest subnormal)" => run_test!(f32::from_bits(0x00000001)),
        "f32 (largest subnormal)" => run_test!(f32::from_bits(0x007fffff)),
        "f32 (smallest normal)" => run_test!(f32::from_bits(0x00800000)),
        "f32 (largest normal)" => run_test!(f32::from_bits(0x7f7fffff)),
        "f32 (smallest number < 1)" => run_test!(f32::from_bits(0x3f800001)),
        "f32 (largest number < 1)" => run_test!(f32::from_bits(0x3f7fffff)),
        // "f32 (infinity)" => run_test!(f32::from_bits(0x7f800000)),
        // "f32 (-infinity)" => run_test!(f32::from_bits(0xff800000)),

        // Float64
        "f64 (0)" => run_test!(f64::from_bits(0x0000000000000000)),
        "f64 (-0)" => run_test!(f64::from_bits(0x8000000000000000)),
        "f64 (one)" => run_test!(f64::from_bits(0x3f80000000000000)),
        "f64 (smallest subnormal)" => run_test!(f64::from_bits(0x0000000000000001)),
        "f64 (largest subnormal)" => run_test!(f64::from_bits(0x007fffffffffffff)),
        "f64 (smallest normal)" => run_test!(f64::from_bits(0x0080000000000000)),
        "f64 (largest normal)" => run_test!(f64::from_bits(0x7f7fffffffffffff)),
        "f64 (smallest number < 1)" => run_test!(f64::from_bits(0x3f80000000000001)),
        "f64 (largest number < 1)" => run_test!(f64::from_bits(0x3f7fffffffffffff)),
        // "f64 (infinity)" => run_test!(f64::from_bits(0x7f80000000000000)),
        // "f64 (-infinity)" => run_test!(f64::from_bits(0xff80000000000000)),

        // Chars, Strings and Binaries
        "char (empty)" => run_test!(0 as u8 as char),
        "str (empty)" => run_test!(""),
        "str" => run_test!("hello world"),
        "bytes" => run_test!(Bytes::new(&[3, 2, 1, 0])),

        // Unit Types
        "unit" => run_test!(()),
        "unit struct" => run_test!(Unit {}),
        "unit variant" => run_test!(UnitVariant::A),

        // Newtype Types
        "newtype struct" => run_test!(NewtypeStruct(u8::max_value())),
        "newtype variant" => run_test!(NewtypeVariant::N(u8::max_value())),
        "newtype variant (ok tuple)" => {
            let ok: Result<u8, String> = Ok(u8::max_value());
            run_test!(ok)
        }
        "newtype variant (error tuple)" => {
            let err: Result<u8, String> = Err(String::from("error reason"));
            run_test!(err)
        }

        // Sequences
        "sequences (empty)" => run_test!(Vec::new() as Vec<u8>),
        "sequences (primitive)" => run_test!(vec!["hello", "world"]),
        "sequences (complex)" => {
            let a = NewtypeStruct(u8::min_value());
            let b = NewtypeStruct(u8::max_value());
            run_test!(vec![a, b])
        }

        // Tuple Types
        "tuple (empty)" => run_test!(()), // same as unit
        "tuple" => run_test!((0, 255)),
        "tuple struct" => run_test!(TupleStruct(0, 128, 255)),
        "tuple variant" => run_test!(TupleVariant::T(0, 255)),

        // Map and Struct Types
        "map (primitive)" => {
            let mut map = HashMap::new();
            map.insert("key", "hello");
            map.insert("val", "world");

            run_test!(map)
        }
        "map (complex)" => {
            let mut map = HashMap::new();
            map.insert("key", Struct { r: 0, g: 0, b: 0 });
            map.insert(
                "val",
                Struct {
                    r: 255,
                    g: 255,
                    b: 255,
                },
            );

            run_test!(map)
        }
        "struct" => run_test!(Struct {
            r: 0,
            g: 128,
            b: 255
        }),
        "struct variant" => run_test!(StructVariant::S {
            r: 0,
            g: 128,
            b: 255
        }),
        _ => Ok(error_tuple(env, "nonexistant test".encode(env))),
    }
}

pub enum TestResult<'a> {
    Ok,
    Err(Term<'a>),
}

fn run_test<'a, T>(
    env: Env<'a>,
    test_type: &str,
    actual: T,
    expected_term: Term<'a>,
) -> NifResult<Term<'a>>
where
    T: PartialEq + Serialize + Deserialize<'a>,
{
    let test_res = match test_type {
        "serialize" => run_ser_test(env, &actual, expected_term),
        "deserialize" => run_de_test(env, &actual, expected_term),
        _ => TestResult::Err(error_tuple(env, "nonexistant test".encode(env))),
    };

    match test_res {
        TestResult::Err(error_term) => Ok(error_term),
        TestResult::Ok => Ok(atoms::ok().encode(env)),
    }
}

/// Serializes a known Rust value, and asserts that the resulting Elixir term is equal to the expected term. Returns `:ok` or `{:error, actual_term}`.
pub fn run_ser_test<'a, T>(env: Env<'a>, actual: &T, expected_term: Term<'a>) -> TestResult<'a>
where
    T: PartialEq + Serialize,
{
    match to_term(env, actual) {
        Err(reason) => {
            let reason_term = reason.description().encode(env);
            TestResult::Err(error_tuple(env, reason_term))
        }
        Ok(actual_term) => {
            if expected_term.eq(&actual_term) {
                TestResult::Ok
            } else {
                TestResult::Err(error_tuple(env, actual_term))
            }
        }
    }
}

/// Deserializes the expected Elixir term, and asserts that the resulting Rust value is equal to the actual value. Returns `:ok` or `{:error, err.description}`.
pub fn run_de_test<'a, T>(env: Env<'a>, actual: &T, expected_term: Term<'a>) -> TestResult<'a>
where
    T: PartialEq + Deserialize<'a>,
{
    match from_term(expected_term) {
        Err(reason) => {
            let reason_term = reason.description().encode(env);
            TestResult::Err(error_tuple(env, reason_term))
        }
        Ok(expected) => {
            if actual.eq(&expected) {
                TestResult::Ok
            } else {
                TestResult::Err(atoms::error().encode(env))
            }
        }
    }
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
