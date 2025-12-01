#INPUT = "C:\\Users\\floyd\\aoc\\2024\\day6\\input.txt"
INPUT = "C:\\Users\\floyd\\aoc\\2024\\day6\\example.txt"

from typing import Tuple
from copy import deepcopy

data: list[list[str]] = []

dx = [-1, 0, 1, 0]
dy = [0, 1, 0, -1]

with open(INPUT, 'r') as f:
    for line in f:
        data.append(list(line.strip()))

def is_valid(x: int, y: int) -> bool:
    return x >= 0 and x < len(data) and y >= 0 and y < len(data[0])

x = 0
y = 0
dir = 0 # 0 = up, 1 = right, 2 = down, 3 = left

for i in range(len(data)):
    for j in range(len(data[i])):
        if data[i][j] == '^':
            x, y = i, j

def walk(layout: list[list[int]], cur_x: int, cur_y: int, dir: int) -> Tuple[list[Tuple[int, int]], bool]:
    # we return a list of visited locations and a boolean indicating if we hit a loop
    locations = [(cur_x, cur_y)]
    loop = False
    history: list[Tuple[int, int, int]] = [(cur_x, cur_y, dir)]
    while True:
        next_x, next_y = cur_x + dx[dir], cur_y + dy[dir]

        if not is_valid(next_x, next_y):
            break

        if not (next_x, next_y) in locations:
            locations.append( (next_x, next_y))

        # look one more step ahead
        next2_x, next2_y = next_x + dx[dir], next_y + dy[dir]
        if is_valid(next2_x, next2_y) and layout[next2_x][next2_y] == '#':
            dir = (dir + 1) % 4

        cur_x, cur_y = next_x, next_y
        if (cur_x, cur_y, dir) in history:
            # loop detected
            loop = True
            break
        history.append((cur_x, cur_y, dir))
    return (locations, loop)
    


possible_locations = []
for i in range(len(data)):
    for j in range(len(data[i])):
        if data[i][j] != '#' and data[i][j] != '^':
            possible_locations.append((i, j))


result_part1 = len(walk(data, x, y, dir)[0])

def part2(layout: list[list[int]], start_x: int, start_y: int, dir: int) -> int:
    loops: set[tuple[int, int]] = set()
    tried: set[tuple[int, int]] = set()
    paths, _ = walk(layout, start_x, start_y, dir)
    while True:
        loop_size = len(loops)
        tried_size = len(tried)
        for path in paths[:]:  # Create a copy of paths to iterate over
            if path in tried:
                continue
            tried.add(path)
            new_paths, new_loop = walk(layout, path[0], path[1], dir)
            if new_loop:
                loops.add(path)
            paths.extend(new_paths)
        if len(loops) == loop_size and len(tried) == tried_size:
            break
    return len(loops)


result_part2 = part2(data, x, y, dir)

print("Part 1:", result_part1)
print("Part 2:", result_part2)

# for i, row in enumerate(locations):
#     for j, cell in enumerate(row):
#         if data[i][j] == '#':
#             print('#', end='')
#         elif cell > 0:
#             print('X', end='')
#         else:
#             print(".", end='')
#     print()

