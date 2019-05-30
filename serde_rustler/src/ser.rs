use super::{atoms, error::Error};
use rustler::{types::tuple, Encoder, Env, Term};
use serde::ser::{self, Serialize};

/**
 *
 */
pub struct Serializer<'a> {
    env: Env<'a>,
}

impl<'a> From<Env<'a>> for Serializer<'a> {
    fn from(env: Env<'a>) -> Serializer<'a> {
        Serializer { env }
    }
}

impl<'a> Serializer<'a> {
    fn to_term<T>(&'a self, value: &T) -> Result<Term<'a>, Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }
}

impl<'a> ser::Serializer for &'a Serializer<'a> {
    type Ok = Term<'a>;
    type Error = Error;

    type SerializeSeq = SequenceSerializer<'a>;
    type SerializeTuple = SequenceSerializer<'a>;
    type SerializeTupleStruct = SequenceSerializer<'a>;
    type SerializeTupleVariant = SequenceSerializer<'a>;
    type SerializeMap = MapSerializer<'a>;
    type SerializeStruct = MapSerializer<'a>;
    type SerializeStructVariant = MapSerializer<'a>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(v.encode(self.env))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(self)
    }
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(v.encode(self.env))
    }
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(v.encode(self.env))
    }
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(v.encode(self.env))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(v.encode(self.env))
    }
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(v.encode(self.env))
    }
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(v.encode(self.env))
    }
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(v.encode(self.env))
    }
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(v.encode(self.env))
    }
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(v.encode(self.env))
    }
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(v.encode(self.env))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(v.encode(self.env))
    }

    // TODO
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        unimplemented!("return Binary or OwnedBinary?");
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(atoms::nil().encode(self.env))
    }

    /// Serializes `struct Unit` as `nil`.
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    /// Serializes `E::A` or `E::B` in `enum E { A, B }` as `:A` or `:B`.
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        atoms::term_from_str(self.env, variant).map_err(|_| Error::InvalidVariant)
    }

    /// Serializes `struct Millimeters(u8)` as the raw `u8` value.
    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(self)
    }

    /// Serializes `E::N` in `enum E { N(u8) }` as `{:N, u8}` or `{"N", u8}`, depending on if the atom `:N` has already been created.
    /// Serializes `Result::Ok` of `enum Result { Ok(u8) }` into `{:ok, u8}`.
    // TODO: ensure `Ok` maps to `:ok`, `Err` to `:error`
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        let mut tuple = SequenceSerializer::new(&self, Some(2), None);
        let variant_term =
            atoms::term_from_str(self.env, variant).map_err(|_| Error::InvalidVariant)?;
        tuple.add(variant_term);
        tuple.add(value.serialize(self)?);
        tuple.to_tuple()
    }

    /// Serializes sequences as a Elixir lists.
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SequenceSerializer::new(&self, len, None))
    }

    /// Serializes tuples as Elixir tuples.
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(SequenceSerializer::new(&self, Some(len), None))
    }

    /// Serializes `struct Rgb(u8, u8, u8)` as `{u8, u8, u8}`.
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_tuple(len)
    }

    /// Serializes `E::T` of `enum E { T(u8, u8) }` as `{:T, {u8, u8}} or {"T", {u8, u8}}`, depending on if the atom `:T` has already been created.
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        let variant_term =
            atoms::term_from_str(self.env, variant).map_err(|_| Error::InvalidVariant)?;
        Ok(SequenceSerializer::new(
            &self,
            Some(len),
            Some(variant_term),
        ))
    }

    /// Serializes map as Elixir map.
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(MapSerializer::new(self, len, None, None))
    }

    // Serializes as map, but attempts to include %{:__struct__ => :STRUCT_NAME}, if the atom `:STRUCT_NAME` has already been created.
    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        let name_term =
            atoms::term_from_str(self.env, name).map_err(|_| Error::InvalidStructName)?;
        Ok(MapSerializer::new(self, Some(len), Some(name_term), None))
    }

    // Serializes `E::S` of `enum E { S { r: u8, g: u8, b: u8 } }` into `{:E, %S{...}}`, attempting to replace `E` and `S` with `:E` and `:S` if the atoms have already been created.
    // TODO: above is incorrect, fix it
    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        let name_term =
            atoms::term_from_str(self.env, name).map_err(|_| Error::InvalidStructName)?;
        let variant_term =
            atoms::term_from_str(self.env, variant).map_err(|_| Error::InvalidVariant)?;
        Ok(MapSerializer::new(
            self,
            Some(len),
            Some(name_term),
            Some(variant_term),
        ))
    }
}

impl<'a> ser::SerializeSeq for SequenceSerializer<'a> {
    type Ok = Term<'a>;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        self.add(value.serialize(self.ser)?);
        Ok(())
    }

    fn end(self) -> Result<Term<'a>, Error> {
        self.to_list()
    }
}

impl<'a> ser::SerializeTuple for SequenceSerializer<'a> {
    type Ok = Term<'a>;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        self.add(value.serialize(self.ser)?);
        Ok(())
    }

    fn end(self) -> Result<Term<'a>, Error> {
        self.to_tuple()
    }
}

impl<'a> ser::SerializeTupleStruct for SequenceSerializer<'a> {
    type Ok = Term<'a>;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        self.add(value.serialize(self.ser)?);
        Ok(())
    }

    fn end(self) -> Result<Term<'a>, Error> {
        self.to_tuple()
    }
}

