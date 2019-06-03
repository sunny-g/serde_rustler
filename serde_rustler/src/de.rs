use super::{atoms, error::Error};
use rustler::{
    dynamic,
    types::{tuple, Binary, ListIterator, MapIterator},
    Decoder, Term, TermType,
};
use serde::de::{
    self, Deserialize, DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor,
};
use std::iter;

/**
 *
 */
pub struct Deserializer<'a> {
    term: Term<'a>,
}

impl<'a> From<Term<'a>> for Deserializer<'a> {
    fn from(term: Term<'a>) -> Deserializer<'a> {
        Deserializer { term }
    }
}

impl<'a> Deserializer<'a> {
    fn parse_number<T: Decoder<'a>>(&self) -> Result<T, Error> {
        if !self.term.is_number() {
            return Err(Error::InvalidNumber);
        }

        self.term.decode().map_err(|_| Error::InvalidNumber)
    }

    // TODO: Binary or OwnedBinary?
    fn parse_binary(&'a self) -> Result<&'a [u8], Error> {
        let binary: Binary = self.term.decode().map_err(|_| Error::InvalidBinary)?;
        Ok(binary.as_slice())
    }

    // TODO: test if Binary?
    fn parse_string<T: Decoder<'a>>(&self) -> Result<T, Error> {
        self.term.decode().map_err(|_| Error::InvalidString)
    }

    fn get_type(&self) -> TermType {
        dynamic::get_type(self.term)
    }
}

impl<'de, 'a: 'de> de::Deserializer<'de> for Deserializer<'a> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self.get_type() {
            // bool (t, f), unit (nil), unit struct (nil), unit variant (atom)
            TermType::Atom => Err(Error::TypeHintsRequired),
            // i8, i16, i32, i64, u8, u16, u32, u64, f32, f64 (i128, u128)
            TermType::Number => Err(Error::TypeHintsRequired),
            // char, string, byte array
            TermType::Binary => Err(Error::TypeHintsRequired),
            // seq
            TermType::List => Err(Error::TypeHintsRequired),
            // map, struct
            TermType::Map => Err(Error::TypeHintsRequired),
            // newtype variant, tuple, tuple struct, tuple variant, struct variant
            TermType::Tuple => Err(Error::TypeHintsRequired),
            _ => Err(Error::TypeHintsRequired),
        }

    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if atoms::nil().eq(&self.term) {
            visitor.visit_unit()
        } else {
            Err(Error::ExpectedNil)
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if atoms::true_().eq(&self.term) {
            visitor.visit_bool(true)
        } else if atoms::false_().eq(&self.term) {
            visitor.visit_bool(false)
        } else {
            Err(Error::InvalidBoolean)
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.get_type() {
            TermType::Atom => {
                if atoms::nil().eq(&self.term) {
                    visitor.visit_none()
                } else {
                    visitor.visit_some(self)
                }
            }
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.parse_number()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.parse_number()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.parse_number()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.parse_number()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.parse_number()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.parse_number()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.parse_number()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.parse_number()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(self.parse_number()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(self.parse_number()?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.parse_string() {
            Err(_) => Err(Error::ExpectedChar),
            res => {
                // TODO: char vs string?
                let string: String = res.unwrap();
                if string.len() == 1 {
                    visitor.visit_char(string.chars().next().unwrap())
                } else {
                    Err(Error::ExpectedChar)
                }
            }
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(self.parse_string()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.parse_string()?)
    }

    // TODO: Binary or OwnedBinary?
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bytes(self.parse_binary()?)
    }

    // TODO: Binary or OwnedBinary?
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bytes(self.parse_binary()?)
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    // Deserialization of compound types like sequences and maps happens by
    // passing the visitor an "Access" object that gives it the ability to
    // iterate through the data contained in the sequence.
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if !self.term.is_list() {
            return Err(Error::ExpectedList);
        }

        let iter: ListIterator = self.term.decode().map_err(|_| Error::ExpectedList)?;
        visitor.visit_seq(SequenceDeserializer::new(iter))
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if !self.term.is_tuple() {
            return Err(Error::ExpectedTuple);
        }

        let tuple = tuple::get_tuple(self.term).map_err(|_| Error::ExpectedTuple)?;
        if tuple.len() != len {
            return Err(Error::InvalidTuple);
        }

        visitor.visit_seq(SequenceDeserializer::new(tuple.into_iter()))
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if !self.term.is_map() {
            return Err(Error::ExpectedMap);
        }

        let iter = MapIterator::new(self.term).ok_or(Error::ExpectedMap)?;
        visitor.visit_map(MapDeserializer::new(iter))
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if !self.term.is_map() {
            return Err(Error::ExpectedMap);
        }

        let __struct__ = atoms::__struct__().to_term(self.term.get_env());
        let struct_name = self
            .term
            .map_get(__struct__)
            .map_err(|_| Error::ExpectedStruct)?;
        let name_term = atoms::term_from_str(self.term.get_env(), name)?;

        if !struct_name.eq(name_term) {
            return Err(Error::ExpectedStruct);
        }
        self.deserialize_map(visitor)
    }

    // could be...?
    //  - unit variant      [enum val]          (atom)
    //  - newtype variant   [enum var]          (tuple(var, T))
    //  - tuple variant     [enum var tuple]    (tuple(var, tuple(<T>))
    //  - struct variant    [enum var struct]   (tuple(var, struct))
    // TODO: assert that the variant atom term is in variants
    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.term.get_type() {
            TermType::Atom => {
                if atoms::nil().eq(&self.term) {
                    return visitor.visit_unit();
                }

                let atom_name = self
                    .term
                    .atom_to_string()
                    .map_err(|_| Error::ExpectedAtom)?;
                visitor.visit_enum(atom_name.into())
            }
            TermType::Tuple => {
                // TODO: support strings, or remove string option from `term_from_bytes` and `term_from_str`
                Err(Error::ExpectedAtom)
            }
            _ => Err(Error::ExpectedAtom),
        }
        // match self.term {
        //     Term::Atom(atom) => {
        //         // We have a unit variant.
        //         visitor.visit_enum(atom.name.to_camel_case().into_deserializer())
        //     }
        //     Term::Tuple(tuple) => match tuple.elements.as_slice() {
        //         [variant_term, value_term] => {
        //             visitor.visit_enum(EnumDeserializer::new(&variant_term, &value_term))
        //         }
        //         _ => return Err(Error::MisSizedVariantTuple),
        //     },
        //     _ => Err(Error::ExpectedAtomOrTuple),
        // }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.term {
            Term::Atom(atom) => visitor.visit_string(atom.name.clone()),
            _ => Err(Error::ExpectedAtom),
        }
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // Just skip over this by calling visit_unit.
        visitor.visit_unit()
    }
}

