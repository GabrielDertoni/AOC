increments = 0
prev = nil
ARGF.each_line.each_cons(3) do |window|
  num = window.map(&:to_i).sum
  if prev != nil && prev < num
    increments += 1
  end
  prev = num
end
puts "There were #{increments} increments"

