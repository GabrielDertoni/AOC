class Bitstream
  attr_accessor :pos

  def initialize num
    @bits = num.to_s(2)
    @pos = 0
  end

  def consume n
    v = @bits[...n].to_i(2)
    @bits = @bits[n...]
    @pos += n
    v
  end
end

class Integer
  def to_bits = Bitstream.new self
end

def parse_packet bits
  version = bits.consume 3
  type_id = bits.consume 3
  version_sum = version

  case type_id
  when 4
    loop do
      group_header = bits.consume 1
      bits.consume(4)
      break if group_header == 0
    end
  else
    length_type_id = bits.consume 1
    len = length_type_id == 0 ? bits.consume(15) : bits.consume(11)
    if length_type_id == 0
      start = bits.pos
      while bits.pos - start < len
        version_sum += parse_packet bits
      end
    else
      for _ in 0...len
        version_sum += parse_packet bits
      end
    end
  end
  return version_sum
end

puts parse_packet gets.to_i(16).to_bits
