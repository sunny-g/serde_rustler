//!
//!

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rustler;
#[macro_use]
extern crate rustler_codegen;
extern crate serde_rustler;

pub mod de;
pub mod ser;
mod types;

rustler_export_nifs! {
    "Elixir.SerdeRustlerTests",
    [   ("serialize", 2, ser::test),
        // ("deserialize", 2, de::test)
    ],
    None
}
