defmodule SerdeRustlerTests.SerTest.Rgb do
  @moduledoc false
  defstruct [:r, :g, :b]
end

defmodule SerdeRustlerTests.SerTest do
  @moduledoc """
  Tests the [Serializer]() trait.
  """
  use ExUnit.Case, async: true

  alias SerdeRustlerTests.SerTest.Rgb

  test "Primitive Types" do
    assert SerdeRustlerTests.serialize("none", nil) == :ok
    assert SerdeRustlerTests.serialize("some", 100) == :ok
    assert SerdeRustlerTests.serialize("true", true) == :ok
    assert SerdeRustlerTests.serialize("false", false) == :ok

    # Numbers
    assert SerdeRustlerTests.serialize("i8 (min)", -128) == :ok
    assert SerdeRustlerTests.serialize("i8 (max)", 127) == :ok
    assert SerdeRustlerTests.serialize("i16 (min)", -32_768) == :ok
    assert SerdeRustlerTests.serialize("i16 (max)", 32_767) == :ok
    assert SerdeRustlerTests.serialize("i32 (min)", -2_147_483_648) == :ok
    assert SerdeRustlerTests.serialize("i32 (max)", 2_147_483_647) == :ok
    assert SerdeRustlerTests.serialize("i64 (min)", -9_223_372_036_854_775_808) == :ok
    assert SerdeRustlerTests.serialize("i64 (max)", 9_223_372_036_854_775_807) == :ok
    # assert SerdeRustlerTests.serialize("i128 (min)", 100) == :ok
    # assert SerdeRustlerTests.serialize("i128 (max)", 100) == :ok
    assert SerdeRustlerTests.serialize("u8 (min)", 0) == :ok
    assert SerdeRustlerTests.serialize("u8 (max)", 255) == :ok
    assert SerdeRustlerTests.serialize("u16 (min)", 0) == :ok
    assert SerdeRustlerTests.serialize("u16 (max)", 65_535) == :ok
    assert SerdeRustlerTests.serialize("u32 (min)", 0) == :ok
    assert SerdeRustlerTests.serialize("u32 (max)", 4_294_967_295) == :ok
    assert SerdeRustlerTests.serialize("u64 (min)", 0) == :ok
    assert SerdeRustlerTests.serialize("u64 (max)", 18_446_744_073_709_551_615) == :ok
    # assert SerdeRustlerTests.serialize("u128 (min)", 100) == :ok
    # assert SerdeRustlerTests.serialize("u128 (max)", 100) == :ok

    # Strings and Binaries
    # assert SerdeRustlerTests.serialize("char (empty)", "") == :ok
    assert SerdeRustlerTests.serialize("str (empty)", "") == :ok
    assert SerdeRustlerTests.serialize("str", "hello world") == :ok
    # assert SerdeRustlerTests.serialize("bytes", <<3, 2, 1, 0>>) == :ok
  end

  test "Unit Types" do
    assert SerdeRustlerTests.serialize("unit", nil) == :ok
    assert SerdeRustlerTests.serialize("unit struct", nil) == :ok
    assert SerdeRustlerTests.serialize("unit variant", :A) == :ok
  end

  test "Newtype Types" do
    assert SerdeRustlerTests.serialize("newtype struct", {:Millimeters, 255}) == :ok
    assert SerdeRustlerTests.serialize("newtype variant", {:N, 255}) == :ok
    assert SerdeRustlerTests.serialize("newtype variant (ok)", {:ok, 255}) == :ok
    assert SerdeRustlerTests.serialize("newtype variant (error)", {:error, "error reason"}) == :ok
  end

  test "Sequences" do
    assert SerdeRustlerTests.serialize("sequences (primitive)", ["hello", "world"]) == :ok

    assert SerdeRustlerTests.serialize("sequences (complex)", [
             {:Millimeters, 0},
             {:Millimeters, 255}
           ]) == :ok
  end

  test "Tuple Types" do
    assert SerdeRustlerTests.serialize("tuple (empty)", {}) == :ok
    assert SerdeRustlerTests.serialize("tuple", {0, 255}) == :ok
    assert SerdeRustlerTests.serialize("tuple struct", {:Rgb, 0, 128, 255}) == :ok
  end

  test "Map and Struct Types" do
    simple_map = %{"key" => "hello", "val" => "world"}
    assert SerdeRustlerTests.serialize("map (simple)", simple_map) == :ok

    complex_map = %{
      "key" => %{__struct__: :Struct, r: 0, g: 0, b: 0},
      "val" => %{__struct__: :Struct, r: 255, g: 255, b: 255}
    }

    assert SerdeRustlerTests.serialize("map (complex)", complex_map) == :ok

    struct = %{__struct__: :Struct, r: 0, g: 128, b: 255}
    assert SerdeRustlerTests.serialize("struct", struct) == :ok

    struct_variant = %{__struct__: :S, r: 0, g: 128, b: 255}
    assert SerdeRustlerTests.serialize("struct variant", struct_variant) == :ok
  end
end
