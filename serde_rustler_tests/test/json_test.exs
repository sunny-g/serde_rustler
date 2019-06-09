defmodule SerdeRustlerTests.Json.JsonTest do
  use ExUnit.Case, async: true

  alias SerdeRustlerTests.Json

  setup :read_data

  @tag filename: "blockchain.json"
  test "Blockchain", ctx do
    run_decode(ctx)
    run_encode(ctx)
  end

  @tag filename: "github.json"
  test "Github", ctx do
    run_decode(ctx)
    run_encode(ctx)
  end

  @tag filename: "giphy.json"
  test "Giphy", ctx do
    run_decode(ctx)
    run_encode(ctx)
  end

  @tag filename: "govtrack.json"
  test "GovTrack", ctx do
    run_decode(ctx)
    run_encode(ctx)
  end

  @tag filename: "issue-90.json"
  test "Issue 90", ctx do
    run_decode(ctx)
    run_encode(ctx)
  end

  @tag filename: "json-generator-pretty.json"
  test "JSON Generator (Pretty)", ctx do
    run_decode(ctx)
    run_encode(ctx)
  end

  @tag filename: "json-generator.json"
  test "JSON Generator", ctx do
    run_decode(ctx)
    run_encode(ctx)
  end

  @tag filename: "pokedex.json"
  test "Pokedex", ctx do
    run_decode(ctx)
    run_encode(ctx)
  end

  @tag filename: "utf-8-escaped.json"
  test "UTF-8 escaped", ctx do
    run_decode(ctx)
    run_encode(ctx)
  end

  @tag filename: "utf-8-unescaped.json"
  test "UTF-8 unescaped", ctx do
    run_decode(ctx)
    run_encode(ctx)
  end

  defp read_data(ctx) do
    path = Path.expand("data/" <> ctx[:filename], __DIR__)
    file = File.read!(path)
    decoded = Poison.decode!(file)

    %{:jsonfile => file, :json => decoded}
  end

  defp run_decode(ctx) do
    expected = ctx[:json]
    actual = Json.decode(ctx[:jsonfile])

    assert expected == actual, ~s"""
      DECODING ERROR :: #{ctx[:filename]}
      expected: #{inspect(ctx[:json])}
      actual: #{inspect(actual)}
    """
  end

  defp run_encode(ctx) do
    expected = ctx[:json]
    actual = Json.encode(expected) |> Poison.decode!()

    assert expected == actual, ~s"""
      ENCODING ERROR :: #{ctx[:filename]}
      expected: #{inspect(expected)}
      actual: #{inspect(actual)}
    """
  end
end
