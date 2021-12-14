require 'set'

mat = ARGF.map { |line| line.strip.chars.map(&:to_i) }
basin_sizes = []

for row, i in mat.each_with_index
  for val, j in row.each_with_index
    checks = [
      (i == 0              or mat[i - 1][j] > val),
      (i == mat.length - 1 or mat[i + 1][j] > val),
      (j == 0              or mat[i][j - 1] > val),
      (j == row.length - 1 or mat[i][j + 1] > val),
    ]
    if checks.all?
      # We are in a low point, BFS to get basin size
      q = [[i, j]]
      vis = Set[]
      while !q.empty?
        x, y = q.pop
        next if vis.include? [x, y]
        vis.add [x, y]
        q.unshift [x - 1, y] if x > 0              and mat[x - 1][y] < 9
        q.unshift [x + 1, y] if x + 1 < mat.length and mat[x + 1][y] < 9
        q.unshift [x, y - 1] if y > 0              and mat[x][y - 1] < 9
        q.unshift [x, y + 1] if y + 1 < row.length and mat[x][y + 1] < 9
      end
      basin_sizes << vis.length
    end
  end
end

puts basin_sizes.max(3).reduce(:*)
