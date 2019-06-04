//!
//!

#[macro_use]
extern crate rustler;
extern crate serde_rustler;

mod types;

rustler_export_nifs! {
    "Elixir.SerdeRustlerTests",
    [("test", 3, test)],
    None
}

use crate::types::{
    NewtypeStruct, NewtypeVariant, Struct, StructVariant, TupleStruct, TupleVariant, Unit,
    UnitVariant,
};
use rustler::{Encoder, Env, NifResult, Term};
use serde::{Deserialize, Serialize};
use serde_rustler::{atoms, de::Deserializer, error::error_tuple, ser::Serializer};
use std::{collections::HashMap, error::Error as StdError};

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

        // Numbers
        "i8 (min)" => run_test!(i8::min_value()),
        "i8 (max)" => run_test!(i8::max_value()),
        "i16 (min)" => run_test!(i16::min_value()),
        "i16 (max)" => run_test!(i16::max_value()),
        "i32 (min)" => run_test!(i32::min_value()),
        "i32 (max)" => run_test!(i32::max_value()),
        "i64 (min)" => run_test!(i64::min_value()),
        "i64 (max)" => run_test!(i64::max_value()),
        // "i128 (min)" => run_test!(i128::min_value()),
        // "i128 (max)" => run_test!(i128::max_value()),
        "u8 (min)" => run_test!(u8::min_value()),
        "u8 (max)" => run_test!(u8::max_value()),
        "u16 (min)" => run_test!(u16::min_value()),
        "u16 (max)" => run_test!(u16::max_value()),
        "u32 (min)" => run_test!(u32::min_value()),
        "u32 (max)" => run_test!(u32::max_value()),
        "u64 (min)" => run_test!(u64::min_value()),
        "u64 (max)" => run_test!(u64::max_value()),
        // "u128 (min)" => run_test!(u128::min_value()),
        // "u128 (max)" => run_test!(u128::max_value()),

        // Strings and Binaries
        "char (empty)" => run_test!(0 as u8 as char),
        "str (empty)" => run_test!(""),
        "str" => run_test!("hello world"),
        "bytes" => run_test!([3, 2, 1, 0]),

        // Unit Types
        "unit" => run_test!(()),
        "unit struct" => run_test!(Unit {}),
        "unit variant" => run_test!(UnitVariant::A),

        // Newtype Types
        "newtype struct" => run_test!(NewtypeStruct::new(u8::max_value())),
        "newtype variant" => run_test!(NewtypeVariant::N(u8::max_value())),
        "newtype variant (ok)" => {
            let ok: Result<u8, String> = Ok(u8::max_value());
            run_test!(ok)
        }
        "newtype variant (error)" => {
            let err: Result<u8, String> = Err(String::from("error reason"));
            run_test!(err)
        }

        // Sequences
        "sequences (primitive)" => run_test!(vec!["hello", "world"]),
        "sequences (complex)" => {
            let a = NewtypeStruct::new(u8::min_value());
            let b = NewtypeStruct::new(u8::max_value());
            run_test!(vec![a, b])
        }

        // Tuple Types
        "tuple (empty)" => run_test!(()), // same as unit
        "tuple" => run_test!((0, 255)),
        "tuple struct" => run_test!(TupleStruct::new(0, 128, 255)),
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
            map.insert("key", Struct::new(0, 0, 0));
            map.insert("val", Struct::new(255, 255, 255));

            run_test!(map)
        }
        "struct" => run_test!(Struct::new(0, 128, 255)),
        "struct variant" => run_test!(StructVariant::S {
            r: 0,
            g: 128,
            b: 255
        }),
        _ => Ok(error_tuple(env, "nonexistant test".encode(env))),
    }
}

enum TestResult<'a> {
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

/// Serializes a known value, and if the resulting term is equal to the expected term, return `:ok`. Else, return `{:error, actual_term}`.
fn run_ser_test<'a, T>(env: Env<'a>, actual: &T, expected_term: Term<'a>) -> TestResult<'a>
where
    T: PartialEq + Serialize,
{
    let ser = Serializer::from(env);
    match actual.serialize(ser) {
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

/// Deserializes the expected term, and if the resulting native type is equal to the actual value, return `:ok`. Else, return `:error`.
fn run_de_test<'a, T>(env: Env<'a>, actual: &T, expected_term: Term<'a>) -> TestResult<'a>
where
    T: PartialEq + Deserialize<'a>,
{
    let de = Deserializer::from(expected_term);
    match T::deserialize(de) {
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
