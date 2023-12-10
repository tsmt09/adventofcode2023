defmodule A1 do
  def mapping() do
    [
      [1, ["1", "one"]],
      [2, ["2", "two"]],
      [3, ["3", "three"]],
      [4, ["4", "four"]],
      [5, ["5", "five"]],
      [6, ["6", "six"]],
      [7, ["7", "seven"]],
      [8, ["8", "eight"]],
      [9, ["9", "nine"]]
    ]
  end

  def starts_with_number(text) do
    try do
      [num, _] = Enum.find(mapping(), fn x -> String.starts_with?(text, List.last(x)) end)
      num
    rescue
      MatchError -> starts_with_number(String.slice(text, 1..-1))
    end
  end

  def ends_with_number(text) do
    try do
      [num, _] = Enum.find(mapping(), fn x -> String.ends_with?(text, List.last(x)) end)
      num
    rescue
      MatchError -> ends_with_number(String.slice(text, 0..-2))
    end
  end

  def number(text) when is_binary(text) do
    first = starts_with_number(text)
    second = ends_with_number(text)
    first*10+second
  end

  def read_lines(path) do
    {:ok, content} = File.read(path)
    String.split(content, "\n")
  end

  def sum_numbers([head | tail], acc) do
    number = number(head)
    sum_numbers(tail, number + acc)
  end

  def sum_numbers([], acc) do
    acc
  end

  def read_result() do
    lines =  read_lines("a1.txt")
    sum_numbers(lines, 0)
  end
end

IO.puts(A1.read_result())
