defmodule A2 do
  def gameinfo(text) when is_binary(text) do
    [name, sets] = String.split(text, ":", trim: true)
    id = String.to_integer(List.last(String.split(name, " ")))
    sets = rounds(String.split(sets, ";", trim: true))
    %{:id => id, :sets => sets}
  end

  def rounds([head | tail]) do
    [sets_map(head) | rounds(tail)]
  end
  def rounds([]) do
    []
  end

  def sets_map(set) do
    map = %{}
    steps_map(String.split(set, ","), map)
  end

  def steps_map([head | tail], map) do
    [number, type] = String.split(head, " ", trim: true)
    map = append_map(type, number, map)
    steps_map(tail, map)
  end
  def steps_map([], map) do
    map
  end

  def append_map(text, number, map) when text == "blue", do: Map.put(map, :blue, String.to_integer(number))
  def append_map(text, number, map) when text == "red", do: Map.put(map, :red, String.to_integer(number))
  def append_map(text, number, map) when text == "green", do: Map.put(map, :green, String.to_integer(number))

  def evaluate(game) do
    if Enum.all?(game[:sets], fn set -> (set[:blue] <= 14 || set[:blue] == nil) && (set[:red] <= 12  || set[:red] == nil) && (set[:green] <= 13 || set[:green] == nil) end) do
      game[:id]
    else
      0
    end
  end

  def sum_games([head | tail], acc) do
    sum_games(tail, acc + evaluate(gameinfo(head)))
  end
  def sum_games([], acc) do
    acc
  end

  def int_zero(value) do
    if is_integer(value) do
      value
    else
      0
    end
  end

  def power(game) do
    blue_min = Enum.max_by(game[:sets], fn set -> int_zero(set[:blue]) end)[:blue]
    red_min = Enum.max_by(game[:sets], fn set -> int_zero(set[:red]) end)[:red]
    green_min = Enum.max_by(game[:sets], fn set -> int_zero(set[:green]) end)[:green]
    blue_min * red_min * green_min
  end

  def sum_power([head | tail], acc) do
    sum_power(tail, acc + power(gameinfo(head)))
  end
  def sum_power([], acc) do
    acc
  end
end

{:ok, content} = File.read("a2.txt")
lines = String.split(content, "\n")
sum = A2.sum_games(lines, 0)
IO.puts(sum)
sum = A2.sum_power(lines, 0)
IO.puts(sum)
