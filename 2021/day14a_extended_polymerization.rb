state = gets.strip
gets
rules = ARGF.map { |line| line.strip.split " -> " }.to_h

for _ in 1..10
  next_state = ""
  state.chars.each_cons(2) do |left, right|
    next_state << left + rules[left + right]
  end
  next_state << state.chars.last
  state = next_state
end

counts = state.chars.group_by(&:itself).transform_values!(&:length)
puts counts.values.max - counts.values.min
