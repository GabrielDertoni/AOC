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

function operate_robot(code)
    painted = Dict{Tuple{Int, Int}, Color}()

    pos = (0, 0)
    dir = (0, 1)
    state = ProgState(copy(code), 1, 0, [], [])

    while true
        # Inputs the color of the tile the robot is in
        push!(state.input, get(painted, pos, black))

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
    end

    return painted
end

function main()
    code = parse.(Int, split(readline(), ","))
    painted = operate_robot(code)
    println("There were $(length(painted)) tiles painted at least once")
end

main()



