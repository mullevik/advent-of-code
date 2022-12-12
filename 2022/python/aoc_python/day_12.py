from collections import defaultdict
from dataclasses import dataclass
from typing import Callable
from collections.abc import Iterable
import numpy as np
from aoc_python.utils import load_stripped_lines, Grid2, Point


def parse(lines: list[str]) -> tuple[Grid2, Point, Point]:
    h = len(lines)
    w = len(lines[0])
    m = Grid2.filled_with(w, h, 0)
    start = None
    end = None
    for y, line in enumerate(lines):
        for x, chr in enumerate(line):
            if chr == "S":
                m[x, y] = "a"
                start = Point(x, y)
            elif chr == "E":
                m[x, y] = "z"
                end = Point(x, y)
            else:
                m[x, y] = chr

    if start is None or end is None:
        raise ValueError(f"start is {start} and end is {end}, which is unexpected")
    return (m, start, end)


def go_up(current: str, adjacent: str) -> bool:
    return ord(adjacent) <= ord(current) + 1


def bfs(topo_map: Grid2, start: Point, adjacent_test: Callable[[str, str], bool]) -> dict[Point, int]:
    bfs_queue = [start]
    visited_points = {start}
    distances: dict[Point, int] = {start: 0}
    while bfs_queue:
        current = bfs_queue.pop(0)
        for adjacent in topo_map.four_neightbours(current):
            if adjacent in visited_points:
                continue
            if adjacent_test(topo_map[current], topo_map[adjacent]):
                bfs_queue.append(adjacent)
                visited_points.add(adjacent)
                distances[adjacent] = distances[current] + 1
    return distances


def solve_first(path: str) -> int:
    topo_map, start, end = parse(load_stripped_lines(path))

    distances = bfs(topo_map, start, go_up)

    if end not in distances:
        raise ValueError(f"end {end} not found in distances")
    return distances[end]


def solve_second(path: str) -> int:
    topo_map, _, end = parse(load_stripped_lines(path))
    distances = bfs(topo_map, end, lambda current, adjacent: go_up(adjacent, current))

    return min(d for p, d in distances.items() if topo_map[p] == "a")


def test_parse() -> None:
    m, s, e = parse(load_stripped_lines("inputs/12_0"))
    assert m.width == 8
    assert m.height == 5
    assert s == Point(0, 0)
    assert e == Point(5, 2)


if __name__ == "__main__":
    test_parse()
    assert solve_first("inputs/12_0") == 31
    assert solve_second("inputs/12_0") == 29
    print(solve_second("inputs/12_1"))
