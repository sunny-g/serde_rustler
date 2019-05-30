# Serde
**Elixir NIF for serialization and deserialization of data and data streams, wrapping [Serde](https://github.com/serde-rs/serde) and common Serde [formats](https://serde.rs/#data-formats)**

[Source](https://github.com/sunny-g/xerde) | [Documentation](https://hexdocs.pm/xerde)

[![Build Status](https://semaphoreci.com/api/v1/sunny-g/xerde/branches/master/badge.svg)](https://semaphoreci.com/sunny-g/xerde)

**TODO: add project description**

## Why?
Recently, I have been:
- playing around with a lot of IPFS utilities, in particular multicodec
- wanting to use multiple (variously maintained) serialization and deserialization libraries in Elixir for multicodec
- also playing a lot with rust and wanting to write a NIF with Rustler
- wanting to learn how to use Elixir benchmarking tools

## Features
**TODO: outline features**

**TODO: add table comparing translation between Elixir and Serde data model**

## Goals
**TODO: define project goals (and if so, non-goals)**

## Installation

Install from [Hex.pm](https://hex.pm/packages/xerde):

```elixir
def deps do
  [{xerde: "~> 0.0.1"}]
end
```

## Basic Usage/Walk-through

```elixir
# TODO: code examples
```

## API

- [`function/1`](#function)

### `function`

```elixir
@spec function(t) :: nil
```

**TODO: `function` description**

##### example:

```elixir
function(0)   # nil
function(nil) # nil
```

## Testing

```
$ mix test
```

## Changelog

| Version | Change Summary |
| ------- | -------------- |
| [v0.0.1](https://hex.pm/packages/xerde/0.0.1) | **TODO: add initial release notes ** |

## Contributing

1. Fork it [https://github.com/your_username/xerde/fork](https://github.com/sunny-g/xerde/fork)
2. Create your feature branch (`git checkout -b feature/fooBar`)
3. Commit your changes (`git commit -am 'Add some fooBar'`)
4. Push to the branch (`git push origin feature/fooBar`)
5. Create a new Pull Request

## Maintainers

- Sunny G - [@sunny-g](https://github.com/sunny-g)

## License

MIT
