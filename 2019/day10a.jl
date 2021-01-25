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

function main()
    asteroids = read_asteroids()

    counts = [(pos, count_visible(pos, asteroids)) for pos in asteroids]
    max_ast_idx = argmax([count for (_, count) in counts])
    max_ast = counts[max_ast_idx][1]
    max_count = counts[max_ast_idx][2]
    println("The asteroid with more visible asteroids is at $max_ast with $max_count in visible range")
end

main()
