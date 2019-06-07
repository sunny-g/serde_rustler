# serde_rustler

<!-- [![GitHub tag](https://img.shields.io/github/tag/Naereen/StrapDown.js.svg)](https://GitHub.com/Naereen/StrapDown.js/tags/) -->
<!-- [![Build Status](https://semaphoreci.com/api/v1/sunny-g/xdr/branches/master/badge.svg)](https://semaphoreci.com/sunny-g/xdr) -->
[![Crates.io](https://img.shields.io/crates/v/serde_rustler.svg)](https://crates.io/crates/serde_rustler)
[![Documentation](https://docs.rs/serde_rustler/badge.svg)](https://docs.rs/serde_rustler)
[![MIT license](https://img.shields.io/badge/License-MIT-blue.svg)](https://lbesson.mit-license.org/)

`serde_rustler` provides [Serde](https://serde.rs) Serializer and Deserializer traits for [Rustler](https://github.com/rusterlium/rustler) types, so you can easily serialize and deserialize native Rust types directly to and from native Elixir terms within your NIFs.

## Installation

Install from [Crates.io](https://crates.io/crates/serde_rustler):

```toml
[dependencies]
serde_rustler = "0.0.1"
```

## API Overview

Below is an example of how you might use `serde_rustler` within a rust NIF:

```rust
#[macro_use]
extern crate rustler;

use rustler::{Env, error::Error as NifError, NifResult, Term};
use serde::{Serialize, Deserialize};
use serde_rustler::{from_term, to_term};

rustler_export_nifs! {
    "Elixir.SerdeNif",
    [("readme", 1, readme)],
    None
}

// NOTE: to serialize to the correct Elixir record, you MUST tell serde to
// rename the variants to the full Elixir record module atom.
#[derive(Debug, Serialize, Deserialize)]
enum AnimalType {
    #[serde(rename = "Elixir.SerdeNif.AnimalType.Cat")]
    Cat(String),
    #[serde(rename = "Elixir.SerdeNif.AnimalType.Dog")]
    Dog(String),
}

// NOTE: to serialize to an actual Elixir struct (rather than a just map with
// a :__struct__ key), you MUST tell serde to rename the struct to the full
// Elixir struct module atom.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Elixir.SerdeNif.Animal")]
struct Animal {
    #[serde(rename = "type")]
    _type: AnimalType,
    name: String,
    age: u8,
    owner: Option<String>,
}

fn readme<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let animal: Animal = from_term(args[0]).or(Err(NifError::BadArg))?;

    println!("serialized animal: {:?}", animal);

    to_term(env, animal).or(Err(NifError::BadArg))
}
```

Corresponding Elixir code (code structure, `import`s, `alias`es and `require`s simplified or omitted for brevity):

```elixir
defmodule SerdeNif do
  use Rustler, otp_app: :serde_nif

  def round_trip(_term), do: :erlang.nif_error(:nif_not_loaded)

  defmodule Animal do
    @type t :: %Animal{
      type: Cat.t() | Dog.t(),
      name: bitstring,
      age: pos_integer,
      owner: nil | bitstring
    }
    defstruct type: Cat.record(), name: "", age: 0, owner: nil

    @doc "Deserializes term as a Rust `Animal` struct, then serializes it back into an Elixir `Animal` struct. Should return true."
    def test() do
      animal = %Animal{
        type: Animal.Cat.record(),
        name: "Garfield",
        age: 41,
      }

      SerdeNif.readme(animal) == animal
    end
  end

  defmodule AnimalType.Cat do
    require Record
    @type t {__MODULE__, String.t()}
    Record.defrecord(:record, __MODULE__, breed: "tabby")
  end

  defmodule AnimalType.Dog do
    require Record
    @type t {__MODULE__, String.t()}
    Record.defrecord(:record, :Dog, breed: "mutt")
  end
end
```

### Conversion Table

| Type Name | Serde (Rust) Values | Elixir Terms (default behaviour) |
|-----------|------------------|---------------------|
| bool | `true` or `false` | `true` or `false` |
| <sup>[1](#todo)</sup> number | `i8`, `i16`, `i32`, `i64`, `u8`, `u16`, `u32`, `u64`, `f32`, `f64` (TODO: `i128` and `u128`) | `number` |
| <sup>[1](#atom)</sup> char | `""` | `bitstring` |
| string | `""` | `bitstring` |
| <sup>[2](#byte)</sup> byte array | `&[u8]` or `Vec<u8>` | `<<_::_*8>>` |
| option | `Some(T)` or `None` | `T` or `:nil` |
| unit | `None` | `:nil` |
| unit struct | `struct Unit` | `:nil` |
| <sup>[3](#atom)</sup> unit variant | `E::A` in `enum UnitVariant { A }` | `:A` |
| <sup>[3](#atom)</sup> newtype struct | `struct Millimeters(u8)` | `{:Millimeters, u8}` |
| <sup>[3](#atom)</sup> newtype variant | `E::N` in `enum E { N(u8) }` | `{:N, u8}` |
| newtype variant (any `Ok` and `Err` tagged enum) | `enum R<T, E> { Ok(T), Err(E) }` | `{:ok, T}` or `{:error, E}` |
| seq | `Vec<T>` | `[T]` |
| tuple | `(u8,)` | `{u8,}` |
| <sup>[3](#atom)</sup> tuple struct | `struct Rgb(u8, u8, u8)` | `{:Rgb, u8, u8, u8}` |
| <sup>[3](#atom)</sup> tuple variant | `E::T` in `enum E { T(u8, u8) }` | `{:T, u8, u8}` |
| <sup>[1](#todo)</sup> map | `HashMap<K, V>` | `%{}` |
| <sup>[3](#atom)</sup> struct | `struct Rgb { r: u8, g: u8, b: u8 }` | `%Rgb{ r: byte, g: byte, b: byte }` |
| <sup>[3](#atom)</sup> struct variant | `E::S` in `enum E { Rgb { r: u8, g: u8, b: u8 } }` | `%Rgb{ r: byte, g: byte, b: byte }` |

<a name="todo">1</a>: API still being decided / implemented.

<a name="byte">2</a>: Requires specifying a specific serialize implementation, such as [`serde_bytes`](https://crates.io/crates/serde_bytes/).

<a name="atom">3</a>: When serializing unknown input to terms, atoms will not be created and will instead be replaced with Elixir bitstrings. Therefore "records" will be tuples (`{bitstring, ...}`) and "structs" will be maps containing `%{:__struct__ => bitstring}` (feedback on this decision is welcome).

## TODO

- [ ] finalize behaviour around chars, charlists, iolists, map keys
- [ ] still getting used to Rust, so may need to improve error handling nnd ergnomoics around API
- [ ] support for `i128` and `u128`
- [ ] more extensive (i.e. possible addition of smoke, property-based) testing
- [ ] benchmarking

## Changelog

| Version | Change Summary |
| ------- | ---------------|
| [v0.0.2](https://crates.io/crates/serde_rustler/0.0.2) | ... |
| [v0.0.1](https://crates.io/crates/serde_rustler/0.0.1) | initial release |

## Contributing

1. Fork it [https://github.com/your_username/serde_rustler/fork](https://github.com/sunny-g/serde_rustler/fork)
2. Create your feature branch (`git checkout -b feature/fooBar`)
3. Commit your changes (`git commit -am 'Add some fooBar'`)
4. Push to the branch (`git push origin feature/fooBar`)
5. Create a new Pull Request

## Maintainers

- Sunny G - [@sunny-g](https://github.com/sunny-g)

<!-- ## Contributors -->

## License

MIT
