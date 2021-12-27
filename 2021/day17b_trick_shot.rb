x_eq, y_eq = gets.split("target area: ")[1].split(", ")
left, right = x_eq.split("=")[1].split("..").map(&:to_i)
bot, top = y_eq.split("=")[1].split("..").map(&:to_i)

# ASSUMPTION: `bot` and `top` are negative, the target area is below the x axis.
# ASSUMPTION: `left` and `right` are positive.

def does_it_land vel_x, vel_y, range_x, range_y
  x = 0
  y = 0
  while x < range_x.last and y > range_y.first
    x += vel_x
    y += vel_y
    vel_y -= 1
    vel_x -= 1 if vel_x > 0
    return true if range_x.include? x and range_y.include? y
  end
  return false
end

lower_bound = ((Math.sqrt(1 + 8 * left) - 1) / 2).floor
puts (lower_bound..right).to_a
  .product((bot..-bot - 1).to_a)
  .lazy
  .filter { |vel_x, vel_y| does_it_land vel_x, vel_y, left..right, bot..top }
  .count
