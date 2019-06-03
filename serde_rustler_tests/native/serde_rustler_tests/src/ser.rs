use crate::types::{Millimeters, NewtypeVariant, Rgb, Struct, StructVariant, Unit, UnitVariant};
use rustler::{Encoder, Env, NifResult, Term};
use serde::Serialize;
use serde_rustler::{atoms, error::error_tuple, ser::Serializer};
use std::{collections::HashMap, error::Error as StdError};

pub fn test<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let test_name: String = args[0].decode()?;
    let expected_term = args[1];

    match test_name.as_ref() {
        "none" => run_test(None as Option<u8>, env, expected_term),
        "some" => run_test(Some(100), env, expected_term),
        "true" => run_test(true, env, expected_term),
        "false" => run_test(false, env, expected_term),

        // Numbers
        "i8 (min)" => run_test(i8::min_value(), env, expected_term),
        "i8 (max)" => run_test(i8::max_value(), env, expected_term),
        "i16 (min)" => run_test(i16::min_value(), env, expected_term),
        "i16 (max)" => run_test(i16::max_value(), env, expected_term),
        "i32 (min)" => run_test(i32::min_value(), env, expected_term),
        "i32 (max)" => run_test(i32::max_value(), env, expected_term),
        "i64 (min)" => run_test(i64::min_value(), env, expected_term),
        "i64 (max)" => run_test(i64::max_value(), env, expected_term),
        // "i128 (min)" => run_test(i128::min_value(), env, expected_term),
        // "i128 (max)" => run_test(i128::max_value(), env, expected_term),
        "u8 (min)" => run_test(u8::min_value(), env, expected_term),
        "u8 (max)" => run_test(u8::max_value(), env, expected_term),
        "u16 (min)" => run_test(u16::min_value(), env, expected_term),
        "u16 (max)" => run_test(u16::max_value(), env, expected_term),
        "u32 (min)" => run_test(u32::min_value(), env, expected_term),
        "u32 (max)" => run_test(u32::max_value(), env, expected_term),
        "u64 (min)" => run_test(u64::min_value(), env, expected_term),
        "u64 (max)" => run_test(u64::max_value(), env, expected_term),
        // "u128 (min)" => run_test(u128::min_value(), env, expected_term),
        // "u128 (max)" => run_test(u128::max_value(), env, expected_term),

        // Strings and Binaries
        "char (empty)" => run_test(0 as u8 as char, env, expected_term),
        "str (empty)" => run_test("", env, expected_term),
        "str" => run_test("hello world", env, expected_term),
        "bytes" => run_test(&[3, 2, 1, 0], env, expected_term),

        // Unit Types
        "unit" => run_test(None as Option<()>, env, expected_term),
        "unit struct" => run_test(Unit {}, env, expected_term),
        "unit variant" => run_test(UnitVariant::A, env, expected_term),

        // Newtype Types
        "newtype struct" => run_test(Millimeters::new(u8::max_value()), env, expected_term),
        "newtype variant" => run_test(NewtypeVariant::N(u8::max_value()), env, expected_term),
        "newtype variant (ok)" => {
            let ok: Result<u8, String> = Result::Ok(u8::max_value());
            run_test(ok, env, expected_term)
        }
        "newtype variant (error)" => {
            let err: Result<u8, String> = Result::Err(String::from("error reason"));
            run_test(err, env, expected_term)
        }

        // Sequences
        "sequences (primitive)" => run_test(vec!["hello", "world"], env, expected_term),
        "sequences (complex)" => {
            let a = Millimeters::new(u8::min_value());
            let b = Millimeters::new(u8::max_value());
            run_test(vec![a, b], env, expected_term)
        }

        // Tuple Types
        "tuple (empty)" => run_test((), env, expected_term),
        "tuple" => run_test((0, 255), env, expected_term),
        "tuple struct" => run_test(Rgb::new(0, 128, 255), env, expected_term),

        // Map and Struct Types
        "map (simple)" => {
            let mut map = HashMap::new();
            map.insert("key", "hello");
            map.insert("val", "world");

            run_test(map, env, expected_term)
        }
        "map (complex)" => {
            let mut map = HashMap::new();
            map.insert("key", Struct::new(0, 0, 0));
            map.insert("val", Struct::new(255, 255, 255));

            run_test(map, env, expected_term)
        }
        "struct" => run_test(Struct::new(0, 128, 255), env, expected_term),
        "struct variant" => run_test(
            StructVariant::S {
                r: 0,
                g: 128,
                b: 255,
            },
            env,
            expected_term,
        ),
        _ => Ok(error_tuple(env, "nonexistant_test".encode(env))),
    }
}

/// Serializes a known value, and if the resulting term is equal to the expected term, return `:ok`. Else, return `{:error, actual}`.
fn run_test<'a, T>(val: T, env: Env<'a>, expected_term: Term<'a>) -> NifResult<Term<'a>>
where
    T: Serialize,
{
    let ser = Serializer::from(env);
    match val.serialize(ser) {
        Err(reason) => {
            let reason_term = reason.description().encode(env);
            Ok(error_tuple(env, reason_term))
        }
        Ok(actual_term) => {
            if expected_term.eq(&actual_term) {
                Ok(atoms::ok().encode(env))
            } else {
                Ok(error_tuple(env, actual_term))
            }
        }
    }
}
