x = 0
y = 0
ARGF.each_line do |line|
  if line.start_with? "forward"
    x += line.delete_prefix("forward").to_i
  elsif line.start_with? "down"
    y += line.delete_prefix("down").to_i
  elsif line.start_with? "up"
    y -= line.delete_prefix("up").to_i
  end
end

puts "Result #{x * y}"
