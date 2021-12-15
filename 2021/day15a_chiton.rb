require 'matrix'
require 'set'

map = Matrix.rows(ARGF.map { |line| line.strip.chars.map(&:to_i) })

# Each element is [x, y, cost]
queue = [[0, 0, 0]]
vis = Set[]
target = [map.column_count - 1, map.row_count - 1]

cost = while not queue.empty?
  x, y, cost = queue.shift
  break cost if [x, y] == target
  next if vis.include? [x, y]
  vis.add [x, y]
  neighbors = [x-1, x+1].product([y]) + [x].product([y-1, y+1])
  for nx, ny in neighbors
    if (0..map.column_count-1).include? nx and (0..map.row_count-1).include? ny
      queue.append [nx, ny, cost + map[nx, ny]]
    end
  end
  queue.sort_by! { |_, _, cost| cost }
end

puts cost
