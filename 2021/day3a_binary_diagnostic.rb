len = 0
bit_counts = Hash[]
ARGF.each_line do |line|
  line.strip.reverse.each_char.zip(0..).each do |bit, idx|
    if bit_counts.has_key? idx
      bit_counts[idx] += bit.to_i
    else
      bit_counts[idx] = bit.to_i
    end
  end
  len += 1
end

gamma = 0
epsilon = 0
bit_counts.each_pair do |idx, ones|
  if ones > len/2
    gamma |= 1 << idx
  else
    epsilon |= 1 << idx
  end
end

puts gamma * epsilon
