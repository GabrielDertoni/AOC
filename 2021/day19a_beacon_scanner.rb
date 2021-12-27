require 'set'
require 'numo/narray'
# This was a pain to get working... Might not work in different systems.
require 'numo/linalg/linalg'
Numo::Linalg::Loader.load_openblas '/usr/local/opt/openblas/lib'
require 'pp'

include Numo

scanners = Hash[]
while gets =~ /--- scanner (\d+) ---/
  scanner = $1.to_i
  scanners[scanner] = []
  while gets =~ /^(-?\d+),(-?\d+),(-?\d+)$/
    x, y, z = $1.to_i, $2.to_i, $3.to_i
    scanners[scanner] << [x, y, z]
  end
end

def dist p1, p2
  p1.zip(p2).map { |l, r| (l - r).abs() }.sum
end

distances = scanners.transform_values do |beacons|
  beacons.map do |row|
    beacons.map { |col| dist(row, col) }.to_set
  end
end

n = scanners.length

def stack_ones narray
  NArray.hstack [narray, DFloat.ones([narray.shape[0], 1])]
end

all_beacons = scanners[0].to_set

# Scanners yet to be processed
todos = scanners.keys.to_set
todos.delete 0

while not todos.empty?
  # Cycle through scanners. If we fail to solve one of them, we can just try a different one
  # and come back later.
  for curr_scanner in scanners.keys
    next if not todos.include? curr_scanner
    dists_a = all_beacons.map { |row| all_beacons.map { |col| dist(row, col) }.to_set }
    dists_b = distances[curr_scanner]

    # No matter what is the coordinate system of the beacon, the same beacon will always
    # be the same distance away from every other beacon. So we can take a guess to try
    # to find beacons that are actually the same. We iterate through every pair of beacons
    # each detected by a scanner and check which distances to other beacons they measure.
    # We choose the 4 beacons that are most alike and then we have 4 points in the coordinate
    # system of `curr_scanner`.

    best_guess = (0...dists_a.length).to_a.product((0...dists_b.length).to_a)
      .max_by(4) { |i, j| (dists_a[i] & dists_b[j]).size }

    pts_scanner_a = stack_ones DFloat[*all_beacons]
    pts_scanner_b = stack_ones DFloat[*scanners[curr_scanner]]

    # To figure out the coordinate system of `curr_scanner` relative to the "absolute position"
    # which is set to be scanner 0, we must solve a linear system. A transformation matrix is
    # always of the form:
    #
    # ```
    # [[ x1 y1 z1 0]
    #  [ x2 y2 z2 0]
    #  [ x3 y3 z3 0]
    #  [ xo yo zo 1]]
    # ```
    #
    # where [x1, y1, z1], [x2, y2, z2] and [x3, y3, z3] are the vectors of the basis of the
    # coordinate system of `curr_scanner` and [xo, yo, zo] is the origin of the coordinate system
    # (the position of `curr_scanner` itself. So let M be thhe trasformation matrix just mentioned,
    # then we know that the points that matched in `curr_scanner` must map to the corresponding
    # points, so just solve the linear equation! (we pad with 1s in the end of each vector because
    # it is required for this sort of transformation matrix to work)

    # b x = a
    b = pts_scanner_b[best_guess.map(&:last), true]
    a = pts_scanner_a[best_guess.map(&:first), true]
    scanner_b_to_a = Linalg.solve(b, a).round

    # Transform the rest of the points from scanner b to absolute position
    transf = Linalg.matmul(pts_scanner_b, scanner_b_to_a)
    transf = Set[*Int64.cast(transf[true, 0..2])]

    inter = transf & all_beacons
    # If there are enough points in common, we have a hit! Mark this scanner as done and add the
    # beacons to the set.
    if inter.size >= 12
      all_beacons |= transf
      todos.delete curr_scanner
    end
  end
end

puts all_beacons.size