impl<'a> ser::SerializeTupleVariant for SequenceSerializer<'a> {
    type Ok = Term<'a>;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        self.add(value.serialize(self.ser)?);
        Ok(())
    }

    fn end(self) -> Result<Term<'a>, Error> {
        self.to_tuple_variant()
    }
}

impl<'a> ser::SerializeMap for MapSerializer<'a> {
    type Ok = Term<'a>;
    type Error = Error;

    fn serialize_key<T>(&mut self, _value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        panic!("Not Implemented")
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        panic!("Not Implemented")
    }

    fn serialize_entry<K: ?Sized, V: ?Sized>(&mut self, key: &K, value: &V) -> Result<(), Error>
    where
        K: Serialize,
        V: Serialize,
    {
        self.add_key(key.serialize(self.ser)?);
        self.add_val(value.serialize(self.ser)?);
        Ok(())
    }

    fn end(self) -> Result<Term<'a>, Error> {
        self.to_map()
    }
}

impl<'a> ser::SerializeStruct for MapSerializer<'a> {
    type Ok = Term<'a>;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        self.add_key(key.serialize(self.ser)?);
        self.add_val(value.serialize(self.ser)?);
        Ok(())
    }

    fn end(self) -> Result<Term<'a>, Error> {
        self.to_struct()
    }
}

impl<'a> ser::SerializeStructVariant for MapSerializer<'a> {
    type Ok = Term<'a>;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        self.add_key(key.serialize(self.ser)?);
        self.add_val(value.serialize(self.ser)?);
        Ok(())
    }

    fn end(self) -> Result<Term<'a>, Error> {
        self.to_struct_variant()
    }
}

/**
 *
 */
pub struct SequenceSerializer<'a> {
    ser: &'a Serializer<'a>,
    variant: Option<Term<'a>>,
    items: Vec<Term<'a>>,
}
impl<'a> SequenceSerializer<'a> {
    fn new(ser: &'a Serializer<'a>, len: Option<usize>, variant: Option<Term<'a>>) -> Self {
        match len {
            None => SequenceSerializer {
                ser,
                variant,
                items: Vec::new(),
            },
            Some(length) => SequenceSerializer {
                ser,
                variant,
                items: Vec::with_capacity(length),
            },
        }
    }

    fn add(&mut self, term: Term<'a>) -> () {
        self.items.push(term)
    }

    fn to_list(&self) -> Result<Term<'a>, Error> {
        Ok(self.items.encode(self.ser.env))
    }

    fn to_tuple(&self) -> Result<Term<'a>, Error> {
        Ok(tuple::make_tuple(self.ser.env, &self.items))
    }

    fn to_tuple_variant(&self) -> Result<Term<'a>, Error> {
        let variant_term = self.variant.ok_or(Error::InvalidVariant)?;
        let tuple_term = self.to_tuple()?;
        Ok(tuple::make_tuple(
            self.ser.env,
            &vec![variant_term, tuple_term],
        ))
    }
}

/**
 *
 */
pub struct MapSerializer<'a> {
    ser: &'a Serializer<'a>,
    name: Option<Term<'a>>,
    variant: Option<Term<'a>>,
    keys: Vec<Term<'a>>,
    values: Vec<Term<'a>>,
}
impl<'a> MapSerializer<'a> {
    fn new(
        ser: &'a Serializer<'a>,
        len: Option<usize>,
        name: Option<Term<'a>>,
        variant: Option<Term<'a>>,
    ) -> Self {
        match len {
            None => MapSerializer {
                ser,
                name,
                variant,
                keys: Vec::new(),
                values: Vec::new(),
            },
            Some(length) => MapSerializer {
                ser,
                name,
                variant,
                keys: Vec::with_capacity(length),
                values: Vec::with_capacity(length),
            },
        }
    }

    fn add_key(&mut self, term: Term<'a>) -> () {
        self.keys.push(term)
    }

    fn add_val(&mut self, term: Term<'a>) -> () {
        self.values.push(term)
    }

    fn to_map(&self) -> Result<Term<'a>, Error> {
        match Term::map_from_arrays(self.ser.env, &self.keys, &self.values) {
            Err(_reason) => Err(Error::InvalidMap),
            Ok(term) => Ok(term),
        }
    }

    fn to_struct(&self) -> Result<Term<'a>, Error> {
        let module_term = self.name.ok_or(Error::InvalidStructName)?;
        let struct_atom = atoms::__struct__().to_term(self.ser.env);
        self.to_map()
            .map_err(|_| Error::InvalidStruct)?
            .map_put(struct_atom, module_term)
            .map_err(|_| Error::InvalidStruct)
    }

    // TODO: support :__struct__ and atom key? is name/variant correct?
    fn to_struct_variant(&self) -> Result<Term<'a>, Error> {
        let variant_term = self.variant.ok_or(Error::InvalidVariant)?;
        let struct_term = self.to_struct()?;
        Ok(tuple::make_tuple(
            self.ser.env,
            &vec![variant_term, struct_term],
        ))
    }
}

mod tests {
    use super::*;

    fn int<'a>(ser: &'a Serializer<'a>, expected: Term<'a>) -> Result<(), Error> {
        let actual = ser.to_term(&100)?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
