defmodule SerdeRustlerTests.Mixfile do
  use Mix.Project

  @name :serde_rustler_tests
  @version "0.0.1"
  @description """
  """
  @github "https://github.com/datalove-app/serde_rustler"
  @files ["mix.exs", "mix.lock", "lib", "native", "test", "README.md"]
  @maintainers ["Sunny G"]
  @licenses ["MIT"]

  # ------------------------------------------------------------

  def project do
    in_production = Mix.env() == :prod

    [
      app: @name,
      version: @version,
      description: @description,
      docs: docs(),
      package: package(),
      deps: deps() ++ dev_deps(),
      test_coverage: [tool: ExCoveralls],
      preferred_cli_env: [
        coveralls: :test,
        "coveralls.detail": :test
      ],
      elixir: "~> 1.8",
      build_embedded: in_production,
      start_permanent: in_production,
      rustler_crates: rustler_crates()
    ]
  end

  def application do
    spec = [extra_applications: []]

    if Mix.env() == :bench do
      Keyword.put_new(spec, :applications, [:logger])
    else
      spec
    end
  end

  defp deps() do
    [{:rustler, "~> 0.22.0"}]
  end

  defp dev_deps() do
    [
      {:benchee, "~> 1.0", only: [:bench]},
      {:benchee_html, "~> 1.0", only: [:bench]},
      {:benchee_markdown, "~> 0.2", only: [:bench]},
      {:credo, "~> 1.0.0", only: [:dev, :test], runtime: false},
      {:dialyxir, "~> 0.5", only: [:dev], runtime: false},
      {:excoveralls, "~> 0.10", only: [:test]},
      {:ex_doc, "~> 0.19", only: [:dev], runtime: false},
      {:mix_test_watch, "~> 0.8", only: [:dev], runtime: false},
      # JSON serialization libs for benchmarks
      {:exjsx, "~> 4.0", only: [:dev, :bench]},
      {:jason, "~> 1.2"},
      {:jiffy, "~> 1.0", only: [:dev, :bench]},
      {:json, "~> 1.3", only: [:dev, :bench]},
      {:jsone, "~> 1.4", only: [:dev, :bench]},
      {:poison, "~> 4.0", only: [:test, :bench]},
      {:tiny, "~> 1.0", only: [:dev, :bench]}
    ]
  end

  defp rustler_crates do
    [
      serde_rustler_tests: [
        path: __DIR__ <> "/native/serde_rustler_tests",
        mode: rustler_mode(Mix.env()),
        default_features: true,
        features: []
      ]
    ]
  end

  defp rustler_mode(:bench), do: :release
  defp rustler_mode(:prod), do: :release
  defp rustler_mode(_), do: :debug

  defp package do
    [
      name: @name,
      files: @files,
      maintainers: @maintainers,
      licenses: @licenses,
      links: %{
        "GitHub" => @github
      }
    ]
  end

  defp docs do
    [main: "readme", source_url: @github, extras: ["README.md"]]
  end
end
