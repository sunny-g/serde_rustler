defmodule SerdeRustlerTests.SerTest.Struct do
  @moduledoc false
  defstruct r: 0, g: 0, b: 0
end

defmodule SerdeRustlerTests.SerTest.S do
  @moduledoc "`enum StructVariant { S{...} }`"
  defstruct r: 0, g: 0, b: 0
end

defmodule SerdeRustlerTests.SerTest do
  @moduledoc """
  Tests the [Serializer]() trait.
  """
  use ExUnit.Case, async: true

  alias SerdeRustlerTests.SerTest.{S, Struct}

  describe "Primitive Types" do
    test "option" do
      assert_ser("none", nil)
      assert_ser("some", 100)
    end

    test "boolean" do
      assert_ser("true", true)
      assert_ser("false", false)
    end

    test "numbers" do
      assert_ser("i8 (min)", -128)
      assert_ser("i8 (max)", 127)
      assert_ser("i16 (min)", -32_768)
      assert_ser("i16 (max)", 32_767)
      assert_ser("i32 (min)", -2_147_483_648)
      assert_ser("i32 (max)", 2_147_483_647)
      assert_ser("i64 (min)", -9_223_372_036_854_775_808)
      assert_ser("i64 (max)", 9_223_372_036_854_775_807)
      # assert_ser("i128 (min)", 100)
      # assert_ser("i128 (max)", 100)
      assert_ser("u8 (min)", 0)
      assert_ser("u8 (max)", 255)
      assert_ser("u16 (min)", 0)
      assert_ser("u16 (max)", 65_535)
      assert_ser("u32 (min)", 0)
      assert_ser("u32 (max)", 4_294_967_295)
      assert_ser("u64 (min)", 0)
      assert_ser("u64 (max)", 18_446_744_073_709_551_615)
      # assert_ser("u128 (min)", 100)
      # assert_ser("u128 (max)", 100)
    end

    test "strings and binaries" do
      # assert_ser("char (empty)", "")
      assert_ser("str (empty)", "")
      assert_ser("str", "hello world")
      # assert_ser("bytes", <<3, 2, 1, 0>>)
    end
  end

  describe "Unit Types" do
    test "unit" do
      assert_ser("unit", nil)
    end

    test "unit struct" do
      assert_ser("unit struct", nil)
    end

    test "unit variant" do
      assert_ser("unit variant", :A)
    end
  end

  describe "Newtype Types" do
    test "newtype struct" do
      assert_ser("newtype struct", {:NewtypeStruct, 255})
    end

    test "newtype variant" do
      assert_ser("newtype variant", {:N, 255})
    end

    test "newtype variant (Result aka tagged tuple)" do
      assert_ser("newtype variant (ok)", {:ok, 255})
      assert_ser("newtype variant (error)", {:error, "error reason"})
    end
  end

  describe "Sequences" do
    test "sequences (primitive)" do
      assert_ser("sequences (primitive)", ["hello", "world"])
    end

    test "sequences (complex)" do
      list = [{:NewtypeStruct, 0}, {:NewtypeStruct, 255}]
      assert_ser("sequences (complex)", list)
    end
  end

  describe "Tuple Types" do
    test "tuple (empty)" do
      assert_ser("tuple (empty)", nil)
    end

    test "tuple" do
      assert_ser("tuple", {0, 255})
    end

    test "tuple struct" do
      assert_ser("tuple struct", {:TupleStruct, 0, 128, 255})
    end

    test "tuple variant" do
      assert_ser("tuple variant", {:T, 0, 255})
    end
  end

  describe "Map and Struct Types" do
    test "map (primitive)" do
      simple_map = %{"key" => "hello", "val" => "world"}
      assert_ser("map (primitive)", simple_map)
    end

    test "map (complex)" do
      complex_map = %{
        "key" => %Struct{r: 0, g: 0, b: 0},
        "val" => %Struct{r: 255, g: 255, b: 255}
      }

      assert_ser("map (complex)", complex_map)
    end

    test "struct" do
      struct = %Struct{r: 0, g: 128, b: 255}
      assert_ser("struct", struct)
    end

    test "struct variant" do
      struct_variant = %S{r: 0, g: 128, b: 255}
      assert_ser("struct variant", struct_variant)
    end
  end

  defp assert_ser(test_name, expected_term) do
    assert SerdeRustlerTests.run_ser_test(test_name, expected_term) == :ok
  end
end
