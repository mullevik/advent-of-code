from collections import defaultdict
import copy
import curses
from dataclasses import dataclass, field
import heapq
import itertools
import time
import math
from aoc_python.colloquium import animate, parse_general_problem, render_network, build_distance_matrix, ALPHABET
from aoc_python.utils import Point2, initialize_curses_colors, Color


@dataclass(order=True)
class PrioritizedState:
    priority: float
    time_added: int
    vertex_idx: int = field(compare=False)

    def __str__(self) -> str:
        return f"{ALPHABET[self.vertex_idx]} p={self.priority:.0f}, t={self.time_added}"


def _viz_open(screen: curses.window, open: list[PrioritizedState]) -> None:
    screen.addstr(0, 0, ", ".join([f"({ALPHABET[ps.vertex_idx]}, {ps.priority:.0f}, {ps.time_added})" for ps in open]))
    screen.clrtoeol()


def _reconstruct_path(predecessors: dict[int, int | None], goal: int) -> list[int]:
    current = predecessors[goal]
    path = [goal]
    while current is not None:
        path.append(current)
        current = predecessors[current]
    return list(reversed(path))


def dijkstra_viz(
    screen: curses.window, vertices: list[Point2], distance_matrix: list[list[float]], start: int, goal: int
) -> list[int]:
    initialize_curses_colors()
    curses.curs_set(0)
    render_network(screen, vertices, distance_matrix)

    counter = itertools.count()

    predecessors: dict[int, int | None] = {start: None}
    open = [PrioritizedState(0, next(counter), start)]
    distances_from_start: dict[int, float] = defaultdict(lambda: float("inf"))
    distances_from_start[start] = 0

    visited: set[int] = set()

    while open:
        _viz_open(screen, open)
        render_network(screen, vertices, distance_matrix)
        for v in visited:
            screen.addstr(vertices[v].y, vertices[v].x, ALPHABET[v], curses.color_pair(Color.CYAN))
        current_state = heapq.heappop(open)
        current = current_state.vertex_idx
        visited.add(current)
        current_vertex = vertices[current]
        screen.addstr(current_vertex.y, current_vertex.x, ALPHABET[current], curses.color_pair(Color.YELLOW))
        animate(screen)

        if current == goal:
            break

        for adjacent, distance in enumerate(distance_matrix[current]):
            if math.isinf(distance):
                continue

            new_distance = distances_from_start[current] + distance
            if new_distance < distances_from_start[adjacent]:
                predecessors[adjacent] = current
                distances_from_start[adjacent] = new_distance
                heapq.heappush(open, PrioritizedState(new_distance, next(counter), adjacent))

    path = _reconstruct_path(predecessors, goal)
    for v in path:
        screen.addstr(vertices[v].y, vertices[v].x, ALPHABET[v], curses.color_pair(Color.MAGENTA))
    screen.addstr(1, 0, "->".join([ALPHABET[v] for v in path]), curses.color_pair(Color.MAGENTA))
    # animate(screen)
    screen.nodelay(False)
    screen.getkey()
    return path


if __name__ == "__main__":
    _vertices, _edges = parse_general_problem("aoc_python/colloquium/problem_2")
    _distance_matrix = build_distance_matrix(_vertices, _edges)
    print(curses.wrapper(dijkstra_viz, _vertices, _distance_matrix, 1, 5))
