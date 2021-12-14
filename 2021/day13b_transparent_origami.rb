require 'matrix'
require 'set'

class Matrix
  def zip_with other, symbol
    row_counts = [self.row_count, other.row_count]
    col_counts = [self.column_count, other.column_count]
    Matrix.build(row_counts.min, col_counts.min) do |row, col|
      self[row, col].send symbol, other[row, col]
    end
  end

  def hflip; Matrix.rows    self.row_vectors.reverse    end
  def vflip; Matrix.columns self.column_vectors.reverse end

  def hslice(range); Matrix.rows    self.row_vectors[range]    end
  def vslice(range); Matrix.columns self.column_vectors[range] end
end

getting_coords = true
coords = []
instructions = []
ARGF.each_line do |line|
  line = line.strip
  if line.empty?
    getting_coords = false
    next
  end

  if getting_coords
    coords << line.split(',').map(&:to_i)
  else
    axis, coord = line.split("fold along ")[1].split("=")
    instructions << [axis, coord.to_i]
  end
end

max_x, max_y = Matrix.rows(coords).column_vectors.map(&:max)

# This is surprisingly slow if we don't use a proper `Set`!
coords = coords.to_set
paper = Matrix.build(max_y + 1, max_x + 1) do |row, col|
  coords.include?([col, row]) ? 1 : 0
end

for axis, coord in instructions
  if axis == 'y'
    up   = paper.hslice(..coord - 1)
    down = paper.hslice(coord + 1..2 * coord)

    # If the two halfs are not equal pad with zeros to make them the same size
    diff = up.row_count - down.row_count
    down = down.vstack Matrix.rows [[0] * down.column_count] * diff if diff > 0
    paper = up.zip_with(down.hflip, :|)
  else
    left  = paper.vslice(..coord - 1)
    right = paper.vslice(coord + 1..2 * coord)

    # If the two halfs are not equal pad with zeros to make them the same size
    diff = left.column_count - right.column_count
    right = right.hstack Matrix.columns [[0] * right.row_count] * diff if diff > 0
    paper = left.zip_with(right.vflip, :|)
  end
end

for row in paper.row_vectors
  puts row.to_a.join.tr("01", " █")
end
