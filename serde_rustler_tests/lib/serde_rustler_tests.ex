defmodule SerdeRustlerTests do
  @moduledoc """
  NIF wrapping Serializer and Deserializer tests written in the `native` rust crate.
  """

  use Rustler, otp_app: :serde_rustler_tests

  def run_ser_test(_test_name, _expected), do: :erlang.nif_error(:nif_not_loaded)
end
