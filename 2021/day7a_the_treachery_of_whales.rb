nums = gets.split(",").map(&:to_i)

all_costs = (nums.min..nums.max).map do |x|
    nums.map { |n| (n - x).abs }.sum
end

puts all_costs.min