pub struct SequenceDeserializer<'a, I>
where
    I: Iterator,
{
    iter: iter::Fuse<I>,
}

impl<'a, I> SequenceDeserializer<'a, I>
where
    I: Iterator,
{
    fn new(iter: I) -> Self {
        SequenceDeserializer { iter: iter.fuse() }
    }
}

impl<'de, 'a: 'de, I> SeqAccess<'de> for SequenceDeserializer<'a, I>
where
    I: Iterator<Item = Term<'a>>,
{
    type Error = Error;

    fn next_element_seed<V>(&mut self, seed: V) -> Result<Option<V::Value>, Error>
    where
        V: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            None => Ok(None),
            Some(term) => seed.deserialize(term.into()).map(Some),
        }
    }
}

pub struct MapDeserializer<'a, I>
where
    I: Iterator,
{
    iter: iter::Fuse<I>,
    current_value: Option<Term<'a>>,
}

impl<'a, I> MapDeserializer<'a, I>
where
    I: Iterator,
{
    fn new(iter: I) -> Self {
        MapDeserializer {
            iter: iter.fuse(),
            current_value: None,
        }
    }
}

impl<'de, 'a: 'de, I> MapAccess<'de> for MapDeserializer<'a, I>
where
    I: Iterator<Item = (Term<'a>, Term<'a>)>,
{
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Error>
    where
        K: DeserializeSeed<'de>,
    {
        if let Some(_) = self.current_value {
            panic!("MapDeserializer.next_key_seed was called twice in a row")
        }

        match self.iter.next() {
            None => Ok(None),
            Some(pair) => {
                let (key, value) = pair;
                self.current_value = Some(value);
                seed.deserialize(key.into()).map(Some)
            }
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<Option<V::Value>, Error>
    where
        V: DeserializeSeed<'de>,
    {
        match self.current_value {
            None => Ok(None),
            Some(value) => {
                self.current_value = None;
                seed.deserialize(value.into()).map(Some)
            }
        }
    }
}

pub struct EnumDeserializer<'a> {
    variant: Term<'a>,
    term: Term<'a>,
}

impl<'a> EnumDeserializer<'a> {
    fn new(variant: Term<'a>, term: Term<'a>) -> Self {
        EnumDeserializer { variant, term }
    }
}

impl<'de> EnumAccess<'de> for EnumDeserializer<'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Error>
    where
        V: DeserializeSeed<'de>,
    {
        Err(Error::InvalidBinary)
    }
}

impl<'de> VariantAccess<'de> for EnumDeserializer<'de> {
    type Error = Error;

    // TODO: If the `Visitor` expected this variant to be a unit variant, the input
    // should have been the plain string case handled in `deserialize_enum`.
    fn unit_variant(self) -> Result<(), Error> {
        Err(Error::ExpectedAtom)
    }

    // TODO: Newtype variants are represented in JSON as `{ NAME: VALUE }` so
    // deserialize the value here.
    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Error>
    where
        T: DeserializeSeed<'de>,
    {
        Err(Error::ExpectedAtom)
        // seed.deserialize(Deserializer::from_term(self.term))
    }

    // TODO: Tuple variants are represented in JSON as `{ NAME: [DATA...] }` so
    // deserialize the sequence of data here.
    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::ExpectedAtom)
        // let deserializer = Deserializer::from_term(self.term);
        // de::Deserializer::deserialize_tuple(deserializer, len, visitor)
    }

    // TODO: Struct variants are represented in JSON as `{ NAME: { K: V, ... } }` so
    // deserialize the inner map here.
    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::ExpectedAtom)
        // let deserializer = Deserializer::from_term(self.term);
        // de::Deserializer::deserialize_map(deserializer, visitor)
    }
}

struct VariantNameDeserializer<'a> {
    term: Term<'a>,
}

impl<'a> VariantNameDeserializer<'a> {
    pub fn from_term(term: Term<'a>) -> Self {
        VariantNameDeserializer { term }
    }
}

impl<'de, 'a: 'de> de::Deserializer<'de> for VariantNameDeserializer<'a> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self.term.get_type() {
            TermType::Atom => Err(Error::InvalidBinary),
            TermType::Binary => Err(Error::InvalidBinary),
            _ => Err(Error::InvalidBinary),
        }

        // match self.term {
        //     Term::Atom(atom) => visitor.visit_string(atom.name.to_camel_case()),
        //     _ => Err(Error::ExpectedAtom),
        // }
    }

    // forward_to_deserialize_any! {
    //     bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
    //         bytes byte_buf option unit unit_struct newtype_struct seq tuple
    //         tuple_struct map struct enum identifier ignored_any
    // }
}
