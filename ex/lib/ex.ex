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

  def validate_all(_, [], _), do: true

  def validate_all(rules, stack, number) do
    stack
    |> Enum.map(fn num -> Enum.any?(rules, &(&1 == [number, num])) end)
    |> Enum.all?(&(&1 == false))
  end

  def part_one({rules, prod}) do
    prod
    |> Enum.map(fn line ->
      Enum.reduce(line, {[], []}, fn current, {checks, numbers} ->
        check = validate_all(rules, numbers, current)
        {[check | checks], numbers ++ [current]}
      end)
    end)
    |> Enum.filter(fn {c, _} -> Enum.all?(c, &(&1 == true)) end)
    |> Enum.reduce(0, fn {_, nums}, acc ->
      acc + Enum.at(nums, div(length(nums), 2))
    end)
  end

  def validate(rules, num1, num2) do
    Enum.any?(rules, fn [n1, n2] -> n1 == num1 and n2 == num2 end)
  end

  def test_line(rules, list, stack \\ [], checks \\ [])

  def test_line(_, [], stack, checks), do: {stack, Enum.any?(checks, &(&1 == :fixed))}

  def test_line(rules, [num | rest], [], checks) do
    test_line(rules, rest, [num], [true | checks])
  end

  def test_line(rules, [num | rest] = numbers, stack, checks) do
    s = List.last(stack)
    current = validate(rules, s, num) |> dbg

    cond do
      current == true ->
        test_line(rules, rest, stack ++ [num], [true | checks])

      current == false and validate(rules, num, s) ->
        {_, new_stack} = List.pop_at(stack, -1)
        test_line(rules, [s | rest], new_stack ++ [num], [:fixed | checks])

      true -> {[], false}
    end
  end

  def part_two({rules, prod}) do
    numbers = Enum.at(prod, 5) |> dbg(charlists: :as_lists)
    {nums, _} = test_line(rules, numbers)
    validate_all(rules, nums, []) |> dbg

    """
    prod
    |> Enum.map(fn numbers ->
      dbg numbers
      test_line(rules, numbers)
    end)
    """
  end

  def main() do
    read_file("example.txt") |> part_two()
  end
end
