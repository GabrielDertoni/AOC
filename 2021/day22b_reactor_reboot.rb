class Range
  # More efficient count for integer range
  def count
    first.is_a?(Integer) && last.is_a?(Integer) ? last - first + 1 : to_enum.count
  end
end

DisjointUnion = Struct.new :subshapes
class DisjointUnion
  def volume; subshapes.sum(&:volume) end
  def empty?; subshapes.all? { |sub| sub.is_a? DisjointUnion and sub.empty? } end
  def | cuboid
    new = self - cuboid
    new.subshapes << cuboid
    return new
  end
  def - cuboid
    DisjointUnion.new subshapes.flat_map { |sub| sub - cuboid }
  end
end

Cuboid = Struct.new :xs, :ys, :zs
class Cuboid
  def volume; xs.count * ys.count * zs.count end
  def & other
    return nil if not other
    def axis_intersection this, other
      [this.first, other.first].max .. [this.last, other.last].min
    end
    inter_xs = axis_intersection xs, other.xs
    inter_ys = axis_intersection ys, other.ys
    inter_zs = axis_intersection zs, other.zs
    return nil if inter_xs.none? or inter_ys.none? or inter_zs.none?
    Cuboid.new(inter_xs, inter_ys, inter_zs)
  end
  def - other
    inter = self & other
    return self if not inter
    [
      Cuboid.new(inter.xs.last + 1 .. xs.last, ys, zs),
      Cuboid.new(xs.first .. inter.xs.first - 1, ys, zs),
      Cuboid.new(inter.xs, inter.ys.last + 1 .. ys.last, zs),
      Cuboid.new(inter.xs, ys.first .. inter.ys.first - 1, zs),
      Cuboid.new(inter.xs, inter.ys, inter.zs.last + 1 .. zs.last),
      Cuboid.new(inter.xs, inter.ys, zs.first .. inter.zs.first - 1),
    ].filter { |cube| cube.volume > 0 }
  end
end

litup = DisjointUnion.new []
while gets =~ /(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)/
  cuboid = Cuboid.new($2.to_i..$3.to_i, $4.to_i..$5.to_i, $6.to_i..$7.to_i)
  litup |= cuboid if $1 == "on"
  litup -= cuboid if $1 == "off"
end

puts "#{litup.volume}"
