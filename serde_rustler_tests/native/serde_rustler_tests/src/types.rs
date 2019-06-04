use serde::Serialize;

#[derive(Serialize)]
pub struct Unit;

#[derive(Serialize)]
pub enum UnitVariant {
    A,
}

#[derive(Serialize)]
pub enum NewtypeVariant {
    N(u8),
}

#[derive(Serialize)]
pub struct NewtypeStruct(u8);

impl NewtypeStruct {
    pub fn new(mm: u8) -> Self {
        NewtypeStruct(mm)
    }
}

#[derive(Serialize)]
pub struct TupleStruct(u8, u8, u8);

impl TupleStruct {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        TupleStruct(r, g, b)
    }
}

#[derive(Serialize)]
pub enum TupleVariant {
    T(u8, u8),
}

#[derive(Serialize)]
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

#[derive(Serialize)]
pub enum StructVariant {
    S { r: u8, g: u8, b: u8 },
}
