defmodule SerdeRustlerTests.Json do
  @doc "Decodes the JSON string into a blockchain map."
  def decode(file), do: SerdeRustlerTests.decode_json(file)

  @doc "Encodes the blockchain map into a compact JSON string"
  def encode(term), do: SerdeRustlerTests.encode_json_pretty(term)
end
