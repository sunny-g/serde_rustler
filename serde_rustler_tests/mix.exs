defmodule SerdeRustler.Mixfile do
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

    [ app:                @name,
      version:            @version,
      description:        @description,
      docs:               docs(),
      package:            package(),
      deps:               deps() ++ dev_deps(),
      test_coverage:      [tool: ExCoveralls],
      preferred_cli_env:  [
        coveralls: :test,
        "coveralls.detail": :test,
      ],
      elixir:             "~> 1.8",
      build_embedded:     in_production,
      start_permanent:    in_production,
      compilers:          [:rustler] ++ Mix.compilers(),
      rustler_crates:     [serde_rustler: [
        path:             __DIR__ <> "/../../rust/serde_rustler",
        mode:             (if Mix.env() == :prod, do: :release, else: :debug),
        default_features: true,
        features:         [],
      ]],
    ]
  end

  defp deps() do
    [ {:rustler,        "~> 0.20.0"},
    ]
  end

  defp dev_deps() do
    [ {:credo,          "~> 1.0.0", only: [:dev, :test], runtime: false},
      {:dialyxir,       "~> 0.5",   only: [:dev],        runtime: false},
      {:excoveralls,    "~> 0.10",  only: [:test]},
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
