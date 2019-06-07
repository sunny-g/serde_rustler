defmodule SerdeRustlerTests.Readme.AnimalType.Cat do
  require Record
  @type t :: {__MODULE__, String.t()}
  Record.defrecord(:record, __MODULE__, breed: "tabby")
end

defmodule SerdeRustlerTests.Readme.AnimalType.Dog do
  require Record
  @type t :: {__MODULE__, String.t()}
  Record.defrecord(:record, __MODULE__, breed: "mutt")
end

defmodule SerdeRustlerTests.Readme.Animal do
  alias SerdeRustlerTests.Readme.AnimalType
  alias AnimalType.{Cat, Dog}
  require Cat
  require Dog

  @type t :: %__MODULE__{
          type: Cat.t() | Dog.t(),
          name: bitstring,
          age: pos_integer,
          owner: nil | bitstring
        }
  defstruct type: Cat.record(),
            name: "",
            age: 0,
            owner: nil
end

defmodule SerdeRustlerTests.Readme do
  alias SerdeRustlerTests.Readme.{Animal, AnimalType.Cat}
  require Cat

  def animal(),
    do: %Animal{
      type: Cat.record(),
      name: "Garfield",
      age: 41
    }

  @doc """
  Round-trips an `t:Animal.t/0` struct using NIF-defined type hints.

  ## Examples:
      iex> #{__MODULE__}.test()
      {:ok, #{__MODULE__}.animal()}
  """
  def test(), do: SerdeRustlerTests.readme(@animal)

  @doc """
  Round-trips an `t:Animal.t/0` struct using `serde-transcode`.

  ## Examples:
      iex> #{__MODULE__}.round_trip()
      {:ok, #{__MODULE__}.animal()}
  """
  def round_trip(), do: SerdeRustlerTests.round_trip(@animal)
end
