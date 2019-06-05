# serde_rustler

[Serde](https://serde.rs) [Serializer]() and [Deserializer]() traits for [Rustler](https://github.com/rusterlium/rustler) NIFs.

[Source](https://github.com/sunny-g/serde_rustler)
<!-- [![Build Status](https://semaphoreci.com/api/v1/sunny-g/xdr/branches/master/badge.svg)](https://semaphoreci.com/sunny-g/xdr) -->
[![Crates.io](https://img.shields.io/crates/v/serde_rustler.svg)](https://crates.io/crates/serde_rustler)
[![Documentation](https://docs.rs/serde_rustler/badge.svg)](https://docs.rs/serde_rustler)

`serde_rustler` provides [Serde](https://serde.rs) [Serializer]() and [Deserializer]() traits for [Rustler](https://github.com/rusterlium/rustler) types, so you can easily serialize and deserialize native Rust types directly to and from native Elixir terms within your [NIFs]().

## Installation

Install from [Crates.io](https://crates.io/crates/serde_rustler):

```toml
[dependencies]
serde_rustler = "0.0.1"
```

## API Overview

```rust
```

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
| unit variant | `E::A` in `enum UnitVariant { A }` | *`:A` |
| newtype struct | `struct Millimeters(u8)` | <sup>[1](#lossy)</sup>`{:Millimeters, u8}` |
| newtype variant | `E::N` in `enum E { N(u8) }` | <sup>[1](#lossy)</sup>`{:N, u8}` |
| seq | `Vec<T>` | `[T]` |
| tuple | `(u8,)` | `{u8,}` |
| tuple struct | `struct Rgb(u8, u8, u8)` | <sup>[1](#lossy)</sup>`{:Rgb, u8, u8, u8}` |
| tuple variant | `E::T` in `enum E { T(u8, u8) }` | <sup>[1](#lossy)</sup>`{:T, u8, u8}` |
| map | `HashMap<K, V>` | `%{}` |
| struct | `struct Rgb { r: u8, g: u8, b: u8 }` | <sup>[1](#lossy)</sup>`%Rgb{ r: byte, g: byte, b: byte }` |
| struct variant | `E::S` in `enum E { Rgb { r: u8, g: u8, b: u8 } }` | <sup>[1](#lossy)</sup>`%Rgb{ r: byte, g: byte, b: byte }` |

<a name="lossy">1</a>: If transcoding, the atom may not exist and will instead be serialized/deserialized to/from an Elixir bitstring. This means that may only be maps with a `__struct__` key and string value. These types are also lossy (newtype structs and variants are indistinguishable in Elixir terms), so deserialization hints may be required.

## Changelog

| Version | Change Summary |
| ------- | ---------------|
| [v0.0.1](https://crates.io/crates/serde_rustler/0.0.1) | initial release |

## Contributing

1. Fork it [https://github.com/your_username/xdr/fork](https://github.com/sunny-g/xdr/fork)
2. Create your feature branch (`git checkout -b feature/fooBar`)
3. Commit your changes (`git commit -am 'Add some fooBar'`)
4. Push to the branch (`git push origin feature/fooBar`)
5. Create a new Pull Request

## Maintainers

- Sunny G - [@sunny-g](https://github.com/sunny-g)

<!-- ## Contributors -->

## License

MIT
