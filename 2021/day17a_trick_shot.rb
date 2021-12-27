x_eq, y_eq = gets.split("target area: ")[1].split(", ")
left, right = x_eq.split("=")[1].split("..").map(&:to_i)
bot, top = y_eq.split("=")[1].split("..").map(&:to_i)

# ASSUMPTION: `bot` and `top` are negative, the target area is below the x axis.
# ASSUMPTION: `left` and `right` are positive.

# The key realization is that when the probe will shoot up with some y velocidty,
# then it will decelerate and stop, the accelerate downwards. When it crosses
# x axis again (y = 0), the y velocity will be the same as when the probe lauched +1,
# but negative. So if we want to have a chance of hitting the target area, this
# velocity must be smaller than the lowes point of the target area. Then we can
# just try all of the possibilities.
for vel_y in bot...0
  vel_y = -vel_y - 1
  # Number of steps to the probe go up, then down and cross the x axis again.
  n_steps = 2 * vel_y + 1
  y = 0
  vy = -vel_y - 1
  while y > bot and y > top
    y += vy
    vy -= 1
    n_steps += 1
  end
  # If we landed in the target area vertically, check if it is possible to do that
  # horizontally as well.
  if y >= bot and y <= top
    # This has a lower bound to what the velocity could be. It is based on the fact
    # that x velocity decreases by 1 each turn which is then a arithmetic progression
    # of velocities. If we solve for the lowest possible velocity that is enough to
    # make the probe get to the left border, we get to this. The highest possible
    # x velocity is just `right` since it would get to the right border in a signle
    # step.
    lower_bound = ((Math.sqrt(1 + 8 * left) - 1) / 2).floor
    x_vel = (lower_bound..right).find do |vel_x|
      x = 0
      steps = 0
      vx = vel_x
      while x < right and steps < n_steps
        x += vel_x
        vel_x -= 1 if vel_x > 0
        steps += 1
      end
      x >= left and x <= right and steps == n_steps
    end

    if x_vel
      puts "x_vel = #{x_vel}"
      puts "y_vel = #{vel_y}"
      puts "height = #{vel_y * (vel_y + 1) / 2}"
      break
    end
  end
end

