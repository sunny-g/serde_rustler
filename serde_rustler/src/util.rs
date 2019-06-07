use crate::{atoms, Error};
use rustler::{types::tuple, Binary, Decoder, Encoder, Env, Term};

#[inline]
pub fn str_to_term<'a>(env: Env<'a>, string: &str) -> Result<Term<'a>, Error> {
    atoms::str_to_term(env, string).or(Ok(string.encode(env)))
}

#[inline]
pub fn term_to_str(term: Term) -> Result<String, Error> {
    atoms::term_to_str(term)
        .or(term.decode())
        .or(Err(Error::ExpectedStringable))
}

pub fn parse_number<'a, T: Decoder<'a>>(term: Term<'a>) -> Result<T, Error> {
    if !term.is_number() {
        return Err(Error::InvalidNumber);
    }

    term.decode().or(Err(Error::ExpectedNumber))
}

pub fn parse_binary(term: Term) -> Result<&[u8], Error> {
    validate_binary(term)?;
    let binary: Binary = term.decode().or(Err(Error::ExpectedBinary))?;
    Ok(binary.as_slice())
}

pub fn validate_binary(term: Term) -> Result<(), Error> {
    if !term.is_binary() {
        Err(Error::ExpectedBinary)
    } else {
        Ok(())
    }
}

pub fn validate_tuple(term: Term, len: Option<usize>) -> Result<Vec<Term>, Error> {
    if !term.is_tuple() {
        return Err(Error::ExpectedTuple);
    }

    let tuple = tuple::get_tuple(term).or(Err(Error::ExpectedTuple))?;
    match len {
        None => Ok(tuple),
        Some(len) => {
            if tuple.len() == len {
                Ok(tuple)
            } else {
                Err(Error::InvalidTuple)
            }
        }
    }
}

pub fn validate_struct<'a>(term: Term<'a>, name: Option<&str>) -> Result<Term<'a>, Error> {
    if !term.is_map() {
        return Err(Error::ExpectedMap);
    }

    let __struct__ = atoms::__struct__().to_term(term.get_env());
    let struct_name_term = term.map_get(__struct__).or(Err(Error::ExpectedStruct))?;

    if let Some(name) = name {
        let name_term =
            atoms::str_to_term(term.get_env(), name).or(Err(Error::InvalidStructName))?;

        if struct_name_term.ne(&name_term) {
            return Err(Error::ExpectedStruct);
        }
    }

    Ok(struct_name_term)
}
