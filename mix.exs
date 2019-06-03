defmodule SerdeRustler.Mixfile do
  @moduledoc false

  use Mix.Project

  @name         :serde_rustler
  @version      "0.0.1-dev"
  @description  """
  """
  @github       "https://github.com/datalove-app/serde_rustler"
  @files        ["mix.exs", "mix.lock", "lib", "test", "README.md"]
  @maintainers  ["Sunny G"]
  @licenses     ["MIT"]

  # ------------------------------------------------------------

  def project do
    in_production = Mix.env == :prod

    [ app:              @name,
      version:          @version,
      description:      @description,
      elixir:           "~> 1.8",
      docs:             docs(),
      package:          package(),
      deps:             deps() ++ dev_deps(),
      build_embedded:   in_production,
      start_permanent:  in_production,
    ]
  end

  defp deps() do
    [ {:serde_rustler_tests,  path: __DIR__ <> "/serde_rustler_tests"},
    ]
  end

  defp dev_deps() do
    [ {:credo,          "~> 1.0.0", only: [:dev, :test], runtime: false},
      {:dialyxir,       "~> 0.5",   only: [:dev],        runtime: false},
      {:ex_doc,         "~> 0.19",  only: [:dev],        runtime: false},
      {:inch_ex, github: "rrrene/inch_ex", only: [:dev, :test], runtime: false},
      {:mix_test_watch, "~> 0.8",   only: [:dev],        runtime: false},
    ]
  end

  defp package do
    [ name:        @name,
      files:       @files,
      maintainers: @maintainers,
      licenses:    @licenses,
      links:       %{
        "GitHub" => @github,
      },
    ]
  end

  defp docs do
    [ main:       "readme",
      source_url: @github,
      extras:     ["README.md"],
    ]
  end
end
