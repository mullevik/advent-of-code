import numpy as np

from aoc_python.utils import Grid2, Point2, load_stripped_lines


def parse(lines: list[str]) -> Grid2:
    grid = Grid2.filled_with(len(lines[0]), len(lines), 0)
    for y, line in enumerate(lines):
        for x, chr in enumerate(line):
            grid[x, y] = int(chr)
    return grid


def cast_ray(trees: Grid2, visibility: Grid2, start: Point2, direction: Point2) -> None:
    max_height = 0
    visibility[start] = True
    p = start
    while visibility.has(p):
        height = trees[p]
        if height > max_height:
            visibility[p] = True
            max_height = height
        p = p + direction


def solve_first(path: str) -> int:
    lines = load_stripped_lines(path)
    trees = parse(lines)
    visibility = Grid2.filled_with(trees.width, trees.height, False)

    for y in range(trees.height):
        cast_ray(trees, visibility, Point2(0, y), Point2(1, 0))
        cast_ray(trees, visibility, Point2(trees.width - 1, y), Point2(-1, 0))

    for x in range(trees.width):
        cast_ray(trees, visibility, Point2(x, 0), Point2(0, 1))
        cast_ray(trees, visibility, Point2(x, trees.height - 1), Point2(0, -1))

    return sum(cell for row in visibility.cells for cell in row)


def scenic_score(trees: Grid2, p: Point2) -> int:
    visibility = Grid2.filled_with(trees.width, trees.height, False)
    for direction in Point2(0, 0).four_neighbors:
        start = p + direction
        if visibility.has(start):
            cast_ray(trees, visibility, p + direction, direction)
    return sum(cell for row in visibility.cells for cell in row)


def solve_second(path: str) -> int:
    lines = load_stripped_lines(path)
    trees = parse(lines)
    return max(scenic_score(trees, p) for p, _ in trees)


if __name__ == "__main__":
    assert solve_first("inputs/08_0") == 21
    assert solve_second("inputs/08_0") == 8
