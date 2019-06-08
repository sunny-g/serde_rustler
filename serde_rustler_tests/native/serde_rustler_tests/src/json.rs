use rustler::{Env, Error as NifError, NifResult, Term};
use serde_bytes::Bytes;
use serde_rustler::{from_term, to_term, Deserializer, Serializer};
use serde_transcode::transcode;

#[inline]
/// Deserializes a JSON string into an Elixir term.
pub fn decode<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let json_bytes: &[u8] = from_term(args[0])?;
    let mut de = serde_json::Deserializer::from_slice(json_bytes);
    let ser = Serializer::from(env);
    transcode(&mut de, ser).map_err(|err| err.into())
}

#[inline]
/// Serializes an Elixir term into a compact JSON string.
pub fn encode_compact<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let de = Deserializer::from(args[0]);
    let mut ser_vec = Vec::new();
    let mut ser = serde_json::Serializer::new(&mut ser_vec);
    transcode(de, &mut ser).or(Err(NifError::RaiseAtom("transcode error")))?;
    to_term(env, Bytes::new(&ser_vec)).map_err(|err| err.into())
}

#[inline]
/// Serializes an Elixir term into a pretty-printed JSON string.
pub fn encode_pretty<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let de = Deserializer::from(args[0]);
    let mut ser_vec = Vec::new();
    let mut ser = serde_json::Serializer::pretty(&mut ser_vec);
    transcode(de, &mut ser).or(Err(NifError::RaiseAtom("transcode error")))?;
    to_term(env, Bytes::new(&ser_vec)).map_err(|err| err.into())
}
