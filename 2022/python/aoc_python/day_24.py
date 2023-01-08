from aoc_python.utils import Grid2, Point2, load_stripped_lines


def parse(lines: list[str]) -> tuple[Grid2, dict[int, Point2], tuple[Point2, ...], Point2, Point2]:
    w = len(lines[0])
    h = len(lines)
    grid = Grid2.filled_with(w, h, ".")
    bliz_loc: list[Point2] = []
    bliz_dir = {}
    for y, line in enumerate(lines):
        for x, char in enumerate(line):
            grid[x, y] = char
            if char == ">":
                bliz_dir[len(bliz_loc)] = Point2(1, 0)
            elif char == "<":
                bliz_dir[len(bliz_loc)] = Point2(-1, 0)
            elif char == "^":
                bliz_dir[len(bliz_loc)] = Point2(0, -1)
            elif char == "v":
                bliz_dir[len(bliz_loc)] = Point2(0, 1)
            if char in (">", "<", "^", "v"):
                bliz_loc.append(Point2(x, y))
    start, goal = None, None
    for x in range(w):
        if grid[x, 0] == ".":
            start = Point2(x, 0)
    for x in range(w):
        if grid[x, h - 1] == ".":
            goal = Point2(x, h - 1)
    assert start is not None
    assert goal is not None
    return grid, bliz_dir, tuple(bliz_loc), start, goal


def simulate_blizzard(location: Point2, direction: Point2, grid: Grid2) -> Point2:
    next_location = location + direction
    while not grid.has(next_location) or grid[next_location] == "#":
        tmp_next_location = next_location + direction
        next_location = Point2(tmp_next_location.x % grid.width, tmp_next_location.y % grid.height)
    return next_location


def bfs_in_time(
    start_time: int,
    start: Point2,
    goal: Point2,
    bliz_env: list[set[Point2]],
    bliz_history: dict[int, tuple[Point2, ...]],
    bliz_dir: dict[int, Point2],
    grid: Grid2,
) -> tuple[tuple[Point2, ...]]:
    start_state = (start, start_time)

    visited = {start_state}
    bfs_queue = [start_state]
    predecessors: dict[tuple[Point2, int], tuple[Point2, int] | None] = {start_state: None}

    while bfs_queue:
        current_state = bfs_queue.pop(0)
        current, current_time = current_state

        if current == goal:
            print(f"found goal at time {current_time=}")
            path = []
            while current_state is not None:
                path.append(current_state)
                current_state = predecessors[current_state]
            return tuple(reversed([p[0] for p in path]))

        for adjacent in grid.four_neighbors(current) + (current,):
            adjacent_time = current_time + 1
            if adjacent_time >= len(bliz_env):
                bliz_loc = tuple(
                    simulate_blizzard(location, bliz_dir[i], grid)
                    for i, location in enumerate(bliz_history[current_time])
                )
                bliz_history[adjacent_time] = bliz_loc
                bliz_env.append(set(bliz_loc))
                print(f"simulating bliz for time {adjacent_time=}")
            if grid[adjacent] == "#" or adjacent in bliz_env[adjacent_time]:
                continue
            adjacent_state = (adjacent, adjacent_time)
            if adjacent_state not in visited:
                visited.add(adjacent_state)
                predecessors[adjacent_state] = current_state
                bfs_queue.append(adjacent_state)

    raise ValueError("empty bfs queue")


def solve_first(path: str) -> int:
    grid, bliz_dir, bliz_loc, start, goal = parse(load_stripped_lines(path))
    bliz_env = [set(bliz_loc)]
    bliz_history = {0: bliz_loc}
    p = bfs_in_time(0, start, goal, bliz_env, bliz_history, bliz_dir, grid)
    print(f"{p=}")
    return len(p) - 1


def solve_second(path: str) -> int:
    grid, bliz_dir, bliz_loc, start, goal = parse(load_stripped_lines(path))
    bliz_env = [set(bliz_loc)]
    bliz_history = {0: bliz_loc}
    path_a = bfs_in_time(0, start, goal, bliz_env, bliz_history, bliz_dir, grid)
    print(f"{path_a=}, {len(path_a)=}")
    path_b = bfs_in_time(len(path_a) - 1, goal, start, bliz_env, bliz_history, bliz_dir, grid)
    print(f"{path_b=}, {len(path_b)=}")
    path_c = bfs_in_time(len(path_a) - 1 + len(path_b) - 1, start, goal, bliz_env, bliz_history, bliz_dir, grid)
    print(f"{path_c=}, {len(path_c)=}")
    return len(path_a) + len(path_b) + len(path_c) - 3


def test_parse() -> None:
    grid, bliz_dir, bliz_loc, start, goal = parse(load_stripped_lines("inputs/24_0"))
    assert grid.width == 8
    assert grid.height == 6
    assert grid[1, 1] == ">"
    assert len(bliz_loc) == 19
    assert len(bliz_dir) == 19
    assert bliz_dir[18] == Point2(1, 0)
    assert bliz_loc[18] == Point2(6, 4)
    assert start == Point2(1, 0)
    assert goal == Point2(6, 5)


if __name__ == "__main__":
    test_parse()
    assert solve_first("inputs/24_0") == 18
    assert solve_second("inputs/24_0") == 54
    print(solve_second("inputs/24_1"))
