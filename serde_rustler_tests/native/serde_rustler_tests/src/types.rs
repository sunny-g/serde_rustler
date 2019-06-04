use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Unit;

#[derive(PartialEq, Serialize, Deserialize)]
pub enum UnitVariant {
    A,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub enum NewtypeVariant {
    N(u8),
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct NewtypeStruct(u8);

impl NewtypeStruct {
    pub fn new(mm: u8) -> Self {
        NewtypeStruct(mm)
    }
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct TupleStruct(u8, u8, u8);

impl TupleStruct {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        TupleStruct(r, g, b)
    }
}

#[derive(PartialEq, Serialize, Deserialize)]
pub enum TupleVariant {
    T(u8, u8),
}

#[derive(PartialEq, Serialize, Deserialize)]
#[serde(rename = "Elixir.SerdeRustlerTests.NifTest.Struct")]
pub struct Struct {
    r: u8,
    g: u8,
    b: u8,
}

impl Struct {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Struct { r: r, g: g, b: b }
    }
}

#[derive(PartialEq, Serialize, Deserialize)]
pub enum StructVariant {
    #[serde(rename = "Elixir.SerdeRustlerTests.NifTest.S")]
    S { r: u8, g: u8, b: u8 },
}
