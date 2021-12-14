initial_state = gets.strip
gets
rules = ARGF.map { |line| line.strip.split " -> " }.to_h
counts = initial_state.chars
  .each_cons(2)
  .group_by(&:itself)
  .transform_values!(&:length)

# We don't need to keep track of the entire sequence, only of the pairs of
# letters that will yield new letters. In every iteration each pair will create
# two new pairs and will disappear. For instance "ABC" has pairs "AB" and "BC".
# If there was a rule "AB -> C", the first pair would become two: "ACB" -> "AC"
# and "CA" and the original pair "AB" is gone. So we only need to keep track of
# the frequency of those pairs, not the entire sequence.

for i in 1..40
  for (left, right), occur in counts.dup
    new = rules[left + right]
    counts[[left, new]]  = counts.fetch([left, new], 0)  + occur
    counts[[new, right]] = counts.fetch([new, right], 0) + occur
    counts[[left, right]] -= occur
  end
end

# We add the counts of the character for each pair. Note that because the pairs
# are overlapping, each char will always* appear in two pairs, for example the
# input: "ABCD" would yield pairs "AB", "BC", "CD". So 'B' and 'C' appeared
# twice. The exception is the characters in the beginning and end of the
# sequence. However, these will not change, since we always put new characters
# in the middle! So in order to get the correct char count, we just add the
# beggining and and chars one more time (so they are also counted twice) and
# divide the whole thing by 2!

letter_counts = Hash.new
for (left, right), occur in counts
  letter_counts[left]  = letter_counts.fetch(left, 0)  + occur
  letter_counts[right] = letter_counts.fetch(right, 0) + occur
end
letter_counts[initial_state.chars[0]] += 1
letter_counts[initial_state.chars[-1]] += 1

letter_counts.transform_values! { |count| count / 2 }

puts letter_counts.values.max - letter_counts.values.min
