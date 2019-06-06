defmodule SerdeRustlerTests do
  @moduledoc """
  NIF wrapping Serializer and Deserializer tests written in the `native` rust crate.
  """

  use Rustler, otp_app: :serde_rustler_tests

  def readme(_animal), do: :erlang.nif_error(:nif_not_loaded)
  def round_trip(_term), do: :erlang.nif_error(:nif_not_loaded)
  def test(_test_type, _test_name, _expected), do: :erlang.nif_error(:nif_not_loaded)
end
