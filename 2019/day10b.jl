using Base.Iterators

struct Position
    x::Int
    y::Int
end

Base.:-(a::Position, b::Position) = Position(a.x - b.x, a.y - b.y)
Base.:+(a::Position, b::Position) = Position(a.x + b.x, a.y + b.y)

# Just a type to serve as a handle
struct PrimeSieve end

# Just a wrapper function so syntax doesnt look too weird
@inline primes() = PrimeSieve()

Base.iterate(sieve::PrimeSieve) = (2, [2])
function Base.iterate(sieve::PrimeSieve, computed)
    for i in countfrom(last(computed))
        if !any(c -> i % c == 0, computed)
            push!(computed, i)
            return (i, computed)
        end
    end
    @assert false "There is always a next prime!"
end

# This should be infinite, but this is as close as we can get.
Base.length(sieve::PrimeSieve) = typemax(Int)

function read_asteroids()
    asteroids = Set{Position}()
    for (y, line) in zip(countfrom(0), readlines())
        for (x, c) in zip(countfrom(0), line)
            if c == '#'
                push!(asteroids, Position(x, y))
            end
        end
    end
    asteroids
end

function simplify_fraction(num, den)
    if num == 0
        return (0, den / abs(den))
    elseif den == 0
        return (num / abs(num), 0)
    end

    for prime in primes()
        while num % prime == 0 && den % prime == 0
            num = div(num, prime)
            den = div(den, prime)
        end

        if abs(num) < prime || abs(den) < prime
            break
        end
    end

    return (num, den)
end

function is_visible(pos, other, asteroids)
    # An asteroid cannot see itself
    if pos == other
        return false
    end

    delta = other - pos

    # By moving from pos by dx and dy we get to the cells exactly in the
    # direction delta.
    dy, dx = simplify_fraction(delta.y, delta.x)
    for step in countfrom(1)
        p = pos + Position(step * dx, step * dy)

        if p ∈ asteroids
            if p == other
                return true
            else
                return false
            end
        end
    end
end
count_visible(pos, asts) = count(a -> is_visible(pos, a, asts), asts)

manhattan_dist(a, b) = abs(a.x - b.x) + abs(a.y - b.y)

# Return an angle in [0, 2π)
abs_ang(ang) = (ang < 0 ? ang + 2π : ang) % 2π

# Adjust the angle so that 0 is pointing up.
adjust_ang(ang) = abs_ang(ang + π/2)

function get_nth_vaporized(asteroids, station_position, n)
    @assert length(asteroids) - 1 >= 200 "There needs to be at least 200 asteroids"

    println("station is at $station_position")

    # All asteroids that are lined up in a certain angle (ratio dy/dx)
    ang_to_asteroids = Dict{Tuple{Int, Int}, Array{Position, 1}}()
    for ast in asteroids
        if ast == station_position
            continue
        end

        delta = ast - station_position
        dy, dx = simplify_fraction(delta.y, delta.x)

        lined_up = get!(ang_to_asteroids, (dy, dx), [])
        push!(lined_up, ast)
    end

    # Sort each of the lined up asteroids by their distance to the station in
    # reversed order. This is because pop!() removes from the back and is more
    # efficient than popfirst!()
    for lined_up in values(ang_to_asteroids)
        sort!(lined_up, by=el -> manhattan_dist(el, station_position), rev=true)
    end

    # Sort all all deltas by their respective angles. This is done in such a way
    # that angle 0 points up, π/2 points right and so on.
    all_angles = collect(keys(ang_to_asteroids))
    sort!(all_angles, by=((dy, dx),) -> adjust_ang(atan(dy, dx)))

    # The last vaporized asteroid
    vaporized = nothing
    count = 0
    ang_idx = 1
    while count < n
        lined_up = get(ang_to_asteroids, all_angles[ang_idx], [])

        if length(lined_up) > 0
            # Vaporize the asteroid!!!
            vaporized = pop!(lined_up)
            count += 1
        end

        # Continue rotating
        ang_idx = ang_idx <= length(all_angles) ? ang_idx + 1 : 1
    end

    return vaporized
end

function main()
    asteroids = read_asteroids()
    counts = [(pos, count_visible(pos, asteroids)) for pos in asteroids]
    max_ast_idx = argmax([count for (_, count) in counts])
    max_ast = counts[max_ast_idx][1]

    nth = 200
    vaporized = get_nth_vaporized(asteroids, max_ast, nth)
    ans = vaporized.x * 100 + vaporized.y
    println("The asteroid at position $vaporized was the $(nth)th asteroid to be vaporized!")
    println("Puzzle anwer is $ans")
end

main()

