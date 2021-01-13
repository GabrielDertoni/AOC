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

function main()
    numbers = parse.(Int, split(readline(), ","))
    idx = 1
    while run_op!(idx, numbers) == advance
        idx += 4
    end
    println("The value at position 0 is $(numbers[1])")
end

main()
