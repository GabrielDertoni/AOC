# A Hash of numbers to numbers. The values are the number of fish with a
# specific internal clock and the key is that clock's value.
state = gets.split(",").map(&:to_i).group_by(&:itself).transform_values!(&:size)

for _ in 1..80
  num_spawning = state[0] || 0
  for day in 0..7
    state[day] = state[day + 1] || 0
  end
  state[8] = num_spawning
  state[6] += num_spawning
end

puts state.each_value.sum
