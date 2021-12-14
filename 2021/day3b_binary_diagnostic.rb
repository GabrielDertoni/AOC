require 'set'

o2 = ARGF.each_line.map(&:strip).to_set
co2 = o2.clone
for i in (0..)
  count1 = o2.map{ |n| n[i].to_i }.sum
  bit1 = count1 >= o2.length / 2.0 ? 1 : 0
  count0 = co2.map{ |n| n[i].to_i }.sum
  bit0 = count0 >= co2.length / 2.0 ? 0 : 1
  o2.delete_if { |n| n[i].to_i != bit1 } if o2.length > 1
  co2.delete_if { |n| n[i].to_i != bit0 } if co2.length > 1
  break if o2.length == 1 and co2.length == 1
end
o2 = o2.to_a[0].to_i(2)
co2 = co2.to_a[0].to_i(2)
puts o2 * co2
