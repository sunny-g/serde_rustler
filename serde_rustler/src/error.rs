use std::fmt::Display;

use rustler::Error as NifError;
use serde::{de, ser};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    DeserializationError(String),

    #[error("Cannot deserialize any, type hints are required")]
    TypeHintsRequired,

    #[error("Failed to deserialize atom")]
    InvalidAtom,

    #[error("Failed to deserialize boolean")]
    InvalidBoolean,

    #[error("Failed to deserialize number")]
    InvalidNumber,

    #[error("Failed to deserialize term as an &str")]
    InvalidStringable,

    #[error("Failed to deserialize list")]
    InvalidList,

    #[error("Failed to deserialize tuple")]
    InvalidTuple,

    #[error("Failed to deserialize sequence element")]
    InvalidSequenceElement,

    #[error("Expected to deserialize atom")]
    ExpectedAtom,

    #[error("Expected to deserialize boolean")]
    ExpectedBoolean,

    #[error("Expected to deserialize binary")]
    ExpectedBinary,

    #[error("Expected to deserialize number")]
    ExpectedNumber,

    #[error("Expected to deserialize char")]
    ExpectedChar,

    #[error("Expected to deserialize a UTF-8 stringable term")]
    ExpectedStringable,

    #[error("Expected to deserialize nil")]
    ExpectedNil,

    #[error("Expected to deserialize list")]
    ExpectedList,

    #[error("Expected to deserialize tuple")]
    ExpectedTuple,

    #[error("Expected to deserialize enum")]
    ExpectedEnum,

    #[error("Expected to deserialize map")]
    ExpectedMap,

    #[error("Expected to deserialize struct")]
    ExpectedStruct,

    #[error("Expected to deserialize struct name")]
    ExpectedStructName,

    #[error("Expected to deserialize struct value")]
    ExpectedStructValue,

    #[error("Expected to deserialize unit variant")]
    ExpectedUnitVariant,

    #[error("Expected to deserialize newtype struct tuple")]
    ExpectedNewtypeStruct,

    #[error("Expected to deserialize new type variant")]
    ExpectedNewtypeVariant,

    #[error("Expected to deserialize tuple variant")]
    ExpectedTupleVariant,

    #[error("Expected to deserialize struct variant")]
    ExpectedStructVariant,

    #[error("{0}")]
    SerializationError(String),

    #[error("Failed to serialize variant to atom or string")]
    InvalidVariantName,

    #[error("Failed to serialize struct name to atom or string")]
    InvalidStructName,

    #[error("Failed to serialize variant to binary")]
    InvalidBinary,

    #[error("Failed to serialize NIF map")]
    InvalidMap,

    #[error("Failed to serialize struct to NIF struct")]
    InvalidStruct,

    #[error("Failed to serialize struct key")]
    InvalidStructKey,
}

impl From<Error> for NifError {
    fn from(err: Error) -> NifError {
        NifError::RaiseTerm(Box::new(err.to_string()))
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
