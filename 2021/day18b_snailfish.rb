nums = ARGF.map { |line| eval line.strip }

# Turn on to see a nice visualization of the solution
DEBUG = false

def format nums, rights
  trav = lambda do |n, rs, color|
    if n.is_a? Integer
      if color == :default and rights.zip(rs).all? { |l, r| l == r }
        return "\e[33m#{n}\e[0m"
      else
        return "#{n}"
      end
    else
      if color == :default and rights.zip(rs).all? { |l, r| l == r }
        left  = trav.call n[0], rs + [false], :yellow
        right = trav.call n[1], rs + [true] , :yellow
        return "\e[33m[#{left},#{right}]\e[0m"
      else
        left  = trav.call n[0], rs + [false], color
        right = trav.call n[1], rs + [true] , color
        return "[#{left},#{right}]"
      end
    end
  end
  return trav.call nums, [], :default
end

# This is more of an experimentation with sort of "continuation passing style"
# type of thing than a function! `num` is the snailfish number, `depth` is the
# bracket depth, `all` is just a reference to the top level snailfish number
# and is used only for debugging. `rights` is also for debugging, and it keeps
# a boolean list of `true` where the path chose to go right (to the right of
# the pair) or `false` where it went left. `update`, `ladd` (left add) and
# `radd` (right add) are lambdas. `update` changes the value in `num` in the
# array from the caller, mutating it. `ladd` adds a value to the value to the
# left of the current pair and `radd` to the value to the right.
def explode num, depth = 0, update = nil, ladd = nil, radd = nil, all = nil, rights = []
  all = num if not all
  return false if num.is_a? Integer
  if depth < 4
    # Functions that traverse the tree to find the leftmost and rightmost value. When
    # it is found, it reconstructs the tree with the target value updated (plus `val).
    lupdate  = lambda { |n, val| n.is_a?(Integer) ? n + val : [lupdate.call(n[0], val), n[1]] }
    rupdate  = lambda { |n, val| n.is_a?(Integer) ? n + val : [n[0], rupdate.call(n[1], val)] }

    # A curried function of the "index assign" operator. The first argument is the
    # index and the second is the value.
    idx_op = num.method(:[]=).curry(2)
    putleft, putright = idx_op.(0), idx_op.(1)

    # These functions are called when the value up the callstack wants to update the
    # value to the right or left.
    next_radd = lambda { |val| putright.call lupdate.call num[1], val }
    next_ladd = lambda { |val| putleft.call rupdate.call num[0], val }

    return (explode num[0], depth + 1, putleft , ladd, next_radd, all, rights + [false] \
         or explode num[1], depth + 1, putright, next_ladd, radd, all, rights + [true])
  else
    puts "[💥]EXPLODE #{format all, rights}" if DEBUG
    left, right = num
    update.(0)
    ladd.(left) if ladd
    radd.(right) if radd
    return true
  end
end

# Similar to `explode`
def split num, depth = 0, update = nil, all = nil, rights = []
  all = num if not all
  if num.is_a? Integer
    return false if num < 10
    puts "[🔪]SPLIT   #{format all, rights}" if DEBUG
    update.([num / 2, (num + 1) / 2])
    return true
  else
    idx_op = num.method(:[]=).curry(2)
    putleft, putright = idx_op.(0), idx_op.(1)
    return (split num[0], depth + 1, putleft , all, rights + [false] \
         or split num[1], depth + 1, putright, all, rights + [true])
  end
end

def reduce num
  continue = true
  while continue
    while explode num; end
    continue = split num
  end
end

def mag num
  return num if num.is_a? Integer
  return 3 * mag(num[0]) + 2 * mag(num[1])
end

class Array
  def deep_dup
    arry = []
    for el in self
      if el.is_a? Array
        arry << el.deep_dup
      else
        arry << el.dup
      end
    end
    arry
  end
end

combinations = nums.product(nums)
biggest = combinations.lazy
  .filter { |lhs, rhs| lhs != rhs }
  .zip(1..)
  .map do |(lhs, rhs), i|
    print "\r#{i}/#{combinations.length}" if i % 100 == 0
    mag(([lhs.deep_dup] + [rhs.deep_dup]).tap(&method(:reduce)))
  end
  .max

puts "\nbiggest = #{biggest}"
