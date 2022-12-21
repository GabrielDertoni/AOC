import sys

def scan_height(iterable):
    highest = -1
    for height in iterable:
        if height > highest:
            highest = height
            yield True
        else:
            yield False

matrix = []
for line in sys.stdin:
    matrix.append(list(map(int, line.strip())))

width = len(matrix)
height = len(matrix[0])

visible_vertical_rows = []
for row in matrix:
    visible_left = scan_height(row)
    visible_right = reversed(list(scan_height(reversed(row))))
    visible_vertical = [left or right for left, right in zip(visible_left, visible_right)]
    visible_vertical_rows.append(visible_vertical)

visible_horizontal_cols = []
for j in range(width):
    visible_top = scan_height(row[j] for row in matrix)
    visible_bot = reversed(list(scan_height(row[j] for row in reversed(matrix))))
    visible_horizontal = [top or bot for top, bot in zip(visible_top, visible_bot)]
    visible_horizontal_cols.append(visible_horizontal)

count = 0
for i in range(width):
    for j in range(height):
        if visible_vertical_rows[j][i] or visible_horizontal_cols[i][j]:
            count += 1

print(count)
