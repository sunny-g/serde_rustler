use crate::error::Error;
use rustler::{Encoder, Env, Term};

pub fn str_to_term<'a>(env: Env<'a>, string: &str) -> Result<Term<'a>, Error> {
    Ok(string.encode(env))
}

pub fn term_to_str<'a>(term: Term<'a>) -> Result<&'a str, Error> {
    term.decode().or(Err(Error::InvalidStringable))
}
