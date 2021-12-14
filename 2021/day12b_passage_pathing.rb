require 'set'

adj = Hash[]

ARGF.each_line do |line|
  from, to = line.strip.split('-')
  adj[from] = adj.fetch(from, []).append to
  adj[to]   = adj.fetch(to  , []).append from
end

class String
  def upper?; /[[:upper:]]/.match? self end
end

def dfs adj, can_visit_twice, curr = "start", path = []
  if curr == "end"
    # If the end was reached, but we haven't passed through the special cave
    # twice, return emtpy since we are already accounting for this path when we
    # calculate without any special caves.
    return [] if can_visit_twice and path.count(can_visit_twice) < 2
    return [path + ["end"]]
  end

  adj[curr]
    .filter   { |cave| cave.upper? or path.count(cave) < (cave == can_visit_twice ? 2 : 1) }
    .flat_map { |conn| dfs adj, can_visit_twice, conn, path + [curr]                       }
end

puts (adj.keys - %w[start end]).grep(/[[:lower:]]/).append(nil)
  .flat_map { |twice| dfs(adj, twice) }
  .count
