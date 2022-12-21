import sys

def iter_seen(height, iter_heights):
    count = 0
    for h in iter_heights:
        count += 1
        if h >= height:
            break
    return count

matrix = []
for line in sys.stdin:
    matrix.append(list(map(int, line.strip())))

width = len(matrix)
height = len(matrix[0])

best = 0
for i in range(height):
    for j in range(width):
        h = matrix[i][j]

        right = iter_seen(h, (matrix[i][x] for x in range(j-1, -1, -1)))
        left  = iter_seen(h, (matrix[i][x] for x in range(j+1, width)))
        top   = iter_seen(h, (matrix[y][j] for y in range(i-1, -1, -1)))
        bot   = iter_seen(h, (matrix[y][j] for y in range(i+1, height)))

        score = right * left * top * bot
        if score > best:
            best = score

print(best)

