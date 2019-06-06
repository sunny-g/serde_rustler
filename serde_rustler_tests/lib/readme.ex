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

  @doc """
  ## Examples:
      iex> #{__MODULE__}.test()
      true
  """
  def test() do
    animal = %Animal{
      type: Cat.record(),
      name: "Garfield",
      age: 41
    }

    SerdeRustlerTests.readme(animal) == {:ok, animal}
  end
end
