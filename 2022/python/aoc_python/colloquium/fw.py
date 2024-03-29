import copy
import curses
import time
import math
from aoc_python.colloquium import animate, parse_general_problem, render_network, build_distance_matrix, ALPHABET
from aoc_python.utils import Point2, initialize_curses_colors, Color


def fw(vertices, distance_matrix):
    for k in range(0, len(vertices)):
        for i in range(0, len(vertices)):
            for j in range(0, len(vertices)):
                if distance_matrix[i][j] > distance_matrix[i][k] + distance_matrix[k][j]:
                    distance_matrix[i][j] = distance_matrix[i][k] + distance_matrix[k][j]
    return distance_matrix


def fw_viz(screen: curses.window, vertices: list[Point2], distance_matrix: list[list[float]]) -> None:
    initialize_curses_colors()
    curses.curs_set(0)

    distances = copy.deepcopy(distance_matrix)
    render_distance_matrix(screen, distances)
    render_network(screen, vertices, distance_matrix)
    animate(screen)

    for k in range(0, len(vertices)):
        for i in range(0, len(vertices)):
            for j in range(0, len(vertices)):
                render_network(screen, vertices, distance_matrix)
                screen.addstr(0, 30, f"{ALPHABET[i]} to {ALPHABET[j]} using shortest path to {ALPHABET[k]}")
                screen.clrtoeol()
                screen.addstr(1, 30, f"is {distances[i][j]} > ({distances[i][k]} + {distances[k][j]})?")
                screen.clrtoeol()
                screen.refresh()
                screen.addstr(vertices[i].y, vertices[i].x, ALPHABET[i], curses.color_pair(Color.YELLOW))
                screen.addstr(vertices[j].y, vertices[j].x, ALPHABET[j], curses.color_pair(Color.YELLOW))
                screen.addstr(vertices[k].y, vertices[k].x, ALPHABET[k], curses.color_pair(Color.CYAN))
                time.sleep(0.05)
                if distances[i][j] > distances[i][k] + distances[k][j]:
                    render_distance_matrix(screen, distances)
                    screen.addstr(i + 2, j * 4 + 2, render_float(distances[i][j]), curses.color_pair(Color.YELLOW))
                    animate(screen)
                    distances[i][j] = distances[i][k] + distances[k][j]
                    screen.addstr(i + 2, j * 4 + 2, render_float(distances[i][j]), curses.color_pair(Color.GREEN))
                    animate(screen)


def render_float(f: float) -> str:
    if math.isinf(f):
        return "inf"
    if f >= 10.0 or f < 0:
        return f" {f:.0f}"
    return f"  {f:.0f}"


def render_distance_matrix(screen: curses.window, distance_matrix: list[list[float]]) -> None:
    screen.addstr(0, 0, "  " + "|".join([f" {ALPHABET[i]} " for i in range(len(distance_matrix))]))
    screen.addstr(1, 1, "----" * len(distance_matrix))

    for row_idx, row in enumerate(distance_matrix):
        row_str = "|".join([render_float(d) for d in row])
        screen.addstr(row_idx + 2, 0, f"{ALPHABET[row_idx]}|{row_str}")


if __name__ == "__main__":
    _vertices, _edges = parse_general_problem("aoc_python/colloquium/problem_2")
    _distance_matrix = build_distance_matrix(_vertices, _edges)
    curses.wrapper(fw_viz, _vertices, _distance_matrix)
