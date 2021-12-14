table = {
  ')' => 1,
  ']' => 2,
  '}' => 3,
  '>' => 4,
}

opening = "{[(<"
closing = "}])>"
match_closing = opening.chars.zip(closing.chars).to_h

scores = []

ARGF.each_line do |line|
  stack = []
  is_currupted = false
  for char in line.strip.chars
    if opening.include? char
      stack << char
    else
      expected = match_closing[stack.pop]
      is_currupted = expected != char
      break if is_currupted
    end
  end
  if !is_currupted and !stack.empty?
    scores << stack.reverse
      .map { |c| table[match_closing[c]] }
      .reduce(0) { |score, n| score * 5 + n }
  end
end

puts scores.sort![scores.length/2]

