
function module_fuel(mass)
    fuel = div(mass, 3) - 2
    fuel <= 0 ? 0 : fuel + module_fuel(fuel)
end

total = sum(module_fuel.(parse.(Int,readlines())))
println("The total fuel needed is $total")

