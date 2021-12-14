total = 0

table = {
  ')' => 3,
  ']' => 57,
  '}' => 1197,
  '>' => 25137,
}

opening = "{[(<"
closing = "}])>"
match_closing = opening.chars.zip(closing.chars).to_h

ARGF.each_line do |line|
  stack = []
  for char in line.strip.chars
    if opening.include? char
      stack << char
    else
      expected = match_closing[stack.pop]
      if expected != char
        total += table[char]
        break
      end
    end
  end
end

puts total
