require 'matrix'

grid = Matrix[*ARGF.map { |line| line.strip.chars.map(&:to_i) }]

iter = for iter in (1..)
  # Start with all indicies of the matrix
  q = (0..grid.row_count - 1).to_a.product((0..grid.column_count - 1).to_a)
  while not q.empty?
    i, j = q.pop
    next if i < 0 or i >= grid.row_count or j < 0 or j >= grid.column_count
    next if grid[i, j] > 9
    grid[i, j] += 1
    q += (i-1..i+1).to_a.product((j-1..j+1).to_a) - [[i, j]] if grid[i, j] > 9
  end

  break iter if grid.all?(&9.method(:<))
  grid.each_with_index { |el, i, j| grid[i, j] = 0 if el > 9 }
end

puts iter
