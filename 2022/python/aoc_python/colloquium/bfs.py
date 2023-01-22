import curses
import time
from aoc_python.colloquium import parse_problem, reconstruct_path

from aoc_python.utils import Grid2, Point2, initialize_curses_colors, Color


def bfs(start, goal, grid):
    visited = {start}
    predecessors = {start: None}
    open = [start]

    while open:
        current = open.pop(0)

        if current == goal:
            break

        for adjacent in grid.four_neighbors(current):
            if adjacent not in visited and grid[adjacent] != "#":
                visited.add(adjacent)
                open.append(adjacent)
                predecessors[adjacent] = current

    return reconstruct_path(goal, predecessors)


def bfs_viz(screen: curses.window, start: Point2, goal: Point2, grid: Grid2) -> list[Point2]:
    initialize_curses_colors()
    curses.curs_set(0)

    visited = {start}
    predecessors: dict[Point2, Point2 | None] = {start: None}
    open = [start]

    for p, v in grid:
        screen.addstr(p.y, p.x, v)

    while open:
        for v in visited:
            screen.addstr(v.y, v.x, ".", curses.color_pair(Color.YELLOW))

        for o in open:
            screen.addstr(o.y, o.x, "o", curses.color_pair(Color.YELLOW))

        current = open.pop(0)
        screen.addstr(current.y, current.x, "x", curses.color_pair(Color.MAGENTA))
        screen.addstr(start.y, start.x, "S", curses.color_pair(Color.GREEN))
        screen.addstr(goal.y, goal.x, "G", curses.color_pair(Color.GREEN))
        try:
            k = screen.getkey()
            if k == "r":
                screen.nodelay(True)
            if k == "s":
                screen.nodelay(False)
        except (TypeError, curses.error):
            time.sleep(0.01)

        if current == goal:
            break

        for adjacent in grid.four_neighbors(current):
            if adjacent not in visited and grid[adjacent] != "#":
                visited.add(adjacent)
                open.append(adjacent)
                predecessors[adjacent] = current

    path = reconstruct_path(goal, predecessors)

    screen.nodelay(False)
    for p in path:
        screen.addstr(p.y, p.x, "p", curses.color_pair(5))
    screen.getkey()
    return path


if __name__ == "__main__":
    _grid, _start, _goal = parse_problem("aoc_python/colloquium/problem_0")
    _path = curses.wrapper(bfs_viz, _start, _goal, _grid)
    print(f"{len(_path)=}")
