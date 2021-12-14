segment_count_to_digits = Hash[
  2 => 1,
  3 => 7,
  4 => 4,
  5 => [2, 3, 5],
  6 => [0, 6],
  7 => 8,
]

count = 0

ARGF.each_line do |line|
  _, outputs = line.split("|").map(&:split)
  outputs.each do |seq|
    count += segment_count_to_digits[seq.length].is_a?(Numeric) ? 1 : 0
  end
end

puts count
