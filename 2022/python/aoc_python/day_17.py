from collections import defaultdict
from fractions import Fraction
import math
from time import sleep
from typing import Iterable, Iterator
from aoc_python.utils import Point2, load_stripped_lines, clear_outputs
from tqdm import tqdm


def parse(lines: list[str]) -> Iterable[Point2]:
    for char in lines[0]:
        if char == ">":
            yield Point2(1, 0)
        elif char == "<":
            yield Point2(-1, 0)
        else:
            ValueError(f"unexpected char {char}")


TILES: list[tuple[Point2, ...]] = [
    (Point2(0, 0), Point2(1, 0), Point2(2, 0), Point2(3, 0)),  #  -
    (Point2(1, 0), Point2(0, 1), Point2(1, 1), Point2(2, 1), Point2(1, 2)),  # +
    (Point2(0, 0), Point2(1, 0), Point2(2, 0), Point2(2, 1), Point2(2, 2)),  # _|
    (Point2(0, 0), Point2(0, 1), Point2(0, 2), Point2(0, 3)),  # |
    (Point2(0, 0), Point2(0, 1), Point2(1, 1), Point2(1, 0)),  # []
]


def generate_tiles(limit: int) -> Iterable[tuple[Point2, ...]]:
    i = 0
    while i < limit:
        yield TILES[i % 5]
        i += 1


def move_tile(tile: tuple[Point2, ...], direction: Point2) -> tuple[Point2, ...]:
    return tuple(p + direction for p in tile)


def is_colliding(tile: tuple[Point2, ...], rocks: set[Point2]) -> bool:
    if min(p.x for p in tile) < 0:
        return True
    if max(p.x for p in tile) > 6:
        return True
    if min(p.y for p in tile) <= 0:
        return True
    return any(p in rocks for p in tile)


def print_state(tile: tuple[Point2, ...], rocks: set[Point2]) -> None:
    min_x, max_x, min_y, max_y = 0, 7, 0, 30
    clear_outputs()
    for y in range(max_y, min_y, -1):
        print(f"{y:<2}| ", end="")
        for x in range(min_x, max_x):
            if Point2(x, y) in rocks:
                print("#", end="")
            elif Point2(x, y) in tile:
                print("@", end="")
            else:
                print(" ", end="")
        print("")
    print("------------")
    sleep(0.1)


def get_rock_heights(rocks: set[Point2]) -> tuple[int, ...]:
    min_x = min(r.x for r in rocks)
    max_x = max(r.x for r in rocks)
    return tuple([max(r.y for r in rocks if r.x == x) for x in range(min_x, max_x + 1)])


def find_period(
    mem_map: dict[tuple[tuple[int, ...], int, tuple[int, ...]], list[tuple[int, int]]]
) -> tuple[int, int, int, int, Fraction, Fraction]:
    (first_x, first_y), (second_x, second_y) = [val for val in mem_map.values() if len(val) >= 2][0][:2]
    dx = second_x - first_x
    dy = second_y - first_y
    slope = Fraction(dy, dx)
    shift = first_y - (slope * first_x)
    return first_x, first_y, dx, dy, slope, shift


def predict_using_periods(
    tile_limit: int,
    mem_map: dict[tuple[tuple[int, ...], int, tuple[int, ...]], list[tuple[int, int]]],
    mem_max_heights: list[int],
) -> int:
    first_x, first_y, dx, dy, slope, shift = find_period(mem_map)
    print(f"{first_x=}, {first_y=}, {dx=}, {dy=}, {slope=}, {shift=}")

    last_full_period_iteration = (((tile_limit - first_x) // dx) * dx) + first_x
    print(f"{last_full_period_iteration=}")
    prediction = (slope * last_full_period_iteration) + shift
    print(f"{float(prediction)=}")
    n_remaining_iterations = tile_limit - last_full_period_iteration
    print(f"{n_remaining_iterations=}, {last_full_period_iteration + n_remaining_iterations=}")

    prediction_addition = mem_max_heights[first_x + (n_remaining_iterations - 0)] - mem_max_heights[first_x]
    return math.floor(prediction) + prediction_addition


def simulate_tetris(
    directions_gen: Iterator[tuple[Point2, bool]],
    tile_limit: int,
) -> int:
    highest_rock_y = 0
    rocks: set[Point2] = set()
    mem_map = defaultdict(list)
    mem_max_heights: list[int] = []

    for iteration, tile in tqdm(enumerate(generate_tiles(tile_limit)), total=tile_limit):

        falling_tile = move_tile(tile, Point2(2, 4 + highest_rock_y))
        falling_step = 0
        while True:
            direction, is_first_direction = next(directions_gen)
            if is_first_direction and rocks:
                print(f"directions repeat... {iteration=}, {falling_step=}, {tile=}")
                print(f"{highest_rock_y=}")
                heights = get_rock_heights(rocks)
                mh = min(heights)
                height_differences = tuple([h - mh for h in heights])
                mem_key = (tuple(p.x for p in falling_tile), falling_step, height_differences)
                mem_map[mem_key].append((iteration, max(heights)))
                print(dict(mem_map))
            shifted_tile = move_tile(falling_tile, direction)
            if not is_colliding(shifted_tile, rocks):
                falling_tile = shifted_tile
            lowered_tile = move_tile(falling_tile, Point2(0, -1))
            if is_colliding(lowered_tile, rocks):
                break
            falling_tile = lowered_tile
            falling_step += 1

        rocks = rocks.union({p for p in falling_tile})
        mem_max_heights.append(highest_rock_y)
        highest_rock_y = max(highest_rock_y, max(p.y for p in falling_tile))

        if any(len(val) == 2 for val in mem_map.values()):
            prediction = predict_using_periods(tile_limit, mem_map, mem_max_heights)
            print(f"{prediction=}")
            return prediction

    return highest_rock_y


def directions_generator(directions: Iterable[Point2]) -> Iterable[tuple[Point2, bool]]:
    all_directions = list(directions)
    i = 0
    while True:
        yield all_directions[i % len(all_directions)], i % len(all_directions) == 0
        i += 1


def solve_first(path: str, tile_limit: int) -> int:
    directions_gen = iter(directions_generator(parse(load_stripped_lines(path))))
    return simulate_tetris(directions_gen, tile_limit)


def test_parse() -> None:
    directions = list(parse(load_stripped_lines("inputs/17_0")))
    assert len(directions) == len(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>")
    assert directions[0] == Point2(1, 0)
    assert directions[3] == Point2(-1, 0)
    assert directions[-1] == Point2(1, 0)


if __name__ == "__main__":
    test_parse()
    assert solve_first("inputs/17_0", 2022) == 3068
    assert solve_first("inputs/17_0", 1000000000000) == 1514285714288
    print(solve_first("inputs/17_1", 1000000000000))
