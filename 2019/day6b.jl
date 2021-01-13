function orbit_path(root, a, b, all_orbits, pathlen=0)
    if !(root in keys(all_orbits))
        return 0
    end

    if a in all_orbits[root] || b in all_orbits[root]
        return pathlen
    end

    nfound = 0
    val = 0
    for orbit in all_orbits[root]
        ret = orbit_path(orbit, a, b, all_orbits, pathlen + 1)
        if ret > 0
            nfound += 1
            val += ret
        end
    end
    
    # If both a and b were found, return the final path distance
    if nfound == 2
        return val - nfound * pathlen
    else
        return val
    end
end

function main()
    all_orbits = Dict{String, Array{String, 1}}()
    for line in readlines()
        (orbited, orbits) = split(line, ")", limit=2)
        v = get!(all_orbits, orbited, [])
        push!(v, orbits)
    end
    pathlen = orbit_path("COM", "YOU", "SAN", all_orbits)
    println("The shortest path is $pathlen")
end

main()

