x = 0
y = 0
aim = 0
ARGF.each_line do |line|
  if line.start_with? "forward"
    increment = line.delete_prefix("forward").to_i
    x += increment
    y += aim * increment
  elsif line.start_with? "down"
    aim += line.delete_prefix("down").to_i
  elsif line.start_with? "up"
    aim -= line.delete_prefix("up").to_i
  end
end

puts "Result #{x * y}"
