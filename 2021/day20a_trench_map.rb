require 'matrix'

algorithm = gets.tr('.#', '01').reverse.to_i(2)
gets # Blank line
image = Matrix.rows(ARGF.map { |line| line.strip.chars.map { |c| c == '#' ? 1 : 0 } })

def enhance image, algorithm, surround
  # image = pad image, surround
  rows, cols = image.row_count, image.column_count
  Matrix.build(rows + 2, cols + 2) do |row, col|
    idx = 0
    for i in row - 2..row
      for j in col - 2..col
        v = if i < 0 or i >= rows or j < 0 or j >= cols
              surround
            else
              image[i, j]
            end
        idx = (idx << 1) | v
      end
    end
    algorithm[idx]
  end
end

n = 2
surround = 0

for _ in 1..n
  image = enhance image, algorithm, surround
  if surround == 0
    surround = algorithm[0b000000000]
  else
    surround = algorithm[0b111111111]
  end
end

puts "Final image"
for row in image.row_vectors
  puts row.to_a.map { |b| b == 1 ? '#' : '.' }.join
end

count = 0
image.each { |el| count += 1 if el == 1 }
puts "Number of '#' in image is #{count}"
