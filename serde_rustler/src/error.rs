use quick_error::quick_error;
use rustler::{Error as NifError, Term};
use serde::{de, ser};
use std::{error::Error as StdError, fmt::Display};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        DeserializationError(err: String) {
            description(err)
        }
        TypeHintsRequired {
            description("Cannot deserialize any, type hints are required")
        }
        InvalidAtom {
            description("Failed to deserialize atom")
        }
        InvalidBoolean {
            description("Failed to deserialize boolean")
        }
        InvalidNumber {
            description("Failed to deserialize number")
        }
        InvalidStringable {
            description("Failed to deserialize term as an &str")
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
        InvalidSequenceElement {
            description("Failed to deserialize sequence element")
        }

        ExpectedAtom {
            description("Expected to deserialize atom")
        }
        ExpectedBoolean {
            description("Expected to deserialize boolean")
        }
        ExpectedBinary {
            description("Expected to deserialize binary")
        }
        ExpectedNumber {
            description("Expected to deserialize number")
        }
        ExpectedChar {
            description("Expected to deserialize char")
        }
        ExpectedStringable {
            description("Expected to deserialize a UTF-8 stringable term")
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
        ExpectedEnum {
            description("Expected to deserialize enum")
        }
        ExpectedMap {
            description("Expected to deserialize map")
        }
        ExpectedStruct {
            description("Expected to deserialize struct")
        }
        ExpectedStructName {
            description("Expected to deserialize struct name")
        }
        ExpectedStructValue {
            description("Expected to deserialize struct value")
        }
        ExpectedUnitVariant {
            description("Expected to deserialize unit variant")
        }
        ExpectedNewtypeStruct {
            description("Expected to deserialize newtype struct tuple")
        }
        ExpectedNewtypeVariant {
            description("Expected to deserialize newtype variant")
        }
        ExpectedTupleVariant {
            description("Expected to deserialize tuple variant")
        }
        ExpectedStructVariant {
            description("Expected to deserialize struct variant")
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
            description("Failed to serialize struct key")
        }
    }
}

impl From<Error> for NifError {
    fn from(err: Error) -> NifError {
        NifError::RaiseTerm(Box::new(String::from(err.description())))
    }
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Error {
        Error::SerializationError(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Error {
        Error::DeserializationError(msg.to_string())
    }
}
