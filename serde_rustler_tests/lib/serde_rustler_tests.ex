defmodule SerdeRustlerTests do
  @moduledoc """
  NIF wrapping Serializer and Deserializer tests written in the `native` rust crate.
  """

  use Rustler, otp_app: :serde_rustler_tests

  def test(_test_type, _test_name, _expected), do: :erlang.nif_error(:nif_not_loaded)
end
