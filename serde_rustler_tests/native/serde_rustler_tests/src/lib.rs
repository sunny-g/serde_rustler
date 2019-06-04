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

    match test_name {
        "none" => run_test(env, test_type, None as Option<u8>, expected_term),
        "some" => run_test(env, test_type, Some(100), expected_term),
        "true" => run_test(env, test_type, true, expected_term),
        "false" => run_test(env, test_type, false, expected_term),

        // Numbers
        "i8 (min)" => run_test(env, test_type, i8::min_value(), expected_term),
        "i8 (max)" => run_test(env, test_type, i8::max_value(), expected_term),
        "i16 (min)" => run_test(env, test_type, i16::min_value(), expected_term),
        "i16 (max)" => run_test(env, test_type, i16::max_value(), expected_term),
        "i32 (min)" => run_test(env, test_type, i32::min_value(), expected_term),
        "i32 (max)" => run_test(env, test_type, i32::max_value(), expected_term),
        "i64 (min)" => run_test(env, test_type, i64::min_value(), expected_term),
        "i64 (max)" => run_test(env, test_type, i64::max_value(), expected_term),
        // "i128 (min)" => run_test(env, test_type, i128::min_value(), expected_term),
        // "i128 (max)" => run_test(env, test_type, i128::max_value(), expected_term),
        "u8 (min)" => run_test(env, test_type, u8::min_value(), expected_term),
        "u8 (max)" => run_test(env, test_type, u8::max_value(), expected_term),
        "u16 (min)" => run_test(env, test_type, u16::min_value(), expected_term),
        "u16 (max)" => run_test(env, test_type, u16::max_value(), expected_term),
        "u32 (min)" => run_test(env, test_type, u32::min_value(), expected_term),
        "u32 (max)" => run_test(env, test_type, u32::max_value(), expected_term),
        "u64 (min)" => run_test(env, test_type, u64::min_value(), expected_term),
        "u64 (max)" => run_test(env, test_type, u64::max_value(), expected_term),
        // "u128 (min)" => run_test(env, test_type, u128::min_value(), expected_term),
        // "u128 (max)" => run_test(env, test_type, u128::max_value(), expected_term),

        // Strings and Binaries
        "char (empty)" => run_test(env, test_type, 0 as u8 as char, expected_term),
        "str (empty)" => run_test(env, test_type, "", expected_term),
        "str" => run_test(env, test_type, "hello world", expected_term),
        "bytes" => run_test(env, test_type, [3, 2, 1, 0], expected_term),

        // Unit Types
        "unit" => run_test(env, test_type, (), expected_term),
        "unit struct" => run_test(env, test_type, Unit {}, expected_term),
        "unit variant" => run_test(env, test_type, UnitVariant::A, expected_term),

        // Newtype Types
        "newtype struct" => run_test(
            env,
            test_type,
            NewtypeStruct::new(u8::max_value()),
            expected_term,
        ),
        "newtype variant" => run_test(
            env,
            test_type,
            NewtypeVariant::N(u8::max_value()),
            expected_term,
        ),
        "newtype variant (ok)" => {
            let ok: Result<u8, String> = Ok(u8::max_value());
            run_test(env, test_type, ok, expected_term)
        }
        "newtype variant (error)" => {
            let err: Result<u8, String> = Err(String::from("error reason"));
            run_test(env, test_type, err, expected_term)
        }

        // Sequences
        "sequences (primitive)" => run_test(env, test_type, vec!["hello", "world"], expected_term),
        "sequences (complex)" => {
            let a = NewtypeStruct::new(u8::min_value());
            let b = NewtypeStruct::new(u8::max_value());
            run_test(env, test_type, vec![a, b], expected_term)
        }

        // Tuple Types
        "tuple (empty)" => run_test(env, test_type, (), expected_term), // same as unit
        "tuple" => run_test(env, test_type, (0, 255), expected_term),
        "tuple struct" => run_test(env, test_type, TupleStruct::new(0, 128, 255), expected_term),
        "tuple variant" => run_test(env, test_type, TupleVariant::T(0, 255), expected_term),

        // Map and Struct Types
        "map (primitive)" => {
            let mut map = HashMap::new();
            map.insert("key", "hello");
            map.insert("val", "world");

            run_test(env, test_type, map, expected_term)
        }
        "map (complex)" => {
            let mut map = HashMap::new();
            map.insert("key", Struct::new(0, 0, 0));
            map.insert("val", Struct::new(255, 255, 255));

            run_test(env, test_type, map, expected_term)
        }
        "struct" => run_test(env, test_type, Struct::new(0, 128, 255), expected_term),
        "struct variant" => run_test(
            env,
            test_type,
            StructVariant::S {
                r: 0,
                g: 128,
                b: 255,
            },
            expected_term,
        ),
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
    T: Eq + Serialize + Deserialize<'a>,
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
    T: Eq + Serialize + Deserialize<'a>,
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

/// Deserializes the expected term, and if the resulting native type is equal to the expected term, return `:ok`. Else, return `:error`.
fn run_de_test<'a, T>(env: Env<'a>, actual: &T, expected_term: Term<'a>) -> TestResult<'a>
where
    T: Eq + Serialize + Deserialize<'a>,
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
