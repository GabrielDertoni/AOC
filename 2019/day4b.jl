function groups(s)
    groupings = String[]
    acc = string(s[1])
    for c in s[2:end]
        if c == acc[1]
            acc *= c
        else
            push!(groupings, acc)
            acc = string(c)
        end
    end
    push!(groupings, acc)
    return groupings
end

function check_num(n)
    s = string(n)
    gs = groups(s)
    if !any(p -> length(p) == 2, gs)
        false
    else
        all(p -> p[1] <= p[2], [gs[i:i+1] for i in 1:length(gs)-1])
    end
end

function main()
    (low, high) = parse.(Int, split(readline(), "-"))
    # Still dumb as hell, but it still works :)
    count = 0
    for i in low:high
        if check_num(i)
            count += 1
        end
    end
    println("The final number count is $count")
end

@assert check_num(112233)
@assert !check_num(123444)
@assert check_num(111122)
main()

