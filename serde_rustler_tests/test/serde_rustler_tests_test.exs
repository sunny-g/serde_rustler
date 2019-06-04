defmodule SerdeRustlerTests.NifTest.Struct do
  @moduledoc false
  defstruct r: 0, g: 0, b: 0
end

defmodule SerdeRustlerTests.NifTest.S do
  @moduledoc "`enum StructVariant { S{...} }`"
  defstruct r: 0, g: 0, b: 0
end

defmodule SerdeRustlerTests.NifTest do
  @moduledoc """
  Tests the Serializer trait.
  """

  use ExUnit.Case, async: true

  alias SerdeRustlerTests.NifTest.{S, Struct}

  describe "Primitive Types" do
    test "option" do
      run_test("none", nil)
      run_test("some", 100)
    end

    test "boolean" do
      run_test("true", true)
      run_test("false", false)
    end

    test "numbers" do
      run_test("i8 (min)", -128)
      run_test("i8 (max)", 127)
      run_test("i16 (min)", -32_768)
      run_test("i16 (max)", 32_767)
      run_test("i32 (min)", -2_147_483_648)
      run_test("i32 (max)", 2_147_483_647)
      run_test("i64 (min)", -9_223_372_036_854_775_808)
      run_test("i64 (max)", 9_223_372_036_854_775_807)
      # run_test("i128 (min)", 100)
      # run_test("i128 (max)", 100)
      run_test("u8 (min)", 0)
      run_test("u8 (max)", 255)
      run_test("u16 (min)", 0)
      run_test("u16 (max)", 65_535)
      run_test("u32 (min)", 0)
      run_test("u32 (max)", 4_294_967_295)
      run_test("u64 (min)", 0)
      run_test("u64 (max)", 18_446_744_073_709_551_615)
      # run_test("u128 (min)", 100)
      # run_test("u128 (max)", 100)
    end

    test "strings and binaries" do
      # run_test("char (empty)", "")
      run_test("str (empty)", "")
      run_test("str", "hello world")
      # run_test("bytes", <<3, 2, 1, 0>>)
    end
  end

  describe "Unit Types" do
    test "unit" do
      run_test("unit", nil)
    end

    test "unit struct" do
      run_test("unit struct", nil)
    end

    test "unit variant" do
      run_test("unit variant", :A)
    end
  end

  describe "Newtype Types" do
    test "newtype struct" do
      run_test("newtype struct", {:NewtypeStruct, 255})
    end

    test "newtype variant" do
      run_test("newtype variant", {:N, 255})
    end

    test "newtype variant (Result aka tagged tuple)" do
      run_test("newtype variant (ok)", {:ok, 255})
      run_test("newtype variant (error)", {:error, "error reason"})
    end
  end

  describe "Sequences" do
    test "sequences (primitive)" do
      run_test("sequences (primitive)", ["hello", "world"])
    end

    test "sequences (complex)" do
      list = [{:NewtypeStruct, 0}, {:NewtypeStruct, 255}]
      run_test("sequences (complex)", list)
    end
  end

  describe "Tuple Types" do
    test "tuple (empty)" do
      run_test("tuple (empty)", nil)
    end

    test "tuple" do
      run_test("tuple", {0, 255})
    end

    test "tuple struct" do
      run_test("tuple struct", {:TupleStruct, 0, 128, 255})
    end

    test "tuple variant" do
      run_test("tuple variant", {:T, 0, 255})
    end
  end

  describe "Map and Struct Types" do
    test "map (primitive)" do
      simple_map = %{"key" => "hello", "val" => "world"}
      run_test("map (primitive)", simple_map)
    end

    test "map (complex)" do
      complex_map = %{
        "key" => %Struct{r: 0, g: 0, b: 0},
        "val" => %Struct{r: 255, g: 255, b: 255}
      }

      run_test("map (complex)", complex_map)
    end

    test "struct" do
      struct = %Struct{r: 0, g: 128, b: 255}
      run_test("struct", struct)
    end

    test "struct variant" do
      struct_variant = %S{r: 0, g: 128, b: 255}
      run_test("struct variant", struct_variant)
    end
  end

  defp run_test(test_name, expected_term) do
    assert SerdeRustlerTests.test("serialize", test_name, expected_term) == :ok
    assert SerdeRustlerTests.test("deserialize", test_name, expected_term) == :ok
  end
end
