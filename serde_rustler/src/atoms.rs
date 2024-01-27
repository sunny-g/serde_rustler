//! Constants and utilities for conversion between Rust string-likes and Elixir atoms.

use crate::Error;
use rustler::{types::atom::Atom, Encoder, Env, Term};

lazy_static::lazy_static! {
    pub static ref OK: String = String::from("Ok");
    pub static ref ERROR: String = String::from("Err");
}

rustler::atoms! {
    /// The atom `nil`.
    nil,

    /// The atom `:ok`.
    ok,

    /// The atom `:error`.
    error,

    /// The atom/Boolean `true`.
    true_ = "true",

    /// The atom/Boolean `false`.
    false_ = "false",

    /// The atom `:__struct__`.
    __struct__,
}

/**
 * Attempts to create an atom term from the provided string (if the atom already exists in the atom table). If not, returns a string term.
 */
pub fn str_to_term<'a>(env: &Env<'a>, string: &str) -> Result<Term<'a>, Error> {
    if string == "Ok" {
        Ok(ok().encode(*env))
    } else if string == "Err" {
        Ok(error().encode(*env))
    } else {
        match Atom::try_from_bytes(*env, string.as_bytes()) {
            Ok(Some(term)) => Ok(term.encode(*env)),
            Ok(None) => Err(Error::InvalidStringable),
            _ => Err(Error::InvalidStringable),
        }
    }
}

/**
 * Attempts to create a `String` from the term.
 */
pub fn term_to_string(term: &Term) -> Result<String, Error> {
    if ok().eq(term) {
        Ok(OK.to_string())
    } else if error().eq(term) {
        Ok(ERROR.to_string())
    } else if term.is_atom() {
        term.atom_to_string().or(Err(Error::InvalidAtom))
    } else {
        Err(Error::InvalidStringable)
    }
}
