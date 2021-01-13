# There are better ways to do this, I am sure. But I'd like to experiment a bit
# with multiple dispatch.
abstract type Direction end

struct Right <: Direction
    steps::Int
end

struct Left <: Direction
    steps::Int
end

struct Up <: Direction
    steps::Int
end

struct Down <: Direction
    steps::Int
end

struct Position
    x::Int
    y::Int
end

function parse_direction(s::AbstractString)
    c = s[1]
    if c == 'R'
        Right(parse(Int, s[2:end]))
    elseif c == 'L'
        Left(parse(Int, s[2:end]))
    elseif c == 'U'
        Up(parse(Int, s[2:end]))
    elseif c == 'D'
        Down(parse(Int, s[2:end]))
    else
        error("No direction starts with '$c'")
    end
end

Base.parse(::Type{Direction}, s::AbstractString) = parse_direction(s)

Base.:+(a::Position , b::Right   ) = Position(a.x + b.steps, a.y)
Base.:+(a::Position , b::Left    ) = Position(a.x - b.steps, a.y)
Base.:+(a::Position , b::Up      ) = Position(a.x, a.y + b.steps)
Base.:+(a::Position , b::Down    ) = Position(a.x, a.y - b.steps)
Base.:+(a::Direction, b::Position) = b + a # Commutative property

Base.isequal(a::Position, b::Position) = a.x == b.x && a.y == b.y
Base.hash(a::Position, h::UInt) = hash((a.x, a.y), h)

@enum Wire black=1 red=2 intersection=3

function main()
    # The first wire is the "black" wire
    path1 = parse.(Direction, split(readline(), ","))

    # The second wire is the "red" wire
    path2 = parse.(Direction, split(readline(), ","))

    occupied = Dict{Position, Wire}()

    pos = Position(0, 0)
    for dir in path1
        for _ in 1:dir.steps
            pos += typeof(dir)(1)
            occupied[pos] = black
        end
    end

    pos = Position(0, 0)
    for dir in path2
        for _ in 1:dir.steps
            pos += typeof(dir)(1)
            if get(occupied, pos, red) != red
                occupied[pos] = intersection
            else
                occupied[pos] = red
            end
        end
    end
    
    marked = collect(keys(filter(p -> p.second == intersection, occupied)))
    closest = reduce(min, map(p -> abs(p.x) + abs(p.y), marked))
    println("The closest intersection has distance $closest to (0, 0)")
end

main()
