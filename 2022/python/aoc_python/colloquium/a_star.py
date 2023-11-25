from collections import defaultdict
import curses
from dataclasses import dataclass, field
import itertools
from typing import Any
from aoc_python.colloquium import (
    animate,
    parse_grid_problem,
    reconstruct_path,
    viz_open_collection,
    viz_path,
    viz_predecessors,
    viz_start_goal,
)
import heapq

from aoc_python.utils import Grid2, Point2, initialize_curses_colors, Color


@dataclass(order=True)
class PrioritizedState:
    priority: float
    time_added: int
    location: Point2 = field(compare=False)

    def __str__(self) -> str:
        return f"({self.location.x:>2}, {self.location.y:>2}) p={self.priority:.0f}, t={self.time_added}"


def distance(a: Point2, b: Point2) -> int:
    return heuristic(a, b)


def heuristic(a: Point2, b: Point2) -> int:
    return abs(a.x - b.x) + abs(a.y - b.y)


def _viz_open(screen: curses.window, open: list[PrioritizedState]) -> None:
    for ps in open:
        screen.addstr(ps.location.y, ps.location.x, "o", curses.color_pair(Color.YELLOW))


def a_star_viz(screen: curses.window, start: Point2, goal: Point2, grid: Grid2) -> list[Point2]:
    initialize_curses_colors()
    curses.curs_set(0)
    counter = itertools.count()

    predecessors: dict[Point2, Point2 | None] = {start: None}
    open = [PrioritizedState(0, next(counter), start)]
    distances_from_start: dict[Point2, float] = defaultdict(lambda: float("inf"))
    distances_from_start[start] = 0

    while open:
        current_state = heapq.heappop(open)
        current = current_state.location

        viz_predecessors(screen, predecessors)
        _viz_open(screen, open)
        viz_open_collection(screen, open, grid.width + 1)
        viz_start_goal(screen, start, goal)
        screen.addstr(current.y, current.x, "x", curses.color_pair(Color.MAGENTA))
        animate(screen)

        if current == goal:
            break

        for adjacent in grid.four_neighbors(current):
            if grid[adjacent] == "#":
                screen.addstr(adjacent.y, adjacent.x, "#")
                continue

            g_score = distances_from_start[current] + distance(current, adjacent)
            if g_score < distances_from_start[adjacent]:
                predecessors[adjacent] = current
                distances_from_start[adjacent] = g_score
                f_score = g_score + heuristic(adjacent, goal)
                heapq.heappush(open, PrioritizedState(f_score, next(counter), adjacent))
                viz_predecessors(screen, predecessors)
                _viz_open(screen, open)
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
    _grid, _start, _goal = parse_grid_problem("aoc_python/colloquium/problem_1")
    _path = curses.wrapper(a_star_viz, _start, _goal, _grid)
    print(f"{len(_path)=}")
