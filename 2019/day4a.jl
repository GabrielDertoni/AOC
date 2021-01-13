function check_num(n)
    s = string(n)
    windows = [s[i:i+1] for i in 1:length(s)-1]
    if !any(p -> p[1] == p[2], windows)
        false
    else
        all(p -> p[1] <= p[2], windows)
    end
end

function main()
    (low, high) = parse.(Int, split(readline(), "-"))
    # This is a dumb solution, but the range is quite small, so we'll be fine
    count = 0
    for i in low:high
        if check_num(i)
            count += 1
        end
    end
    println("The final number count is $count")
end

main()
