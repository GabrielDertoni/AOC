
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

# Parse a single "<x=-1, y=0, z=2>" like object
function Base.parse(::Type{Vec3D}, s::AbstractString)
    # Remove angle brackets, split by comma and for each of the coordinates
    # remove the "x=", "y=" or "z=" from the beginning
    coords = map(v -> v[3:end], split(s[begin+1:end-1], ", "))
    Vec3D((parse.(Int, coords))...)
end

function apply_gravity!(pos, vels)
    len = length(vels)
    for i in 1:len-1, j in i+1:len
        # Iterate through every coordinate and ajust velocities
        for c in 1:3
            if pos[i][c] > pos[j][c]
                vels[i][c] -= 1
                vels[j][c] += 1
            elseif pos[i][c] < pos[j][c]
                vels[i][c] += 1
                vels[j][c] -= 1
            end
        end
    end
end

function apply_velocity!(pos, vels)
    for (p, v) in zip(pos, vels)
        p.x += v.x
        p.y += v.y
        p.z += v.z
    end
end

function calculate_energy(pos, vels)
    # Using lots of generators, but I think this might be really efficient
    sum(sum(abs(c) for c in p) * sum(abs(c) for c in v) for (p, v) in zip(pos, vels))
end

function main()
    n_steps = 1000

    coords = parse.(Vec3D, readlines())
    vels = copy.(repeat([Vec3D(0, 0, 0)], length(coords)))
    #       ^--- copy is required, otherwise only a reference to the vec will be
    #       replicated

    for _ in 1:n_steps
        apply_gravity!(coords, vels)
        apply_velocity!(coords, vels)
    end

    println("The total energy of the system is $(calculate_energy(coords, vels))")
end

main()
