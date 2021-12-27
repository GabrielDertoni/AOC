player1, player2 = nil, nil
for _ in 1..2
  player, pos = gets.match(/Player (\d) starting position: (\d+)/).captures
  eval "player#{player} = #{pos}"
end

dice = (1..100).lazy.cycle
p1_score = 0
p2_score = 0
p1_pos = (1..10).lazy.cycle.drop player1
p2_pos = (1..10).lazy.cycle.drop player2

rolls = 0
is_p1_turn = true

while p1_score < 1000 and p2_score < 1000
  move = 3.times.lazy.map { dice.next }.sum
  rolls += 3
  p1_score += move.times.map { p1_pos.next }.last if is_p1_turn
  p2_score += move.times.map { p2_pos.next }.last if not is_p1_turn
  is_p1_turn = !is_p1_turn
end

puts rolls * (p1_score >= 1000 ? p2_score : p1_score)
