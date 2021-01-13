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

function run_op!(op_idx, inst)
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
            inst[arg1 + 1] = parse(Int, readline())
        else # write
            out = arg(inst, op_idx, 1)
            print("$out ")
        end
        Advance(2)
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

function run_prg!(memory)
    idx = 1
    while true
        retr = run_op!(idx, memory)
        if typeof(retr) <: Advance
            idx += retr.steps
        elseif typeof(retr) <: Jump
            idx = retr.addr
        else
            break # Its a halt!
        end
    end
end

function main()
    numbers = parse.(Int, split(readline(), ","))
    run_prg!(numbers)
end

main()
