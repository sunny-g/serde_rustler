# serde_rustler

[![star](http://githubbadges.com/star.svg?user=sunny-g&repo=serde_rustler&style=default)](https://github.com/sunny-g/serde_rustler)
[![fork](http://githubbadges.com/fork.svg?user=sunny-g&repo=serde_rustler&style=default)](https://github.com/sunny-g/serde_rustler/fork)
<!-- [![Build Status](https://semaphoreci.com/api/v1/sunny-g/xdr/branches/master/badge.svg)](https://semaphoreci.com/sunny-g/xdr) -->

[Serde](https://serde.rs) Serializer and Deserializer for [Rustler](https://github.com/rusterlium/rustler) NIFs.

## Contents

This repo contains the following:

- [`serde_rustler`](https://github.com/sunny-g/serde_rustler/tree/master/serde_rustler): Rust crate providing the Serializer and Deserializer; meant to be used within your NIF libraries.
- [`serde_rustler_tests`](https://github.com/sunny-g/serde_rustler/tree/master/serde_rustler_tests): NIF written for testing and benchmarking `serde_rustler`
  - see the repo [README](https://github.com/sunny-g/serde_rustler/blob/master/serde_rustler/README.md) for installation instructions, a simple example of usage, and data model conversion table.
  - check out the tests in [`readme_test.exs`](https://github.com/sunny-g/serde_rustler/blob/master/serde_rustler_tests/test/readme_test.exs) for the README example or [`serde_rustler_tests_test.exs`](https://github.com/sunny-g/serde_rustler/blob/master/serde_rustler_tests/test/serde_rustler_tests_test.exs) to see the Elixir-end of the types and tests, or [`serde_rustler_tests/src/test.rs`](https://github.com/sunny-g/serde_rustler/blob/master/serde_rustler_tests/native/serde_rustler_tests/src/test.rs) and [`serde_rustler_tests/src/types.rs`](https://github.com/sunny-g/serde_rustler/blob/master/serde_rustler_tests/native/serde_rustler_tests/src/types.rs) to see the NIF tests and Rust native types.
  - see [`json.rs`](https://github.com/sunny-g/serde_rustler/blob/master/serde_rustler_tests/native/serde_rustler_tests/src/json.rs) for the implementation of the JSON encoding/decoding benchmark functions, or [`benchmarks.exs`](https://github.com/sunny-g/serde_rustler/blob/master/serde_rustler_tests/test/benchmarks.exs) for the benchmarks themselves, or just [`encode.md`](https://github.com/sunny-g/serde_rustler/blob/master/serde_rustler_tests/output/encode.md) or [`decode.md`](https://github.com/sunny-g/serde_rustler/blob/master/serde_rustler_tests/output/decode.md) to skip to the results.
