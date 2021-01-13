
function count_orbiting(from, all_orbits, prev_count=0)
    if !(from in keys(all_orbits))
        return prev_count
    end

    count = prev_count
    for orbit in all_orbits[from]
        count += count_orbiting(orbit, all_orbits, prev_count + 1)
    end
    return count
end

function main()
    all_orbits = Dict{String, Array{String, 1}}()
    for line in readlines()
        (orbited, orbits) = split(line, ")", limit=2)
        v = get!(all_orbits, orbited, [])
        push!(v, orbits)
    end
    total_count = count_orbiting("COM", all_orbits)
    println("The total orbit count is $total_count")
end

main()
