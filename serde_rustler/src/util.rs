use crate::{atoms, error::Error};
use rustler::{types::tuple, Encoder, Env, Term};

pub fn str_to_term<'a>(env: Env<'a>, string: &str) -> Result<Term<'a>, Error> {
    Ok(string.encode(env))
}

pub fn term_to_str<'a>(term: Term<'a>) -> Result<&'a str, Error> {
    term.decode().or(Err(Error::InvalidStringable))
}

pub fn error_tuple<'a>(env: Env<'a>, reason_term: Term<'a>) -> Term<'a> {
    let err_term = atoms::error().encode(env);
    tuple::make_tuple(env, &vec![err_term, reason_term])
}
