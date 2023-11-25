import curses
from aoc_python.colloquium import (
    animate,
    parse_grid_problem,
    reconstruct_path,
    viz_open,
    viz_open_collection,
    viz_path,
    viz_predecessors,
    viz_start_goal,
)
from aoc_python.utils import Grid2, Point2, initialize_curses_colors, Color


def dfs(start, goal, grid):
    visited = {start}
    predecessors = {start: None}
    open = [start]

    while open:
        current = open.pop()

        if current == goal:
            break

        for adjacent in grid.four_neighbors(current):
            if adjacent not in visited and grid[adjacent] != "#":
                visited.add(adjacent)
                open.append(adjacent)
                predecessors[adjacent] = current

    return reconstruct_path(goal, predecessors)


def dfs_viz(screen: curses.window, start: Point2, goal: Point2, grid: Grid2) -> list[Point2]:
    initialize_curses_colors()
    curses.curs_set(0)

    visited = {start}
    predecessors: dict[Point2, Point2 | None] = {start: None}
    open = [start]

    for p, v in grid:
        screen.addstr(p.y, p.x, v)

    while open:
        current = open.pop()
        viz_predecessors(screen, predecessors)
        viz_open(screen, open)
        viz_open_collection(screen, open, grid.width + 1)
        viz_start_goal(screen, start, goal)
        screen.addstr(current.y, current.x, "x", curses.color_pair(Color.MAGENTA))
        animate(screen)

        if current == goal:
            break

        for adjacent in grid.four_neighbors(current):
            if adjacent not in visited and grid[adjacent] != "#":
                visited.add(adjacent)
                open.append(adjacent)
                predecessors[adjacent] = current
                viz_predecessors(screen, predecessors)
                viz_open(screen, open)
                viz_open_collection(screen, open, grid.width + 1)
                viz_start_goal(screen, start, goal)
                screen.addstr(current.y, current.x, "x", curses.color_pair(Color.MAGENTA))
                screen.addstr(adjacent.y, adjacent.x, "x", curses.color_pair(Color.CYAN))
                animate(screen)

    path = reconstruct_path(goal, predecessors)
    screen.addstr(
        grid.height + 1,
        0,
        f"path length: {len(path) - 1}, closed states: {len(predecessors)}, open states: {len(open)}",
    )
    viz_path(screen, path)
    return path


if __name__ == "__main__":
    _grid, _start, _goal = parse_grid_problem("aoc_python/colloquium/problem_0")
    _path = curses.wrapper(dfs_viz, _start, _goal, _grid)
    print(f"{len(_path)=}")
