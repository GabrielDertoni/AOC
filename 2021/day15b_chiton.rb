require 'matrix'
require 'set'
require 'algorithms'

curr = Matrix.rows(ARGF.map { |line| line.strip.chars.map(&:to_i) })
acc = Matrix.empty curr.row_count, 0
for _ in 1..5
  acc = acc.hstack curr.dup
  curr.map! { |val| (val % 9) + 1 }
end

curr = acc
acc = Matrix.empty 0, curr.column_count
for _ in 1..5
  acc = acc.vstack curr.dup
  curr.map! { |val| (val % 9) + 1 }
end

map = acc

# Each element is [x, y, cost]
queue = Containers::MinHeap.new
queue.push 0, [0, 0]
vis = Set[]
target = [map.column_count - 1, map.row_count - 1]

cost = while not queue.empty?
  cost = queue.next_key
  x, y = queue.pop
  break cost if [x, y] == target
  next if vis.include? [x, y]
  vis.add [x, y]
  neighbors = [x-1, x+1].product([y]) + [x].product([y-1, y+1])
  for nx, ny in neighbors
    if (0..map.column_count-1).include? nx and (0..map.row_count-1).include? ny
      queue.push cost + map[nx, ny], [nx, ny]
    end
  end
end

puts cost
