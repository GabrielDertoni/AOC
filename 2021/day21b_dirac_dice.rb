N_WAYS_TO_ROLL = ([(1..3).to_a] * 3)
  .reduce(&:product)
  .map(&:flatten)
  .map(&:sum)
  .tally
def n_ways_to_roll? dice_sum; N_WAYS_TO_ROLL[dice_sum] end

# For each player, the board is seen as a list of hashes where the key is
# the score and the value is the number of occurrences of that score in that
# square of the board (the square is indicated by the index in the board list).
boards = [:player1, :player2].map { |k| [k, 10.times.map { Hash[] }] }.to_h
for _ in 2.times
  player, pos = gets.match(/Player (\d) starting position: (\d+)/).captures
  boards[:"player#{player}"][pos.to_i - 1][0] = 1
end

wins = { :player1 => 0, :player2 => 0 }
turn = :player1

# A little fun with Ruby :)
class Symbol
  INVERT = [:player1, :player2].zip([:player2, :player1]).to_h
  def ~; INVERT[self] end
end

# We are only done when all possible games are finished
while !boards.values.all? { |b| b.all?(&:empty?) }
  new_board = 10.times.map { Hash[] }
  for from in 0...10
    for dice_sum in 3..9
      to = (from + dice_sum) % 10
      scores = boards[turn]
        .at(from)
        .transform_keys    { |score| score + to + 1                    }
        .transform_values! { |occur| occur * n_ways_to_roll?(dice_sum) }

      new_board[to].merge!(scores) { |_, old, new| old + new }
      new_board[to].delete_if do |score, occur|
        # Note that `delete_if` block is expected to return a boolean. If, however, no
        # expression is evaluated it will return `nil` which is a falsy value. If it is
        # it will return a number, which is truthy.
        wins[turn] += occur * boards[~turn].lazy.flat_map(&:values).sum if score >= 21
      end
    end
  end
  boards[turn] = new_board
  turn = ~turn
end

puts wins.values.max
