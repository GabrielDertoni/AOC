require 'set'

adj = Hash[]

ARGF.each_line do |line|
  from, to = line.strip.split('-')
  adj[from] = adj.fetch(from, []).append to
  adj[to]   = adj.fetch(to  , []).append from
end

def dfs adj, curr = "start", vis = Set[]
  return 1 if curr == "end"
  vis.add curr if /[[:lower:]]/.match? curr
  # Visit all caves connected to `curr` that have not been visited
  total = adj[curr].lazy
    .filter { |cave| not vis.include? cave }
    .map    { |conn| dfs adj, conn, vis    }
    .sum
  vis.delete curr
  return total
end

puts dfs adj
