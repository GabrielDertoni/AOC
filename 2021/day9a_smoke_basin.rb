mat = ARGF.map { |line| line.strip.chars.map(&:to_i) }
total = 0
for row, i in mat.each_with_index
  for val, j in row.each_with_index
    checks = [
      (i == 0              or mat[i - 1][j] > val),
      (i == mat.length - 1 or mat[i + 1][j] > val),
      (j == 0              or mat[i][j - 1] > val),
      (j == row.length - 1 or mat[i][j + 1] > val),
    ]
    total += val + 1 if checks.all?
  end
end
puts total
