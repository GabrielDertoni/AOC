segment_signals = Hash[
  "abcefg"  => 0,
  "cf"      => 1,
  "acdeg"   => 2,
  "acdfg"   => 3,
  "bcdf"    => 4,
  "abdfg"   => 5,
  "abdefg"  => 6,
  "acf"     => 7,
  "abcdefg" => 8,
  "abcdfg"  => 9,
]

total = 0

ARGF.each_line do |line|
  ins, outs = line.split("|").map(&:split)

  # This is a brute force solution, we just go through every possible
  # permutation and choose one that is sensible.

  mapping = ('a'..'g').to_a.permutation.lazy
    # Create a mapping from the permutation. A map from input wires to output signals
    .map { |perm| ('a'..'g').zip(perm).to_h }
    # Filter only mappings that produce valid sets of signals.
    .filter { |mapping|
      ins.all? do |inp|
        decoded = inp.chars.map { |c| mapping[c] }.sort!.join
        segment_signals.include? decoded
      end
    }
    .first

  total += outs
    .map { |out| out.chars.map { |c| mapping[c] }.sort!.join }
    .map { |out| segment_signals[out] }.join.to_i
end

puts total
