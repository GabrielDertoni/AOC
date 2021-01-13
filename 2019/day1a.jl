total = sum((div.(parse.(Int,readlines()),3)).-2)
println("The total fuel needed is $total")

# Code breakdown:
# ===============
#
# readlines() gets a collection of all lines from stdin
# parse.(Int, ...) reads each line as an Int
# div.(..., 3) performes integer division by 3 on the collection
# (...).-2 subtracts 2 from each item
# sum(...) sums all elements in the collection
#
