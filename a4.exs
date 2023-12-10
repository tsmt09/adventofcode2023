defmodule A4 do
  def gameinfo(text) when is_binary(text) do
    [name, data] = String.split(text, ":", trim: true)
    id = String.to_integer(List.last(String.split(name, " ")))
    [wins, picks] = data |> String.split("|", trim: true)
    wins = wins |> String.split(" ", trim: true)
    picks = picks |> String.split(" ", trim: true)
    %{:id => id, :wins => wins, :picks => picks}
  end

  def incr([head | tail], count, inc) do
    if count == 0 do
      [ head | tail ]
    else
      head = head + inc
      [ head | incr(tail, count - 1, inc) ]
    end
  end

  def incr([], count, inc) do
    if count > 1 do
      [ 1 + inc | incr([], count - 1, inc)]
    else
      if count == 1 do
        [ 1 + inc ]
      else
        [ 1 ]
      end
    end
  end

end

{:ok, content} = File.read("a4.txt")

points = String.split(content, "\n", trim: true)
  |> Enum.reduce(0, fn x, acc ->
    info = A4.gameinfo(x)
    findings = Enum.filter(info[:picks], fn y -> Enum.find(info[:wins], fn x -> x ==y end) end) |> length()
    if findings <= 1 do
      acc + findings
    else
      acc + :math.pow(2, findings - 1)
    end
  end)
IO.puts(points)

cardssum = String.split(content, "\n", trim: true)
  |> Enum.reduce(%{:sum => 0, :arr => [1]}, fn x, acc ->
    info = A4.gameinfo(x)
    {thisscore, new_arr} = List.pop_at(acc[:arr], 0)
    findings = Enum.filter(info[:picks], fn y -> Enum.find(info[:wins], fn x -> x ==y end) end) |> length()
    new_arr = A4.incr(new_arr, findings, thisscore)
    %{:sum => acc[:sum] + thisscore, :arr => new_arr}
  end)

IO.puts(cardssum[:sum])
