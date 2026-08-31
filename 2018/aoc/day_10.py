import re
from collections.abc import Iterable
from math import floor

from aoc.day_06 import manhattan_dist

type vec2 = tuple[int, int]


def p1(inp: str) -> str:
    return str(simulate_all(list(parse(inp))))


def p2(inp: str) -> str:
    return str(simulate_all(list(parse(inp))))


RESOLUTION_WIDTH = 100
RESOLUTION_HEIGHT = 50
SLOW_DOWN_CONST = 100
SLOW_DOWN_MULTIPLIER = 8.0


def simulate_all(points: list[tuple[vec2, vec2]]) -> int:
    n_seconds = 0
    min_avg_dist = avg_dist(points)
    while True:
        avg_dst = avg_dist(points)
        multiplier = max(
            1, int(floor(avg_dst / SLOW_DOWN_MULTIPLIER)) - SLOW_DOWN_CONST
        )

        if avg_dst > min_avg_dist:
            points = list(simulate(points, -1))
            print(f"After {n_seconds - 1=} {avg_dst=}")
            display(points)
            return n_seconds - 1

        n_seconds += 1 * multiplier
        min_avg_dist = min(avg_dst, min_avg_dist)
        points = list(simulate(points, multiplier))


def avg_dist(points: list[tuple[vec2, vec2]]) -> float:
    total = 0
    n = 0
    for p, _ in points:
        for other, _ in points:
            total += manhattan_dist(p, other)
            n += 1
    return total / n


def display(points: list[tuple[vec2, vec2]]) -> None:

    min_x = min(p[0][0] for p in points)
    min_y = min(p[0][1] for p in points)

    disp = [["." for _ in range(RESOLUTION_WIDTH)] for _ in range(RESOLUTION_HEIGHT)]

    for p in points:
        shifted = (p[0][0] - min_x, p[0][1] - min_y)

        disp_char = "#"
        if shifted[0] >= RESOLUTION_HEIGHT or shifted[1] >= RESOLUTION_WIDTH:
            disp_char = "@"
        disp[min(shifted[1], RESOLUTION_HEIGHT - 1)][
            min(shifted[0], RESOLUTION_WIDTH - 1)
        ] = disp_char

    for row in disp:
        for cell in row:
            print(cell, end="")
        print()


def simulate(
    points: Iterable[tuple[vec2, vec2]], multiplier: int
) -> Iterable[tuple[vec2, vec2]]:

    for p in points:
        x, y = p[0]
        dx, dy = p[1]
        yield ((x + dx * multiplier, y + dy * multiplier), (dx, dy))


def parse(inp: str) -> Iterable[tuple[vec2, vec2]]:

    for line in inp.split("\n"):
        match = re.search(
            r"position=< *(-?\d+), *(-?\d+)> velocity=< *(-?\d+), *(-?\d+)>", line
        )

        if match is not None:
            x = int(match.group(1))
            y = int(match.group(2))
            dx = int(match.group(3))
            dy = int(match.group(4))

            yield ((x, y), (dx, dy))
