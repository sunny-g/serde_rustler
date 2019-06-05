use crate::{atoms, error::Error, util};
use rustler::{
    types::{tuple, Binary, ListIterator, MapIterator},
    Decoder, Term, TermType,
};
use serde::{
    de::{
        self, Deserialize, DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess,
        Visitor,
    },
    forward_to_deserialize_any,
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

impl<'de, 'a: 'de> de::Deserializer<'de> for Deserializer<'a> {
    type Error = Error;

    // TODO
    // TODO: perhaps get rid of this
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self.term.get_type() {
            // unit (nil)
            // bool (t, f)
            // unit struct (nil)
            // unit variant (atom)
            TermType::Atom => {
                if atoms::nil().eq(&self.term) {
                    self.deserialize_unit(visitor)
                } else if atoms::true_().eq(&self.term) || atoms::false_().eq(&self.term) {
                    self.deserialize_bool(visitor)
                } else {
                    let string = atoms::term_to_str(self.term).or(Err(Error::ExpectedAtom))?;
                    visitor.visit_string(string)
                }
            }
            // i8, i16, i32, i64, u8, u16, u32, u64, f32, f64 (i128, u128)
            // TODO: blanket cast to isize or usize?
            TermType::Number => Err(Error::TypeHintsRequired),
            // char
            // string
            // byte array
            TermType::Binary => Err(Error::TypeHintsRequired),
            // seq
            TermType::List => Err(Error::TypeHintsRequired),
            // map
            // struct
            // struct variant
            TermType::Map => Err(Error::TypeHintsRequired),
            // newtype struct
            // newtype variant (atom, len 2)
            // tuple (any len)
            // tuple struct (atom, len 3+)
            // tuple variant (atom, len 3+)
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
            Err(Error::ExpectedBoolean)
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if atoms::nil().eq(&self.term) {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(parse_number(self.term)?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(parse_number(self.term)?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(parse_number(self.term)?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(parse_number(self.term)?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(parse_number(self.term)?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(parse_number(self.term)?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(parse_number(self.term)?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(parse_number(self.term)?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(parse_number(self.term)?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(parse_number(self.term)?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        validate_binary(self.term)?;
        util::term_to_str(self.term)
            .or(Err(Error::ExpectedChar))
            .and_then(|string| {
                if string.len() == 1 {
                    visitor.visit_char(string.chars().next().unwrap())
                } else {
                    Err(Error::ExpectedChar)
                }
            })
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        validate_binary(self.term)?;
        let string = util::term_to_str(self.term)?;
        visitor.visit_borrowed_str(string)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_bytes(parse_binary(self.term)?)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_bytes(parse_binary(self.term)?)
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
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let tuple = validate_tuple(self.term, Some(2))?;
        let name_term =
            atoms::str_to_term(self.term.get_env(), name).or(Err(Error::ExpectedStructName))?;

        if tuple[0].ne(&name_term) {
            return Err(Error::InvalidStructName);
        }

        visitor.visit_newtype_struct(Deserializer::from(tuple[1]))
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

        let iter: ListIterator = self.term.decode().or(Err(Error::ExpectedList))?;
        visitor.visit_seq(SequenceDeserializer::new(iter))
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let tuple = validate_tuple(self.term, Some(len))?;
        visitor.visit_seq(SequenceDeserializer::new(tuple.into_iter()))
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let mut tuple = validate_tuple(self.term, Some(len + 1))?;
        let name_term =
            atoms::str_to_term(self.term.get_env(), name).or(Err(Error::ExpectedStructName))?;

        if tuple[0].ne(&name_term) {
            return Err(Error::InvalidStructName);
        }

        let iter = tuple.split_off(1).into_iter();
        visitor.visit_seq(SequenceDeserializer::new(iter))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if !self.term.is_map() {
            return Err(Error::ExpectedMap);
        }

        let iter = MapIterator::new(self.term).ok_or(Error::ExpectedMap)?;
        visitor.visit_map(MapDeserializer::new(iter, false))
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
        validate_struct(self.term, Some(name))?;

        let iter = MapIterator::new(self.term).ok_or(Error::ExpectedStruct)?;
        visitor.visit_map(MapDeserializer::new(iter, true))
    }

    // could be...?
    //  - unit variant      [enum var]              (atom)
    //  - newtype variant   [enum var + val]        (tuple(var, T))
    //  - tuple variant     [enum var + vals...]    (tuple(var, vals...))
    //  - struct variant    [enum var + struct]     (struct(var, vals...))
    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let variant: Option<(EnumDeserializerType, Term<'a>)> = match self.term.get_type() {
            // unit variant
            TermType::Atom => Some((EnumDeserializerType::Unit, self.term)),
            TermType::Binary => Some((EnumDeserializerType::Unit, self.term)),
            TermType::Number => Some((EnumDeserializerType::Unit, self.term)),
            // newtype or tuple variant
            TermType::Tuple => {
                let tuple = validate_tuple(self.term, None)?;
                match tuple.len() {
                    0 | 1 => None,
                    2 => Some((EnumDeserializerType::Newtype, tuple[0])),
                    _ => Some((EnumDeserializerType::Tuple, tuple[0])),
                }
            }
            // struct variant
            TermType::Map => Some((
                EnumDeserializerType::Struct,
                validate_struct(self.term, None)?,
            )),
            _ => None,
        };

        variant.ok_or(Error::ExpectedEnum).and_then(|variant| {
            let (var_type, var_term) = variant;
            let enum_de = EnumDeserializer::new(var_type, var_term, variants, Some(self.term))?;
            visitor.visit_enum(enum_de)
        })
    }

    // TODO: is this right?
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.term.get_type() {
            TermType::Atom => self.deserialize_str(visitor),
            TermType::Binary => self.deserialize_str(visitor),
            TermType::Number => self.deserialize_i64(visitor),
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

struct SequenceDeserializer<'a, I>
where
    I: Iterator<Item = Term<'a>>,
{
    iter: iter::Fuse<I>,
}

impl<'a, I> SequenceDeserializer<'a, I>
where
    I: Iterator<Item = Term<'a>>,
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
            Some(term) => seed.deserialize(Deserializer::from(term)).map(Some),
        }
    }
}

struct MapDeserializer<'a, I>
where
    I: Iterator,
{
    is_struct: bool,
    iter: iter::Fuse<I>,
    current_value: Option<Term<'a>>,
}

impl<'a, I> MapDeserializer<'a, I>
where
    I: Iterator,
{
    fn new(iter: I, is_struct: bool) -> Self {
        MapDeserializer {
            is_struct,
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

        self.iter
            .next()
            .map_or(None, |pair| {
                let (key, _value) = pair;

                if atoms::__struct__().eq(&key) {
                    self.iter.next()
                } else {
                    Some(pair)
                }
            })
            .map_or(Ok(None), |pair| {
                let (key, value) = pair;
                self.current_value = Some(value);

                if self.is_struct {
                    seed.deserialize(VariantNameDeserializer::from(key))
                        .map(Some)
                } else {
                    seed.deserialize(Deserializer::from(key)).map(Some)
                }
            })
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Error>
    where
        V: DeserializeSeed<'de>,
    {
        match self.current_value {
            None => Err(Error::ExpectedStructValue),
            Some(value) => {
                self.current_value = None;
                seed.deserialize(Deserializer::from(value))
            }
        }
    }
}

enum EnumDeserializerType {
    Unit,
    Newtype,
    Tuple,
    Struct,
}

struct EnumDeserializer<'a> {
    variant_type: EnumDeserializerType,
    variant_term: Term<'a>,
    variant: String,
    term: Option<Term<'a>>,
}

impl<'a> EnumDeserializer<'a> {
    fn new(
        variant_type: EnumDeserializerType,
        variant_term: Term<'a>,
        variants: &'static [&'static str],
        term: Option<Term<'a>>,
    ) -> Result<Self, Error> {
        let var_de = VariantNameDeserializer::from(variant_term);
        let variant: String = String::deserialize(var_de).or(Err(Error::InvalidVariantName))?;

        if variants.contains(&variant.as_str()) {
            Ok(EnumDeserializer {
                variant_type,
                variant_term,
                variant,
                term,
            })
        } else {
            Err(Error::InvalidVariantName)
        }
    }
}

impl<'de, 'a: 'de> EnumAccess<'de> for EnumDeserializer<'a> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Error>
    where
        V: DeserializeSeed<'de>,
    {
        let var_de = VariantNameDeserializer::from(self.variant_term);
        let val = seed.deserialize(var_de)?;
        Ok((val, self))
    }
}

impl<'de, 'a: 'de> VariantAccess<'de> for EnumDeserializer<'a> {
    type Error = Error;

    // is an atom or string
    fn unit_variant(self) -> Result<(), Error> {
        match self.variant_type {
            EnumDeserializerType::Unit => Ok(()),
            _ => Err(Error::ExpectedUnitVariant),
        }
    }

    // is a tagged (len 2) tuple (starting with atom or string)
    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.variant_type {
            EnumDeserializerType::Newtype => {
                if let Some(term) = self.term {
                    let tuple = validate_tuple(term, Some(2))?;
                    seed.deserialize(Deserializer::from(tuple[1]))
                } else {
                    Err(Error::ExpectedNewtypeVariant)
                }
            }
            _ => Err(Error::ExpectedNewtypeVariant),
        }
    }

    // is a tagged (len 3+) tuple (starting with atom or string)
    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self.variant_type {
            EnumDeserializerType::Tuple => {
                if let Some(term) = self.term {
                    let mut tuple = validate_tuple(term, Some(len + 1))?;
                    let iter = tuple.split_off(1).into_iter();
                    visitor.visit_seq(SequenceDeserializer::new(iter))
                } else {
                    Err(Error::ExpectedTupleVariant)
                }
            }
            _ => Err(Error::ExpectedTupleVariant),
        }
    }

    // is a struct, with atom or string struct_name
    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self.variant_type {
            EnumDeserializerType::Struct => {
                if let Some(term) = self.term {
                    validate_struct(term, Some(&self.variant))?;
                    let iter = MapIterator::new(term).ok_or(Error::ExpectedStruct)?;
                    visitor.visit_map(MapDeserializer::new(iter, true))
                } else {
                    Err(Error::ExpectedStructVariant)
                }
            }
            _ => Err(Error::ExpectedStructVariant),
        }
    }
}

struct VariantNameDeserializer<'a> {
    variant: Term<'a>,
}

impl<'a> From<Term<'a>> for VariantNameDeserializer<'a> {
    fn from(variant: Term<'a>) -> Self {
        VariantNameDeserializer { variant }
    }
}

impl<'de, 'a: 'de> de::Deserializer<'de> for VariantNameDeserializer<'a> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        let variant = match self.variant.get_type() {
            TermType::Atom => {
                let string = atoms::term_to_str(self.variant).or(Err(Error::InvalidVariantName))?;
                Ok(string)
            }
            TermType::Binary => Ok(String::from(util::term_to_str(self.variant)?)),
            TermType::Number => Ok(String::from(util::term_to_str(self.variant)?)),
            _ => Err(Error::ExpectedStringable),
        }?;

        visitor.visit_string(variant)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
            bytes byte_buf option unit unit_struct newtype_struct seq tuple
            tuple_struct map struct enum identifier ignored_any
    }
}

// TODO: perhaps use boundary checking to deserialize any number?
fn parse_number<'a, T: Decoder<'a>>(term: Term<'a>) -> Result<T, Error> {
    if !term.is_number() {
        return Err(Error::InvalidNumber);
    }

    term.decode().or(Err(Error::ExpectedNumber))
}

// TODO: Binary or OwnedBinary?
fn parse_binary<'a>(term: Term<'a>) -> Result<&'a [u8], Error> {
    validate_binary(term)?;
    let binary: Binary = term.decode().or(Err(Error::ExpectedBinary))?;
    Ok(binary.as_slice())
}

fn validate_binary<'a>(term: Term<'a>) -> Result<(), Error> {
    if !term.is_binary() {
        Err(Error::ExpectedBinary)
    } else {
        Ok(())
    }
}

fn validate_tuple<'a>(term: Term<'a>, len: Option<usize>) -> Result<Vec<Term<'a>>, Error> {
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

fn validate_struct<'a>(term: Term<'a>, name: Option<&str>) -> Result<Term<'a>, Error> {
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
