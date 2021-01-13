@enum OP_RETURN halt=1 advance=2

function run_op!(op_idx, inst)
    op = inst[op_idx]
    if op ∈ [1, 2]
        # In julia, indices start at 1!!
        pos1 = inst[op_idx + 1] + 1
        pos2 = inst[op_idx + 2] + 1
        pos3 = inst[op_idx + 3] + 1
        if op == 1
            inst[pos3] = inst[pos1] + inst[pos2]
        else
            inst[pos3] = inst[pos1] * inst[pos2]
        end
        advance
    elseif op == 99
        halt
    else
        error("An instruction should be 1, 2 or 99")
    end
    
end

function run_prg(memory)
    idx = 1
    while run_op!(idx, memory) == advance
        idx += 4
    end
    memory[1]
end

function solve(numbers, target)
    for i in 0:99, j in 0:99
        numbers[2] = i
        numbers[3] = j
        if run_prg(copy(numbers)) == target
            return (i, j)
        end
    end
    error("No solution was found")
end

function main()
    numbers = parse.(Int, split(readline(), ","))
    noun, verb = solve(numbers, 19690720)
    println("noun = $noun, verb = $verb. Final answer = $(100 * noun + verb)")
end

main()

