nums = gets.split(",").map(&:to_i)

all_costs = (nums.min..nums.max).map do |x|
    nums.map do |n|
        dist = (n - x).abs
        (dist.pow(2) + dist) / 2
    end
    .sum
end

puts all_costs.min

