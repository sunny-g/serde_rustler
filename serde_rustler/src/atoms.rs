use crate::{error::Error, util};
use rustler::{types::atom::Atom, Encoder, Env, Term};

lazy_static! {
    pub static ref OK: String = String::from("Ok");
    pub static ref ERROR: String = String::from("Err");
}

rustler_atoms! {
    atom nil;
    atom ok;
    atom error;
    atom true_ = "true";
    atom false_ = "false";
    atom __struct__;
}

/**
 * Attempts to create an atom term from the provided string (if the atom already exists in the atom table). If not, returns a string term.
 */
pub fn str_to_term<'a>(env: Env<'a>, string: &str) -> Result<Term<'a>, Error> {
    if string == "Ok" {
        Ok(ok().encode(env))
    } else if string == "Err" {
        Ok(error().encode(env))
    } else {
        match Atom::try_from_bytes(env, string.as_bytes()) {
            Ok(Some(term)) => Ok(term.encode(env)),
            Ok(None) => util::str_to_term(env, string).or(Err(Error::InvalidStringable)),
            _ => Err(Error::InvalidStringable),
        }
    }
}

/**
 * Attempts to create a `String` from the term.
 */
pub fn term_to_str<'a>(term: Term<'a>) -> Result<String, Error> {
    if ok().eq(&term) {
        Ok(OK.to_string())
    } else if error().eq(&term) {
        Ok(ERROR.to_string())
    } else if term.is_atom() {
        term.atom_to_string().or(Err(Error::InvalidAtom))
    } else {
        let string: String = util::term_to_str(term)
            .or(Err(Error::InvalidStringable))?
            .to_string();
        Ok(string)
    }
}
