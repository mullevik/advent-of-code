import curses
import time
import math
from typing import Any
from aoc_python.utils import Color, Grid2, Point2, load_stripped_lines, brehemsen_line

ALPHABET = "ABCDEFGHIJK"


def parse_grid_problem(path: str) -> tuple[Grid2, Point2, Point2]:
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


def parse_general_problem(path: str) -> tuple[list[Point2], dict[tuple[int, int], int]]:
    lines = load_stripped_lines(path)
    n_vertices, n_edges = lines[0].split(" ")
    vertex_lines = lines[1 : 1 + int(n_vertices)]
    edge_lines = lines[1 + int(n_vertices) : 1 + int(n_vertices) + int(n_edges)]
    points = []
    for line in vertex_lines:
        x, y = line.split(",")
        points.append(Point2(int(x), int(y)))
    edges = {}
    for line in edge_lines:
        u, v, d = line.split(",")
        edges[(int(u), int(v))] = int(d)
        edges[(int(v), int(u))] = int(d)
    return points, edges


def reconstruct_path(goal: Point2, predecessors: dict[Point2, Point2 | None]) -> list[Point2]:
    path = []
    current: Point2 | None = goal
    while current is not None:
        path.append(current)
        current = predecessors[current]
    return list(reversed(path))


def viz_start_goal(screen: curses.window, start: Point2, goal: Point2) -> None:
    screen.addstr(start.y, start.x, "S", curses.color_pair(Color.GREEN))
    screen.addstr(goal.y, goal.x, "G", curses.color_pair(Color.GREEN))


def viz_predecessors(screen: curses.window, predecessors: dict[Point2, Point2 | None]) -> None:
    for p in predecessors.keys():
        screen.addstr(p.y, p.x, ".", curses.color_pair(Color.YELLOW))


def viz_open(screen: curses.window, open: list[Point2]) -> None:
    for o in open:
        screen.addstr(o.y, o.x, "o", curses.color_pair(Color.YELLOW))


def viz_open_collection(screen: curses.window, open: list[Any], x: int) -> None:
    for y in range(22):
        screen.move(y, x)
        screen.clrtoeol()
    for y, item in enumerate(open):
        if y > 20:
            screen.addstr(y, x, f"... (and {len(open) - 20} more)")
            break
        else:
            screen.addstr(y, x, str(item))


def viz_path(screen: curses.window, path: list[Point2]) -> None:
    screen.nodelay(False)
    for p in path:
        screen.addstr(p.y, p.x, "p", curses.color_pair(Color.MAGENTA))
    screen.getkey()


def animate(screen: curses.window, delay: float = 0.01) -> None:
    try:
        k = screen.getkey()
        if k == "r":
            screen.nodelay(True)
        if k == "s":
            screen.nodelay(False)
    except (TypeError, curses.error):
        time.sleep(delay)


def render_network(screen: curses.window, vertices: list[Point2], distance_matrix: list[list[float]]) -> None:
    visited: set[frozenset[int]] = set()
    for i, row in enumerate(distance_matrix):
        for j, d in enumerate(row):

            if math.isinf(d):
                continue

            if {i, j} in visited:
                continue
            visited.add(frozenset({i, j}))

            line = brehemsen_line(vertices[i], vertices[j])

            for p in line:
                screen.addstr(p.y, p.x, ".")

            mid_point = line[len(line) // 2] if len(line) > 0 else vertices[i]
            screen.addstr(mid_point.y, mid_point.x, f"{d:.0f}")

    for i, v in enumerate(vertices):
        screen.addstr(v.y, v.x, ALPHABET[i])


def build_distance_matrix(vertices: list[Point2], edges: dict[tuple[int, int], int]) -> list[list[float]]:
    distances = [[float("inf") for _ in range(len(vertices))] for _ in range(len(vertices))]
    print(distances)
    for i, v in enumerate(vertices):
        for j, u in enumerate(vertices):
            if i == j:
                distances[i][j] = 0.0
            if (i, j) in edges:
                distances[i][j] = float(edges[(i, j)])

    return distances
