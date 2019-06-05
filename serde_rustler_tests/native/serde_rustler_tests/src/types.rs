use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Unit;

#[derive(PartialEq, Serialize, Deserialize)]
pub enum UnitVariant {
    #[serde(rename = "UnitVariant::A")]
    A,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub enum NewtypeVariant {
    N(u8),
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct NewtypeStruct(pub u8);

#[derive(PartialEq, Serialize, Deserialize)]
pub struct TupleStruct(pub u8, pub u8, pub u8);

#[derive(PartialEq, Serialize, Deserialize)]
pub enum TupleVariant {
    T(u8, u8),
}

#[derive(PartialEq, Serialize, Deserialize)]
#[serde(rename = "Elixir.SerdeRustlerTests.NifTest.Struct")]
pub struct Struct {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub enum StructVariant {
    #[serde(rename = "Elixir.SerdeRustlerTests.NifTest.S")]
    S { r: u8, g: u8, b: u8 },
}
