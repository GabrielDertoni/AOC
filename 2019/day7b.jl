abstract type OpReturn end
struct Halt <: OpReturn end
struct Advance <: OpReturn
    steps::Int
end
struct Jump <: OpReturn
    addr::Int
end
mutable struct ProgState
    memory::Array{Int,1}
    ptr::Int
    input::Array{Int,1} # Its like the stdin of the program
    output::Array{Int,1} # Like the stdout
end

function arg_mode(instruction, off)
    div(instruction, 10 * 10^off) % 10 == 1
end

function arg(instructions, op_idx, off)
    addr_or_val = instructions[op_idx + off]
    if arg_mode(instructions[op_idx], off)
        addr_or_val
    else
        instructions[addr_or_val + 1]
    end
end

function run_op!(state)
    op_idx = state.ptr
    inst = state.memory

    instruction = inst[op_idx]
    op = instruction % 100
    if op ∈ [1, 2] # Arith instructions
        arg1 = arg(inst, op_idx, 1)
        arg2 = arg(inst, op_idx, 2)
        arg3 = inst[op_idx + 3]

        inst[arg3 + 1] = op == 1 ?  arg1 + arg2 : arg1 * arg2
        Advance(4)
    elseif op ∈ [3, 4] # IO instructions
        if op == 3 # read
            arg1 = inst[op_idx + 1]
            inst[arg1 + 1] = popfirst!(state.input)
            Advance(2)
        else # write
            out = arg(inst, op_idx, 1)
            push!(state.output, out)
            Advance(2)
        end
    elseif op ∈ [5, 6] # Jump instructions
        arg1 = arg(inst, op_idx, 1)
        arg2 = arg(inst, op_idx, 2)

        if op == 5 && arg1 != 0 # jump-if-true
            Jump(arg2 + 1)
        elseif op == 6 && arg1 == 0 # jump-if-false
            Jump(arg2 + 1)
        else
            Advance(3)
        end
    elseif op ∈ [7, 8] # Comparison instructions
        arg1 = arg(inst, op_idx, 1)
        arg2 = arg(inst, op_idx, 2)
        arg3 = inst[op_idx + 3]

        inst[arg3 + 1] = if op == 7 # less than
                             arg1 < arg2 ? 1 : 0
                         else # equals
                             arg1 == arg2 ? 1 : 0
                         end
        Advance(4)
    elseif op == 99
        Halt()
    else
        error("An op code should be 1, 2, 3, 4, 5, 6, 7, 8 or 99, but got $op")
    end
end

# Runs the program untill it halts or it outputs something
function run_prg!(state)
    while true
        retr = run_op!(state)

        if typeof(retr) <: Advance
            state.ptr += retr.steps
        elseif typeof(retr) <: Jump
            state.ptr = retr.addr
        else
            break # Its a halt!
        end

        if length(state.output) > 0
            return pop!(state.output)
        end
    end
    return nothing
end

# I could get away with less type annotations. But I am trying to learn it :)
function permutations( to_perm::Array{T,1}
                     , curr_perm::Array{T,1} = Array{T,1}()
                     )::Array{T,2} where {T}
    perms = Array{T,2}[]
    if length(to_perm) > 0
        for element in to_perm
            next_perm = push!(copy(curr_perm), element)
            remaining_elements = filter(el -> el != element, to_perm)
            perms = cat(perms, permutations(remaining_elements, next_perm), dims=1)
        end
        return perms
    else
        return curr_perm'
    end
end

function run_amplifier(phase_settings, code)
    modules = [copy(code) for _ in phase_settings]

    # Initial states of the amplifiers. Each input stream starts only with
    # the corresponding phase setting
    states = []
    for (s, m) in zip(phase_settings, modules)
        push!(states, ProgState(m, 1, [s], []))
    end

    prev_out = 0
    halted = false
    while !halted
        for st in states
            push!(st.input, prev_out)
            retr = run_prg!(st)

            if retr != nothing
                prev_out = retr
            else
                halted = true
                break
            end
        end
    end
    return prev_out
end

function main()
    code = parse.(Int, split(readline(), ","))
    perms = permutations(collect(5:9))

    max_i = 0
    max_out = 0
    for (i, phase_settings) in enumerate(eachrow(perms))
        out = run_amplifier(phase_settings, code)
        if max_out < out
            max_out = out
            max_i = i
        end
    end
    println("The best configuration was $(perms[max_i, :]) with output $max_out")
end

main()

