defmodule SerdeRustlerTests.NifTest.NewtypeStruct do
  @moduledoc "`struct NewtypeStruct(u8)`"
  require Record
  @type t :: {__MODULE__, non_neg_integer}
  Record.defrecord(:record, __MODULE__, num: 0)
end

defmodule SerdeRustlerTests.NifTest.NewtypeVariant do
  defmodule N do
    @moduledoc "`enum NewtypeVariant { N(u8) }`"
    require Record
    @type t :: {__MODULE__, non_neg_integer}
    Record.defrecord(:record, __MODULE__, num: 0)
  end
end

defmodule SerdeRustlerTests.NifTest.TupleStruct do
  @moduledoc "`struct TupleStruct(u8, u8, u8)`"
  require Record
  @type t :: {__MODULE__, non_neg_integer, non_neg_integer, non_neg_integer}
  Record.defrecord(:record, __MODULE__, r: 0, g: 0, b: 0)
end

defmodule SerdeRustlerTests.NifTest.TupleVariant do
  defmodule T do
    @moduledoc "`enum TupleVariant { T(u8, u8) }`"
    require Record
    @type t :: {__MODULE__, non_neg_integer, non_neg_integer}
    Record.defrecord(:record, __MODULE__, a: 0, b: 0)
  end
end

defmodule SerdeRustlerTests.NifTest.Struct do
  @moduledoc "`struct Struct {...}`"
  defstruct r: 0, g: 0, b: 0
end

defmodule SerdeRustlerTests.NifTest.StructVariant do
  defmodule S do
    @moduledoc "`enum StructVariant { S{...} }`"
    defstruct r: 0, g: 0, b: 0
  end
end

defmodule SerdeRustlerTests.NifTest do
  @moduledoc """
  Tests the Serializer and Deserializer trait's behaviour against the primitives, records and structs defined here and the enums and structs defined in `native/serde_rustler_tests/src/types.rs`.
  """

  use ExUnit.Case, async: true

  alias SerdeRustlerTests.NifTest.{
    NewtypeStruct,
    NewtypeVariant,
    TupleStruct,
    TupleVariant,
    Struct,
    StructVariant
  }

  require NewtypeStruct
  require NewtypeVariant.N
  require TupleStruct
  require TupleVariant.T

  describe "Primitive Types:" do
    test "option" do
      run_tests("none", nil)
      run_tests("some", 100)
    end

    test "boolean" do
      run_tests("true", true)
      run_tests("false", false)
    end

    test "i8" do
      run_tests("i8 (min)", -128)
      run_tests("i8 (max)", 127)
    end

    test "i16" do
      run_tests("i16 (min)", -32_768)
      run_tests("i16 (max)", 32_767)
    end

    test "i32" do
      run_tests("i32 (min)", -2_147_483_648)
      run_tests("i32 (max)", 2_147_483_647)
    end

    test "i64" do
      run_tests("i64 (min)", -9_223_372_036_854_775_808)
      run_tests("i64 (max)", 9_223_372_036_854_775_807)
    end

    @tag :skip
    test "i128" do
      run_tests("i128 (min)", 100)
      run_tests("i128 (max)", 100)
    end

    test "u8" do
      run_tests("u8 (min)", 0)
      run_tests("u8 (max)", 255)
    end

    test "u16" do
      run_tests("u16 (min)", 0)
      run_tests("u16 (max)", 65_535)
    end

    test "u32" do
      run_tests("u32 (min)", 0)
      run_tests("u32 (max)", 4_294_967_295)
    end

    test "u64" do
      run_tests("u64 (min)", 0)
      run_tests("u64 (max)", 18_446_744_073_709_551_615)
    end

    @tag :skip
    test "u128" do
      run_tests("u128 (min)", 100)
      run_tests("u128 (max)", 100)
    end

    @tag :skip
    test "chars" do
      # TODO: should be charlist type
      run_tests("char (empty)", '')
      # run_tests("char", '')
    end

    test "strings" do
      run_tests("str (empty)", "")
      run_tests("str", "hello world")
    end

    test "bytes" do
      run_tests("bytes", <<3, 2, 1, 0>>)
    end
  end

  describe "Unit Types:" do
    test "unit" do
      run_tests("unit", nil)
    end

    test "unit struct" do
      run_tests("unit struct", nil)
    end

    test "unit variant" do
      run_tests("unit variant", :"UnitVariant::A")
    end
  end

  describe "Newtype Types:" do
    test "newtype struct" do
      run_tests("newtype struct", NewtypeStruct.record(num: 255))
    end

    test "newtype variant" do
      run_tests("newtype variant", NewtypeVariant.N.record(num: 255))
    end

    test "newtype variant (Result::Ok(T), or {:ok, T})" do
      run_tests("newtype variant (ok)", {:ok, 255})
    end

    test "newtype variant (Result::Err(T), or {:error, T}" do
      run_tests("newtype variant (error)", {:error, "error reason"})
    end
  end

  describe "Sequences:" do
    test "sequences (primitive)" do
      run_tests("sequences (primitive)", ["hello", "world"])
    end

    test "sequences (complex)" do
      list = [NewtypeStruct.record(num: 0), NewtypeStruct.record(num: 255)]
      run_tests("sequences (complex)", list)
    end
  end

  describe "Tuple Types:" do
    test "tuple (empty)" do
      run_tests("tuple (empty)", nil)
    end

    test "tuple" do
      run_tests("tuple", {0, 255})
    end

    test "tuple struct" do
      run_tests("tuple struct", TupleStruct.record(r: 0, g: 128, b: 255))
    end

    test "tuple variant" do
      run_tests("tuple variant", TupleVariant.T.record(a: 0, b: 255))
    end
  end

  describe "Map and Struct Types:" do
    test "map (primitive)" do
      simple_map = %{"key" => "hello", "val" => "world"}
      run_tests("map (primitive)", simple_map)
    end

    test "map (complex)" do
      complex_map = %{
        "key" => %Struct{r: 0, g: 0, b: 0},
        "val" => %Struct{r: 255, g: 255, b: 255}
      }

      run_tests("map (complex)", complex_map)
    end

    test "struct" do
      struct = %Struct{r: 0, g: 128, b: 255}
      run_tests("struct", struct)
    end

    test "struct variant" do
      struct_variant = %StructVariant.S{r: 0, g: 128, b: 255}
      run_tests("struct variant", struct_variant)
    end
  end

  defp run_tests(test_name, expected_term) do
    actual_ser = SerdeRustlerTests.test("serialize", test_name, expected_term)

    assert actual_ser == :ok, ~s"""
      serializing `#{test_name}`
      expected: #{inspect(expected_term)} actual: #{print_err(actual_ser)}
    """

    actual_de = SerdeRustlerTests.test("deserialize", test_name, expected_term)

    assert actual_de == :ok, ~s"""
      deserializing `#{test_name}`
      error: #{print_err(actual_de)}
    """
  end

  defp print_err(:ok), do: ""
  defp print_err({:error, term}), do: inspect(term)
end
