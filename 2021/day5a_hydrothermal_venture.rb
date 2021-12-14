position_counts = Hash[]

ARGF.each_line do |line|
  (x1,y1), (x2,y2) = line.split(" -> ").map { |part| part.split(",").map(&:to_i) }

  if x1 == x2
    ys = [y1, y2]
    for y in ys.min..ys.max
      position_counts[[x1, y]] = (position_counts[[x1, y]] || 0) + 1
    end
  elsif y1 == y2
    xs = [x1, x2]
    for x in xs.min..xs.max
      position_counts[[x, y1]] = (position_counts[[x, y1]] || 0) + 1
    end
  end
end

# Just trying out Ruby's features :)
class Integer
  def ge_two; self >= 2 end
end

puts position_counts.each_value.filter(&:ge_two).count
