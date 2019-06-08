defmodule SerdeRustlerTests.Json do
  require Poison

  @blockchain_pretty "../test/data/blockchain.json"
                     |> Path.expand(__DIR__)
                     |> File.read!()
  @blockchain Poison.decode!(@blockchain_pretty)

  @doc """
  Decodes the JSON string into a blockchain map.

  ## Examples:
      iex> decode()
      blockchain()

  """
  def decode(), do: SerdeRustlerTests.decode_json(@blockchain_pretty)

  @doc """
  Encodes the blockchain map into a compact JSON string

  ## Examples:
      iex> encode()
      blockchain()
  """
  def encode() do
    SerdeRustlerTests.encode_json_pretty(@blockchain)
    |> Poison.decode!()
  end

  def blockchain(), do: @blockchain
end
