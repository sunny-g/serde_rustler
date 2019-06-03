defmodule SerdeRustlerTests do
  use Rustler, otp_app: :serde_rustler_tests

  def serialize(_test_name, _expected), do: nif_error()

  defp nif_error, do: :erlang.nif_error(:nif_not_loaded)
end
