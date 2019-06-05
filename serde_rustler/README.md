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
// within your rustler NIF
#[macro_use]
extern crate rustler;

use rustler::{Env, Error, NifResult, Term};
use serde::{Serialize, Deserialize};
use serde_rustler::{Serializer, Deserializer};

rustler_export_nifs! {
    "Elixir.SerdeNif",
    [("round_trip", 1, round_trip)],
    None
}

#[derive(Debug, Serialize, Deserialize)]
enum AnimalType {
    Cat(String),
    Dog(String),
}

// NOTE: to actually serialize to an Elixir struct (rather than a just map with
// a :__struct__ key), you MUST tell serde to rename the struct to the full
// Elixir struct module atom.
#[derive(Debug, Serialize, Deserialize)]
[serde(rename = "Elixir.SerdeNif.Animal")]
struct Animal {
    name: String,
    age: u8,
}

fn round_trip<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let animal: Animal = Deserializer::from(args[0]).deserialize()?;

    println!("serialized animal = {}", animal);

    return Serializer::from(env).serialize();
}
```

Corresponding Elixir code:

```elixir
defmodule SerdeNif do
  use Rustler, otp_app: :serde_nif

  def round_trip(_term), do: :erlang.nif_error(:nif_not_loaded)

  defmodule Animal do
    @type t :: %Animal{name: bitstring, age: pos_integer}
    defstruct name: "", age: 0
  end

  defmodule AnimalType.Cat do
    require Record
    @type t :: record(:Cat, breed: String.t())
    Record.defrecord(:Cat, breed: "tabby")
  end

  defmodule AnimalType.Dog do
    require Record
    @type t :: record(:Dog, breed: String.t())
    Record.defrecord(:Dog, breed: "mutt")
  end
end
```

### Conversion Table

| Type Name | Serde Data Model | Default Elixir Term |
|-----------|------------------|---------------------|
| bool | `true` or `false` | `true` or `false` |
| number | `i8`, `i16`, `i32`, `i64`, `u8`, `u16`, `u32`, `u64`, `f32`, `f64` | `number` |
| char | `""` | `bitstring` |
| string | `""` | `bitstring` |
| byte array | `[u8]` | `<<_::_*8>>` |
| option | `Some(T)` or `None` | `T` or `:nil` |
| unit | `None` | `:nil` |
| unit struct | `struct Unit` | `:nil` |
| unit variant | `E::A` in `enum UnitVariant { A }` | <sup>[1](#atom)</sup> `:A` |
| newtype struct | `struct Millimeters(u8)` | <sup>[1](#atom)</sup> `{:Millimeters, u8}` |
| newtype variant | `E::N` in `enum E { N(u8) }` | <sup>[1](#atom)</sup> `{:N, u8}` |
| newtype variant (Result) | `Result::Ok(T)` or `Result::Err(E)` in `Result<T, E>` | <sup>[1](#atom)</sup> `{:ok, T}` or `{:error, E}` |
| seq | `Vec<T>` | `[T]` |
| tuple | `(u8,)` | `{u8,}` |
| tuple struct | `struct Rgb(u8, u8, u8)` | <sup>[1](#atom)</sup> `{:Rgb, u8, u8, u8}` |
| tuple variant | `E::T` in `enum E { T(u8, u8) }` | <sup>[1](#atom)</sup> `{:T, u8, u8}` |
| map | `HashMap<K, V>` | `%{}` |
| struct | `struct Rgb { r: u8, g: u8, b: u8 }` | <sup>[1](#atom)</sup> `%Rgb{ r: byte, g: byte, b: byte }` |
| struct variant | `E::S` in `enum E { Rgb { r: u8, g: u8, b: u8 } }` | <sup>[1](#atom)</sup> `%Rgb{ r: byte, g: byte, b: byte }` |

<a name="atom">1</a>: When serializing unknown input to terms, struct and record atoms will not be created and instead replaced with Elixir bitstrings. Therefore "records" will be tuples (`{bitstring, ...}`) and "structs" will be maps containing `%{:__struct__ => bitstring}` (feedback welcome :)).

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
