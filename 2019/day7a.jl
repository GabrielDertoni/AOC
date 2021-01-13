abstract type OpReturn end
struct Halt <: OpReturn end
struct Advance <: OpReturn
    steps::Int
end
struct Jump <: OpReturn
    addr::Int
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

# Retuns a tuple: first any output, second the operation return
function run_op!(op_idx, inst, input=0)
    instruction = inst[op_idx]
    op = instruction % 100
    if op ∈ [1, 2] # Arith instructions
        arg1 = arg(inst, op_idx, 1)
        arg2 = arg(inst, op_idx, 2)
        arg3 = inst[op_idx + 3]

        inst[arg3 + 1] = op == 1 ?  arg1 + arg2 : arg1 * arg2
        (nothing, Advance(4))
    elseif op ∈ [3, 4] # IO instructions
        if op == 3 # read
            arg1 = inst[op_idx + 1]
            inst[arg1 + 1] = input
            (nothing, Advance(2))
        else # write
            out = arg(inst, op_idx, 1)
            (out, Advance(2))
        end
    elseif op ∈ [5, 6] # Jump instructions
        arg1 = arg(inst, op_idx, 1)
        arg2 = arg(inst, op_idx, 2)

        if op == 5 && arg1 != 0 # jump-if-true
            (nothing, Jump(arg2 + 1))
        elseif op == 6 && arg1 == 0 # jump-if-false
            (nothing, Jump(arg2 + 1))
        else
            (nothing, Advance(3))
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
        (nothing, Advance(4))
    elseif op == 99
        (nothing, Halt())
    else
        error("An op code should be 1, 2, 3, 4, 5, 6, 7, 8 or 99, but got $op")
    end
end

function run_prg!(memory, input)
    outputs = []
    idx = 1
    while true
        inp = 0
        if memory[idx] % 10 == 3 # Read instruction
            inp = popfirst!(input)
        end

        (out, retr) = run_op!(idx, memory, inp)

        if out != nothing
            push!(outputs, out)
        end

        if typeof(retr) <: Advance
            idx += retr.steps
        elseif typeof(retr) <: Jump
            idx = retr.addr
        else
            break # Its a halt!
        end
    end
    return outputs
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
    prev_out = 0
    for (mod, setting) in zip(modules, phase_settings)
        retr = run_prg!(mod, [setting, prev_out])
        @assert length(retr) == 1
        prev_out = retr[1]
    end
    return prev_out
end

function main()
    code = parse.(Int, split(readline(), ","))
    perms = permutations(collect(0:4))

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
