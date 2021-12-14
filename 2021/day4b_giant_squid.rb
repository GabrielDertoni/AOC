require 'matrix'

def read_board()
  Matrix[*(1..5).map { |_| gets.chomp.split.map(&:to_i) }]
end

nums = gets.chomp.split(",").map(&:to_i)
boards = []
while gets
  boards.append read_board
end

last_winner = nil
last_winners_lucky_num = nil
i = -1
while boards.length >= 1 && i < nums.length
  i += 1
  num = nums[i]
  winners_in_round = []
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
      last_winner = board
      last_winners_lucky_num = i
      winners_in_round.append board
    end
  end

  # We need to get out of the loop first to then delete the boards.
  winners_in_round.each do |board|
    boards.delete board
  end
end

sum = last_winner.filter(&:positive?).sum
puts sum * nums[last_winners_lucky_num]

