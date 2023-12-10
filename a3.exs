defmodule A3 do
  def dotted_len(str, len) when len > 0 do
    str = str <> "."
    dotted_len(str, len - 1)
  end
  def dotted_len(str, len) when len <= 0 do
    str
  end
  def iterate_line([head | tail], last, sum) do
    last = if last == nil do
      dotted_len("", String.length(head))
    else
      last
    end
    peek = case List.first(tail) do
      nil -> dotted_len("", String.length(head))
      e -> e
    end
    IO.puts("---")
    sum = sum + find_sum_line(String.split(head, "", trim: true), last, head, peek, :other, nil, 0, "", 0)
    IO.puts("current sum: #{sum}")
    iterate_line(tail, head, sum)
  end
  def iterate_line([], last, sum) do
    sum
  end

  def find_sum_line([head | tail], last, current, peek, pretype, left_bound, index, collector, sum) do
    type = case Integer.parse(head) do
      {_, ""} -> :number
      _ -> :other
    end
    left_bound = if left_bound == nil && type == :number do
      if index == 0 do
        index
      else
        index - 1
      end
    else
      left_bound
    end
    collector = if type == :number do
      collector <> head
    else
      collector
    end
    [sum, left_bound, collector] = if pretype == :number && type != :number do
      last_slice = String.split(String.slice(last, left_bound..index), "", trim: true)
      current_slice = String.split(String.slice(current, left_bound..index), "", trim: true)
      peek_slice = String.split(String.slice(peek, left_bound..index), "", trim: true)
      IO.inspect(last_slice)
      IO.inspect(current_slice)
      IO.inspect(peek_slice)
      if check_line_symbol_in_bounds?(last_slice) ||
          check_line_symbol_in_bounds?(current_slice) ||
          check_line_symbol_in_bounds?(peek_slice) do
        ## todo check surroundings here
        {num, _} = Integer.parse((collector))
        [sum + num, nil, ""]
      else
        IO.puts("NOT surround FOR #{collector}")
        [sum, nil, ""]
      end
    else
      [sum, left_bound, collector]
    end
    find_sum_line(tail, last, current, peek, type, left_bound, index + 1, collector, sum)
  end
  def find_sum_line([], last, current, peek, pretype, left_bound, index, collector, sum) do
    if pretype == :number do
      last_slice = String.split(String.slice(last, left_bound..index),"", trim: true)
      current_slice = String.split(String.slice(current, left_bound..index),"", trim: true)
      peek_slice = String.split(String.slice(peek, left_bound..index), "", trim: true)
      if check_line_symbol_in_bounds?(last_slice) || check_line_symbol_in_bounds?(current_slice) || check_line_symbol_in_bounds?(peek_slice) do
        {num, _} = Integer.parse((collector))
        sum + num
      else
        IO.puts("NOT surround FOR #{collector}")
        sum
      end
    else
      sum
    end
  end

  def iterate_gear_line([head | tail], last, sum) do
    last = if last == nil do
      dotted_len("", String.length(head))
    else
      last
    end
    peek = case List.first(tail) do
      nil -> dotted_len("", String.length(head))
      e -> e
    end
    IO.puts("---")
    sum = sum + gear_ratios_line(String.split(head, "", trim: true), last, head, peek, 0, 0)
    IO.puts("current sum: #{sum}")
    iterate_gear_line(tail, head, sum)
  end
  def iterate_gear_line([], last, sum) do
    sum
  end

  def gear_ratios_line([head | tail], last, current, peek, index, sum) do
    sum = if head == "*" do
      IO.puts("found gear at #{index}")
      ratio = gear_ratio(last, current, peek, index)
      if ratio > 1 do
        sum + ratio
      else
        sum
      end
    else
      sum
    end
    gear_ratios_line(tail, last, current, peek, index + 1, sum)
  end
  def gear_ratios_line([], last, current, peek, index, sum) do
    sum
  end

  def gear_ratio(last, current, peek, index) do
    [last_sum, last_count] = find_prod_gear(String.split(last, "", trim: true), index, :other, nil, 0, "", 1, 0)
    [current_sum, current_count] = find_prod_gear(String.split(current, "", trim: true), index, :other, nil, 0, "", 1, 0)
    [peek_sum, peek_count] = find_prod_gear(String.split(peek, "", trim: true), index, :other, nil, 0, "", 1, 0)
    count = last_count + current_count + peek_count
    if count > 1 do
      last_sum*current_sum*peek_sum
    else
      0
    end
  end

  def find_prod_gear([head | tail], gear_index, pretype, left_bound, index, collector, prod, count) do
    type = case Integer.parse(head) do
      {_, ""} -> :number
      _ -> :other
    end
    left_bound = if left_bound == nil && type == :number do
      if index == 0 do
        index
      else
        index - 1
      end
    else
      left_bound
    end
    collector = if type == :number do
      collector <> head
    else
      collector
    end
    [prod, left_bound, collector, count] = if pretype == :number && type != :number do
      if gear_index >= left_bound && gear_index <= index do
        {num, _} = Integer.parse((collector))
        res = prod * num
        IO.puts("prod number #{res} at #{left_bound},#{index}. #{gear_index}")
        [res, nil, "", count + 1]
      else
        [prod, nil, "", count]
      end
    else
      [prod, left_bound, collector, count]
    end
    find_prod_gear(tail, gear_index, type, left_bound, index + 1, collector, prod, count)
  end
  def find_prod_gear([], gear_index, pretype, left_bound, index, collector, prod, count) do
    if pretype == :number do
      if gear_index >= left_bound && gear_index <= index do
        {num, _} = Integer.parse((collector))
        count = count + 1
        [prod * num, count]
      else
        [prod, count]
      end
    else
      [prod, count]
    end
  end

  def check_line_symbol_in_bounds?([head | tail]) do
    if head not in [".", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"] do
      true
    else
      check_line_symbol_in_bounds?(tail)
    end
  end
  def check_line_symbol_in_bounds?([]) do
    false
  end
end

test = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
...$.*...1
.664.598.."

{:ok, content} = File.read("a3.txt")
sum = A3.iterate_line(String.split(content, "\n", trim: true), nil, 0)
IO.puts(sum)
gear = A3.iterate_gear_line(String.split(content, "\n", trim: true), nil, 0)
IO.puts(gear)
