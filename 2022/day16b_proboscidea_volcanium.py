import sys, re
from collections import defaultdict
from itertools import product

connections = dict()
flow_rates = dict()
nonzero = set()
for line in sys.stdin:
    line = line.strip()
    match = re.match(r"^Valve (\w{2}) has flow rate=(\d+); tunnels? leads? to valves? ((\w{2})(, \w{2})*)$", line)
    flow_rate = int(match.group(2))
    flow_rates[match.group(1)] = flow_rate
    connections[match.group(1)] = match.group(3).split(", ")
    if flow_rate > 0:
        nonzero.add(match.group(1))


n = len(connections)
distances = defaultdict(lambda: n + 1)
for this, conns in connections.items():
    distances[this, this] = 0
    for conn in conns:
        distances[this, conn] = 1

for k in connections.keys():
    for i in connections.keys():
        for j in connections.keys():
            distances[i, j] = min(distances[i, j], distances[i, k] + distances[k, j])

def calculate_upper_bound(nonzero, minutes_lefts):
    minutes_left = max(minutes_lefts)
    released = 0
    to_open = list(reversed(sorted(nonzero)))
    for i in range(0, len(to_open), 2):
        # Two rounds, one to go to the `valve` and one to open it
        minutes_left -= 2
        if minutes_left <= 0: break
        released += flow_rates[to_open[i    ]] * minutes_left
        if i + 1 < len(to_open):
            released += flow_rates[to_open[i + 1]] * minutes_left

    return released

all_time_best = 0

# All that matters now is choosing a permutation of `nonzero` that maximizes the total pressure
# released
def branch_and_bound(ats, nonzero, lower_bound, released, minutes_lefts, depth=0):
    global all_time_best

    if all(map(lambda left: left == 0, minutes_lefts)) or len(nonzero) == 0:
        return released

    upper_bound = released + calculate_upper_bound(nonzero, minutes_lefts)
    # If we can't do better than the lower bound we already have, there is no point in continuing
    if upper_bound <= lower_bound:
        return upper_bound

    best = max(lower_bound, released)
    for valve in nonzero:
        for i in range(len(ats)):
            minutes_after = minutes_lefts.copy()
            minutes_after[i] = minutes_after[i] - distances[ats[i], valve] - 1
            if minutes_after[i] <= 0:
                continue
            released_after = released + flow_rates[valve] * minutes_after[i]

            ats_ = ats.copy()
            ats_[i] = valve
            best = max(best, branch_and_bound(ats_, nonzero - {valve}, best, released_after, minutes_after, depth=depth+1))

    if best > all_time_best:
        all_time_best = best
        print(all_time_best)

    return best

print(branch_and_bound(["AA", "AA"], nonzero, 0, 0, [26, 26]))
