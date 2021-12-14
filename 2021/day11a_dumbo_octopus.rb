require 'matrix'

grid = Matrix[*ARGF.map { |line| line.strip.chars.map(&:to_i) }]
count = 0

for n in 1..100
  # Start with all indicies of the matrix
  q = (0..grid.row_count - 1).to_a.product((0..grid.column_count - 1).to_a)
  while not q.empty?
    i, j = q.pop
    next if i < 0 or i >= grid.row_count or j < 0 or j >= grid.column_count
    next if grid[i, j] > 9
    grid[i, j] += 1
    q += (i-1..i+1).to_a.product((j-1..j+1).to_a) - [[i, j]] if grid[i, j] > 9
  end

  grid.each_with_index do |el, i, j|
    if el > 9
      grid[i, j] = 0
      count += 1
    end
  end
end

puts count
