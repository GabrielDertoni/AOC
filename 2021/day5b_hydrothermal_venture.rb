position_counts = Hash[]

# Just trying out Ruby's features :)
class Integer
  def ge_two; self >= 2 end
  def sign; self > 0 ? 1 : (self < 0 ? -1 : 0) end
end


ARGF.each_line do |line|
  (x1,y1), (x2,y2) = line.split(" -> ").map { |part| part.split(",").map(&:to_i) }

  dx = (x2 - x1).sign
  dy = (y2 - y1).sign
  x = x1
  y = y1
  while x != x2 or y != y2
    position_counts[[x, y]] = (position_counts[[x, y]] || 0) + 1
    x += dx if x != x2
    y += dy if y != y2
  end
  position_counts[[x2, y2]] = (position_counts[[x2, y2]] || 0) + 1
end

puts position_counts.each_value.filter(&:ge_two).count

