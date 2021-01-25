#=
# The solution presented in day12a.jl is not efficient enough for this part of
# the problem.
#
# This approach takes advantage of the fact that different dimensions don't
# affect each other: all changes in the x value of the position or velocity
# is only affected by other x coordinates. Because of this, each dimension may
# be simulated independently. On its own, this fact is not enough to optimize
# the simulation. In order to do that we must first note some other facts:
#
#   1. The function of applying gravity and then velocity is surjective, that is,
#      some state leads to another and that relationship is unique. We could just
#      as easily make an inverse function that reverses the process without any
#      ambiguity.
#
#   2. Because of [1], if there is a loop, it has to loop back to the initial
#      state. If that was not the case, there would need to be two states that
#      lead to the same loop connection state which is not possible.
#
#   3. Each dimension may have different loop sizes. For example, looking only
#      at the x values of position and velocity of the moons, after a certain
#      number of iterations they will go back to the initial state. From that
#      point on, it will loop. Y and Z dimensions, however, will have different
#      loop sizes.
#
# Finally, we realize that the entire system will loop once the iteration number
# is a multiple of the loop size of X, Y, and Z. Therefore, the entire system
# will first loop in the LCM of those three values.
#
# If we are a bit optimistic and consider that each of the three loop sizes is
# about the same, say "l", but they have GCD 1, the final loop size would bee in
# the order of l^3. Thus, to find a loop of size "n", we need about n^(1/3).
# In other words, this solution has time efficient of roughly O(n^(1/3)).
#
# The problem states that we should expect loop sizes on the order of 4686774924
# or bigger, and 4686774924^(1/3) ≈ 1673.5. So, it should be fast enough.
=#

mutable struct Vec3D
    x::Int
    y::Int
    z::Int
end

Base.copy(v::Vec3D) = Vec3D(v.x, v.y, v.z)

Base.:+(a::Vec3D, b::Vec3D) = Vec3D(a.x + b.x, a.y + b.y, a.z + b.z)
Base.:-(a::Vec3D, b::Vec3D) = Vec3D(a.x - b.x, a.y - b.y, a.z - b.z)

# Iterate over dimensions
Base.iterate(v::Vec3D) = (v.x, 2)
Base.iterate(v::Vec3D, state) = state ∈ [1, 2, 3] ? (v[state], state + 1) : nothing
Base.length(v::Vec3D) = 3

Base.getindex(v::Vec3D, i) = (v.x, v.y, v.z)[i]
function Base.setindex!(v::Vec3D, val::Int, i::Int)
    if     i == 1; v.x = val
    elseif i == 2; v.y = val
    elseif i == 3; v.z = val
    else;  error("Vec3D index should be ∈ [1, 2, 3]")
    end
end

Base.hash(v::Vec3D, h::UInt) = hash((v.x, v.y, v.z), h)

# Parse a single "<x=-1, y=0, z=2>" like object
function Base.parse(::Type{Vec3D}, s::AbstractString)
    # Remove angle brackets, split by comma and for each of the coordinates
    # remove the "x=", "y=" or "z=" from the beginning
    coords = map(v -> v[3:end], split(s[begin+1:end-1], ", "))
    Vec3D((parse.(Int, coords))...)
end

function apply_gravity!(pos, vels)
    len = length(pos)
    for i in 1:len-1, j in i+1:len
        vels[i] += sign(pos[j] - pos[i])
        vels[j] += sign(pos[i] - pos[j])
    end
end

function apply_velocity!(pos, vels)
    len = length(pos)
    for i in 1:len
        pos[i] += vels[i]
    end
end

function find_loop(pos, vels)
    starting = (copy(pos), copy(vels))

    iters = 0
    while true
        apply_gravity!(pos, vels)
        apply_velocity!(pos, vels)
        iters += 1

        if (pos, vels) == starting
            break
        end
    end
    return iters
end

function main()
    coords = parse.(Vec3D, readlines())
    vels = copy.(repeat([Vec3D(0, 0, 0)], length(coords)))
    #       ^--- copy is required, otherwise only a reference to the vec will be
    #       replicated

    pos_coords = collect.(zip(coords...))
    vels_coords = collect.(zip(vels...))

    iters = 1
    for c in 1:3
        retr = find_loop(pos_coords[c], vels_coords[c])
        println("Dimension $c had loop of size $retr")
        iters = lcm(iters, retr)
    end
    println("# of iterations until a full repeat = $iters")
end

main()

