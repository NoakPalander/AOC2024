defmodule Ex.Day5 do
  def read_file(path) do
    [rules, prod] = File.read!(path) |> String.split("\n\n") |> Enum.map(&String.trim/1)

    rules =
      rules
      |> String.split("\n")
      |> Enum.map(fn s -> String.split(s, "|") |> Enum.map(&String.to_integer/1) end)

    prod =
      prod
      |> String.split("\n")
      |> Enum.map(fn s -> String.split(s, ",") |> Enum.map(&String.to_integer/1) end)

    {rules, prod}
  end

  def validate(_, [], _), do: true

  def validate(rules, stack, number) do
    stack
    |> Enum.map(fn num -> Enum.any?(rules, &(&1 == [number, num])) end)
    |> Enum.all?(&(&1 == false))
  end

  def part_one({rules, prod}) do
    prod
    |> Enum.map(fn line ->
      Enum.reduce(line, {[], []}, fn current, {checks, numbers} ->
        check = validate(rules, numbers, current)
        {[check | checks], numbers ++ [current]}
      end)
    end)
    |> Enum.filter(fn {c, _} -> Enum.all?(c, &(&1 == true)) end)
    |> Enum.reduce(0, fn {_, nums}, acc ->
      acc + Enum.at(nums, div(length(nums), 2))
    end)
  end

  def main() do
    read_file("input.txt") |> part_one()
  end
end
