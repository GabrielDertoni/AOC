increments = 0
prev = nil
ARGF.each_line.map(&:to_i).each do |num|
  if prev != nil && prev < num
    increments += 1
  end
  prev = num
end
puts "There were #{increments} increments"
