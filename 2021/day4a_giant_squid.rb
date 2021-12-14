require 'matrix'

def read_board()
  Matrix[*(1..5).map { |_| gets.chomp.split.map(&:to_i) }]
end

nums = gets.chomp.split(",").map(&:to_i)
boards = []
while gets
  boards.append read_board
end

winner = nil
i = -1
while !winner
  i += 1
  num = nums[i]
  for board in boards
    # Mark lucky numbers
    board.each_with_index do |el, row, col|
      if el == num
        board[row, col] = -1
      end
    end

    # Check if board is a winner
    if board.row_vectors.any?    { |row| row.all? &:negative? } || \
       board.column_vectors.any? { |col| col.all? &:negative? }
      winner = board
      break
    end
  end
end

sum = winner.filter(&:positive?).sum
puts sum * nums[i]
