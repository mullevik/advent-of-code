from aoc_python.utils import Grid2, Point2, load_stripped_lines


def parse_problem(path: str) -> tuple[Grid2, Point2, Point2]:
    lines = load_stripped_lines(path)
    w = len(lines[0])
    h = len(lines)
    grid = Grid2.filled_with(w, h, ".")

    start, goal = Point2(-1, -1), Point2(-1, -1)
    for y, line in enumerate(lines):
        for x, char in enumerate(line):
            if char == "G":
                goal = Point2(x, y)
            elif char == "S":
                start = Point2(x, y)
            else:
                grid[x, y] = char
    return grid, start, goal


def reconstruct_path(goal: Point2, predecessors: dict[Point2, Point2 | None]) -> list[Point2]:
    path = []
    current: Point2 | None = goal
    while current is not None:
        path.append(current)
        current = predecessors[current]
    return list(reversed(path))
