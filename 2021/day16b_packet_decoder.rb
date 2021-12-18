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
  def min(other) = [self, other].min
  def max(other) = [self, other].max
end

class TrueClass
  def to_i = 1
end

class FalseClass
  def to_i = 0
end

TYPE_ID_TO_OP = {
  0 => :+,
  1 => :*,
  2 => :min,
  3 => :max,
  4 => nil,
  5 => :>,
  6 => :<,
  7 => :==,
}

bits = gets.to_i(16).to_bits

def parse_packet bits
  version = bits.consume 3
  type_id = bits.consume 3

  case type_id
  when 4
    num = 0
    loop do
      group_header = bits.consume 1
      num = (num << 4) | bits.consume(4)
      return num if group_header == 0
    end
  else
    length_type_id = bits.consume 1
    len = length_type_id == 0 ? bits.consume(15) : bits.consume(11)
    packets = []
    if length_type_id == 0
      start = bits.pos
      while bits.pos - start < len
        packets << parse_packet(bits)
      end
    else
      for _ in 0...len
        packets << parse_packet(bits)
      end
    end
    return packets.reduce(TYPE_ID_TO_OP[type_id]).to_i
  end
end

puts parse_packet bits
