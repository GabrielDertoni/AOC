# Merges two layers by zipping each pixel of one image with the other and then
# merging individual pixels with a map()
merge_layers(a, b) = map(((p1, p2),) -> p1 == 2 ? p2 : p1, zip(a, b))
#                                 ^--- this is weird, but the only way it works

function print_image(image)
    char_image = map(p -> p == 1 ? '█' : ' ', image)
    lines = map(r -> string(r...), eachrow(char_image))
    str = join(lines, "\n")
    println(str)
end

function main()
    width = 25
    height = 6

    data = parse.(Int, collect(readline()))

    # Generate a matrix where layered_data[:, :, 1] is all of the pixels in the
    # first layer. However, because of the way reshape works layered_data[:, :, 1]
    # will be the transpose of the image we want.
    image = reshape(data, width, height, :)
    nlayers = size(image)[3]

    # Transpose width with height to get height as the first dimension which will
    # be usefull when printing.
    image = permutedims(image, [2, 1, 3])

    # In order to make reduce work, we create a generator that iterates through
    # all layers of the image
    image = reduce(merge_layers, (image[:, :, i] for i in 1:nlayers))

    print_image(image)
end

main()

