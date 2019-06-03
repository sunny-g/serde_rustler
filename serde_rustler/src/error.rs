use crate::atoms;
use quick_error::quick_error;
use rustler::{types::tuple, Encoder, Env, Term};
use serde::{de, ser};
use std::fmt::Display;

pub fn error_tuple<'a>(env: Env<'a>, reason_term: Term<'a>) -> Term<'a> {
    let err_term = atoms::error().encode(env);
    tuple::make_tuple(env, &vec![err_term, reason_term])
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        DeserializationError(err: String) {
            description(err)
        }
        TypeHintsRequired {
            description("Cannot deserialize any, type hints are required")
        }
        InvalidBoolean {
            description("Failed to deserialize boolean")
        }
        InvalidNumber {
            description("Failed to deserialize number")
        }
        InvalidString {
            description("Failed to deserialize string")
        }
        InvalidBinary {
            description("Failed to deserialize binary")
        }
        InvalidList {
            description("Failed to deserialize list")
        }
        InvalidTuple {
            description("Failed to deserialize tuple")
        }
        ExpectedAtom {
            description("Expected to deserialize atom")
        }
        ExpectedBoolean {
            description("Expected to deserialize boolean")
        }
        ExpectedNumber {
            description("Expected to deserialize number")
        }
        ExpectedChar {
            description("Expected to deserialize char")
        }
        ExpectedNil {
            description("Expected to deserialize nil")
        }
        ExpectedList {
            description("Expected to deserialize list")
        }
        ExpectedTuple {
            description("Expected to deserialize tuple")
        }
        ExpectedMap {
            description("Expected to deserialize map")
        }
        ExpectedStruct {
            description("Expected to deserialize struct")
        }

        SerializationError(err: String) {
            description(err)
        }
        InvalidVariantName {
            description("Failed to serialize variant to atom or string")
        }
        InvalidStructName {
            description("Failed to serialize struct name to atom or string")
        }
        InvalidMap {
            description("Failed to serialize map to NIF map")
        }
        InvalidStruct {
            description("Failed to serialize struct to NIF struct")
        }
        InvalidStructKey {
            description("Failed to serialized struct key")
        }
    }
}

impl ser::Error for Error {
    // #[cold]
    fn custom<T: Display>(msg: T) -> Error {
        Error::SerializationError(msg.to_string())
    }
}

impl de::Error for Error {
    // #[cold]
    fn custom<T: Display>(msg: T) -> Error {
        Error::DeserializationError(msg.to_string())
    }
}
