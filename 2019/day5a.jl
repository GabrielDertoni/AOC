abstract type OpReturn end
struct Halt <: OpReturn end
struct Advance <: OpReturn
    steps::Int
end

function run_op!(op_idx, inst)
    instruction = inst[op_idx]
    op = instruction % 100
    if op ∈ [1, 2] # Arith instructions
        arg1_mode = div(instruction, 100 ) % 10 == 1
        arg2_mode = div(instruction, 1000) % 10 == 1

        arg1 = arg1_mode ? inst[op_idx + 1] : inst[inst[op_idx + 1] + 1]
        arg2 = arg2_mode ? inst[op_idx + 2] : inst[inst[op_idx + 2] + 1]
        arg3 = inst[op_idx + 3]

        inst[arg3 + 1] = op == 1 ?  arg1 + arg2 : arg1 * arg2
        Advance(4)
    elseif op ∈ [3, 4] # IO instructions
        arg1_mode = div(instruction, 100) % 10 == 1
        arg1 = inst[op_idx + 1]

        if op == 3
            inst[arg1 + 1] = parse(Int, readline())
        else
            out = arg1_mode ? arg1 : inst[arg1 + 1]
            print("$out ")
        end
        Advance(2)
    elseif op == 99
        Halt()
    else
        error("An instruction should be 1, 2, 3, 4, 5, 6, 7, 8 or 99")
    end
end

function run_prg!(memory)
    idx = 1
    while true
        retr = run_op!(idx, memory)
        if typeof(retr) <: Advance
            idx += retr.steps
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


