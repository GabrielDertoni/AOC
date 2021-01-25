abstract type OpReturn end
struct Halt <: OpReturn end
struct Advance <: OpReturn
    steps::Int
end
struct Jump <: OpReturn
    addr::Int
end
mutable struct ProgState
    memory::Array{Int64,1}
    ptr::Int64
    relative_base::Int64
    input::Array{Int64,1} # Its like the stdin of the program
    output::Array{Int64,1} # Like the stdout
end

memget(memory::Array{Int64,1}, addr) = addr <= length(memory) ? memory[addr] : 0

function memwrite!(memory::Array{Int64,1}, addr, val)
    if addr <= length(memory)
        memory[addr] = val
    else
        # Append as many zeroes as needed until address. Hopefully we won't need
        # tons of memory to do that on the puzzle input.
        append!(memory, repeat([0], addr - length(memory)))
        memory[addr] = val
    end
end

argmode(instruction, off) = div(instruction, 10 * 10^off) % 10

function arg(state, off)
    inst = memget(state.memory, state.ptr)
    addr_or_val = memget(state.memory, state.ptr + off)
    mode = argmode(inst, off)
    if mode == 0
        memget(state.memory, addr_or_val + 1)
    elseif mode == 1
        addr_or_val
    else
        memget(state.memory, addr_or_val + state.relative_base + 1)
    end
end

# Gets an address argument. This function is used to get arguments that are ment
# to serve as write addresses, thus mode 1 is disallowed.
function addr_arg(state, off)
    inst = memget(state.memory, state.ptr)
    addr = memget(state.memory, state.ptr + off)
    mode = argmode(inst, off)
    if mode == 0
        addr + 1
    elseif mode == 2
        addr + 1 + state.relative_base
    else
        error("An address argument cannot have mode $mode")
    end
end

function run_op!(state)
    op_idx = state.ptr
    inst = state.memory

    instruction = memget(inst, op_idx)
    op = instruction % 100
    if op ∈ [1, 2] # Arith instructions
        arg1 = arg(state, 1)
        arg2 = arg(state, 2)
        arg3 = addr_arg(state, 3)

        memwrite!(inst, arg3, op == 1 ?  arg1 + arg2 : arg1 * arg2)
        Advance(4)
    elseif op ∈ [3, 4] # IO instructions
        if op == 3 # read
            arg1 = addr_arg(state, 1)
            memwrite!(inst, arg1, popfirst!(state.input))
            Advance(2)
        else # write
            arg1 = arg(state, 1)
            push!(state.output, arg1)
            Advance(2)
        end
    elseif op ∈ [5, 6] # Jump instructions
        arg1 = arg(state, 1)
        arg2 = arg(state, 2)

        if op == 5 && arg1 != 0 # jump-if-true
            Jump(arg2 + 1)
        elseif op == 6 && arg1 == 0 # jump-if-false
            Jump(arg2 + 1)
        else
            Advance(3)
        end
    elseif op ∈ [7, 8] # Comparison instructions
        arg1 = arg(state, 1)
        arg2 = arg(state, 2)
        arg3 = addr_arg(state, 3)

        w = if op == 7 # less than
                arg1 < arg2 ? 1 : 0
            else # equals
                arg1 == arg2 ? 1 : 0
            end

        memwrite!(inst, arg3, w)
        Advance(4)
    elseif op == 9 # Adjust relative base
        arg1 = arg(state, 1)
        state.relative_base += arg1
        Advance(2)
    elseif op == 99
        Halt()
    else
        error("An op code should be 1, 2, 3, 4, 5, 6, 7, 8, 9 or 99, but got $op")
    end
end

abstract type Color end
struct White <: Color end
struct Black <: Color end

white = White() # Singleton instance of White
black = Black() # Singleton instance of Black

Base.convert(::Type{Int}, x::Black) = 0
Base.convert(::Type{Int}, x::White) = 1
function Base.convert(::Type{Color}, x::Int)
    if x == 0
        black
    elseif x == 1
        white
    else
        error("Expected value 0 or 1 in conversion from Int to Color, but got $x")
    end
end

# Runs the program untill it halts or it outputs something
function next_output!(state)::Union{Nothing, Int}
    while true
        retr = run_op!(state)

        if typeof(retr) <: Advance
            state.ptr += retr.steps
        elseif typeof(retr) <: Jump
            state.ptr = retr.addr
        else
            return nothing # Its a halt!
        end

        if length(state.output) > 0
            return pop!(state.output)
        end
    end
end

function rotate_dir_left(dir)
    if     dir == ( 1,  0); ( 0,  1)
    elseif dir == ( 0,  1); (-1,  0)
    elseif dir == (-1,  0); ( 0, -1)
    elseif dir == ( 0, -1); ( 1,  0)
    else;  error("Invalid direction $dir")
    end
end

function rotate_dir_right(dir)
    if     dir == ( 1,  0); ( 0, -1)
    elseif dir == ( 0, -1); (-1,  0)
    elseif dir == (-1,  0); ( 0,  1)
    elseif dir == ( 0,  1); ( 1,  0)
    else;  error("Invalid direction $dir")
    end
end

function operate_robot(code; animate=false)
    painted = Dict{Tuple{Int, Int}, Color}()

    pos = (0, 0)
    dir = (0, 1)
    state = ProgState(copy(code), 1, 0, [white], [])
    #         the first input is white ---^
    # Using "white" here is fine because convert(::Type{Int}, x::Color)
    # is defined and so convert(::Type{Array{Int,1}}, x::Array{Color,1})
    # is also defined.

    while true
        # Get 2 outputs
        color, rot = [next_output!(state) for _ in 1:2]

        # Detect if the program halted
        if color == nothing || rot == nothing
            break
        end

        painted[pos] = color

        @assert rot ∈ [0, 1] "Expected rotation value to be 0 or 1, but got $rot"

        dir = rot == 0 ? rotate_dir_left(dir) : rotate_dir_right(dir)
        pos = (pos[1] + dir[1], pos[2] + dir[2])

        # Inputs the color of the tile the robot is in
        push!(state.input, get(painted, pos, black))

        if animate
            # Clear the screen and put the cursor at (0, 0)
            print("\033[J\033[H")
            print_painted(painted)
            sleep(0.1)
        end
    end

    if animate
        print("\033[J\033[H")
    end
    return painted
end

function print_painted(painted)
    whites = [pos for (pos, color) in painted if color == white]
    if length(whites) == 0
        return
    end
    min_x, min_y = reduce((p1, p2) -> min.(p1, p2), whites)
    #                                    ^--- broadcast is really smart and will
    #                                    call (min(p1[1], p2[1]), min(p1[2], p2[2]))

    # Adjust positions to be 0 based
    whites = map(pos -> (pos[1] - min_x, pos[2] - min_y), whites)
    max_x, max_y = reduce((p1, p2) -> max.(p1, p2), whites)

    # The matrix that will hold the caracters to construct the output image.
    char_image = repeat([' '], max_y + 1, max_x + 1)

    for (j, i) in whites
        # Y dimension has to be inverted because it should increase going up,
        # but matrices have smaller indices going up.
        char_image[max_y - i + 1, j + 1] = '█'
    end
    lines = map(r -> string(r...), eachrow(char_image))
    str = join(lines, "\n")
    println(str)
end

function main()
    code = parse.(Int, split(readline(), ","))
    painted = operate_robot(code, animate=true)
    print_painted(painted)
end

main()




