
# Generators!! Because this is lazy, no extra memory needs to be allocated
count_digit(data, digit) = sum(i == digit ? 1 : 0 for i in data)

# Argmin doesn't work with generators, so we need to allocate some data to use
# this syntax. But we'll be fine
find_fewest_0_digit_layer(layers) = argmin([count_digit(l, 0) for l in eachcol(layers)])

function main()
    width = 25
    height = 6

    data = parse.(Int, collect(readline()))

    # Generate a matrix where columns are layers. So layered_data[:, 1] is the
    # first layer and layered_data[1, :] is all of the layers in that pixel
    layered_data = reshape(data, height * width, :)

    idx = find_fewest_0_digit_layer(layered_data)
    layer = layered_data[:, idx]
    ans = count_digit(layer, 1) * count_digit(layer, 2)

    println("The layer with index $idx had fewer 0 digits.")
    println("In this layer, the number of 1s times the number of 2s is $ans")
end

main()
