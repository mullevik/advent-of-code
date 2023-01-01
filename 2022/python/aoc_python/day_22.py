import copy
import math
import string
from typing import Callable
from aoc_python.utils import Grid2, Point2, clear_outputs, load_raw_lines, load_stripped_lines, rotate
from fractions import Fraction


def parse_grid(lines: list[str]) -> Grid2:
    w = max(len(x[:-1]) for x in lines)
    h = len(lines)
    grid = Grid2.filled_with(w, h, " ")
    for y, row in enumerate(lines):
        for x, cell in enumerate(row[:-1]):
            grid[x, y] = cell
    return grid


def parse_instructions(line: str) -> list[str | int]:
    buffer: list[str] = []
    instructions: list[str | int] = []
    for chr in line:
        if not buffer:
            if chr in string.digits:
                buffer.append(chr)
            else:
                instructions.append(chr)
        else:
            if chr in string.digits:
                buffer.append(chr)
            else:
                instructions.append(int("".join(buffer)))
                instructions.append(chr)
                buffer = []
    if buffer:
        instructions.append(int("".join(buffer)))
    return instructions


def parse(lines: list[str]) -> tuple[Grid2, list[str | int]]:
    return parse_grid(lines[:-2]), parse_instructions(lines[-1])


def get_initial_position_and_direction(grid: Grid2) -> tuple[Point2, Point2]:
    direction = Point2(1, 0)
    location = Point2(0, 0)
    for x in range(grid.width):
        if grid[x, 0] != " ":
            location = Point2(x, 0)
            break
    return location, direction


def rotate_clockwise(direction: Point2) -> Point2:
    return rotate(direction, math.pi / 2)


def rotate_anticlockwise(direction: Point2) -> Point2:
    return rotate(direction, (3 / 2) * math.pi)


def find_first(grid: Grid2, direction: Point2, starting_point: Point2) -> Point2:
    current = starting_point
    while grid.has(current):
        if grid[current] != " ":
            return current
        current = current + direction
    raise ValueError(f"{current=}")


def move(grid: Grid2, location: Point2, direction: Point2) -> Point2:
    next_location = location + direction

    if not grid.has(next_location) or grid[next_location] == " ":
        if direction == Point2(1, 0):
            next_location = find_first(grid, direction, Point2(0, location.y))
        elif direction == Point2(0, 1):
            next_location = find_first(grid, direction, Point2(location.x, 0))
        elif direction == Point2(-1, 0):
            next_location = find_first(grid, direction, Point2(grid.width - 1, location.y))
        elif direction == Point2(0, -1):
            next_location = find_first(grid, direction, Point2(location.x, grid.height - 1))
        else:
            raise ValueError(f"{direction=}")

    if grid[next_location] == "#":
        return location
    if grid[next_location] == ".":
        return next_location
    raise ValueError(f"{location=}, {direction=}")


def get_password(location: Point2, direction: Point2) -> int:
    direction_value = 0
    if direction == Point2(0, -1):
        direction_value = 3
    elif direction == Point2(-1, 0):
        direction_value = 2
    elif direction == Point2(0, 1):
        direction_value = 1

    password = (1000 * (location.y + 1)) + (4 * (location.x + 1)) + direction_value
    print(f"1000 * {location.y=} + 1 + 4 * {location.x=} + 1 + {direction_value=} = {password}")
    return password


def solve_first(path: str) -> int:
    grid, instructions = parse(load_raw_lines(path))
    path_grid = copy.deepcopy(grid)

    location, direction = get_initial_position_and_direction(grid)

    for instruction in instructions:

        if instruction == "R":
            direction = rotate_clockwise(direction)
        elif instruction == "L":
            direction = rotate_anticlockwise(direction)
        else:
            assert isinstance(instruction, int)
            for step in range(instruction):
                new_location = move(grid, location, direction)
                path_grid[new_location] = "x"
                location = new_location

        # clear_outputs()
        # print(grid)
        # print(path_grid)
        print(f"{location}, {direction} after {instruction=}")

    return get_password(location, direction)


def get_cube_map(grid: Grid2, cube_size: int) -> Grid2:
    w = (grid.width // cube_size) + 2
    h = (grid.height // cube_size) + 2

    cube_map = Grid2.filled_with(w, h, "?")
    letters = iter(string.ascii_letters)
    for p, _ in cube_map:
        cube_map[p] = next(letters)
    return cube_map


def to_cube_space(p: Point2, cube_size: int) -> Point2:
    scaled_p = p // cube_size
    return scaled_p + 1


def to_world_space(p: Point2, cube_size: int) -> Point2:
    return (p * cube_size) - cube_size


def get_face_origin(face: str, cube_map: Grid2, cube_size: int) -> Point2:
    cube_p = Point2(-1, -1)
    for p, f in cube_map:
        if f == face:
            cube_p = p
            break
    assert cube_p.x >= 0 and cube_p.y >= 0
    return to_world_space(cube_p, cube_size)


def warp(
    src_p: Point2,
    src_d: Point2,
    cube_map: Grid2,
    cube_size: int,
    warp_map: dict[tuple[str, Point2], tuple[str, int]],
) -> tuple[Point2, Point2]:
    src_face = cube_map[to_cube_space(src_p, cube_size)]
    dst_face, n_rotations = warp_map[(src_face, src_d)]

    src_face_origin = get_face_origin(src_face, cube_map, cube_size)
    dst_face_origin = get_face_origin(dst_face, cube_map, cube_size)

    src_addition = src_p - src_face_origin
    # print(f"{src_face_origin=}, {dst_face_origin=}, {src_addition=}")

    dst_d = src_d
    for _ in range(n_rotations):
        _rotated_addition = rotate_clockwise(src_addition)
        src_addition = Point2(_rotated_addition.x + (cube_size - 1), _rotated_addition.y)
        dst_d = rotate_clockwise(dst_d)
        # print(f"{_rotated_addition=}, {src_addition=}, {dst_d=}")

    dst_p = dst_face_origin + src_addition + (dst_d * (cube_size - 1))
    # print(f"{dst_p=}")

    next_location = dst_p + dst_d
    return next_location, dst_d


def move_with_warp(
    location: Point2,
    direction: Point2,
    grid: Grid2,
    cube_map: Grid2,
    cube_size: int,
    warp_map: dict[tuple[str, Point2], tuple[str, int]],
) -> tuple[Point2, Point2]:
    next_location = location + direction
    next_direction = direction

    if not grid.has(next_location) or grid[next_location] == " ":
        print(f"warping at {location=}, {direction=}")
        next_location, next_direction = warp(next_location, direction, cube_map, cube_size, warp_map)

    if grid[next_location] == "#":
        return location, direction
    if grid[next_location] == ".":
        return next_location, next_direction
    raise ValueError(f"{next_location=}, {next_direction=}, {grid[next_location]=}")


def solve_second(path: str, cube_size: int, warp_map: dict[tuple[str, Point2], tuple[str, int]]) -> int:
    grid, instructions = parse(load_raw_lines(path))
    path_grid = copy.deepcopy(grid)
    cube_map = get_cube_map(grid, cube_size)
    print(cube_map)
    location, direction = get_initial_position_and_direction(grid)

    for instruction in instructions:

        if instruction == "R":
            direction = rotate_clockwise(direction)
        elif instruction == "L":
            direction = rotate_anticlockwise(direction)
        else:
            assert isinstance(instruction, int)
            for step in range(instruction):
                new_location, new_direction = move_with_warp(location, direction, grid, cube_map, cube_size, warp_map)
                path_grid[new_location] = "x"
                location = new_location
                direction = new_direction

        # clear_outputs()
        # print(grid)
        # print(path_grid)
        # print(f"{location}, {direction} after {instruction=}")

    return get_password(location, direction)


def test_parse() -> None:
    grid, instructions = parse(load_raw_lines("inputs/22_0"))

    print(grid)
    print(instructions)
    assert grid.width == 16
    assert grid.height == 12
    assert grid[0, 0] == " "
    assert len(instructions) == 13
    assert instructions[0] == 10
    assert instructions[-1] == 5


# key: entry face and direction
# value: exit face and number of clockwise turns
SMALL_WARP_MAP = {
    ("d", Point2(0, -1)): ("h", 2),
    ("k", Point2(1, 0)): ("x", 2),
    ("q", Point2(1, 0)): ("q", 1),
    ("q", Point2(0, -1)): ("q", 3),
    ("x", Point2(1, 0)): ("k", 2),
    ("C", Point2(0, 1)): ("m", 3),
    ("B", Point2(0, 1)): ("t", 2),
    ("u", Point2(-1, 0)): ("u", 1),
    ("u", Point2(0, 1)): ("u", 3),
    ("t", Point2(0, 1)): ("B", 2),
    ("m", Point2(-1, 0)): ("C", 1),
    ("h", Point2(0, -1)): ("d", 2),
    ("i", Point2(0, -1)): ("i", 1),
    ("i", Point2(-1, 0)): ("i", 3),
}
LARGE_WARP_MAP = {
    ("c", Point2(0, -1)): ("u", 1),
    ("u", Point2(-1, 0)): ("c", 3),
    ("d", Point2(0, -1)): ("A", 0),
    ("A", Point2(0, 1)): ("d", 0),
    ("j", Point2(1, 0)): ("s", 2),
    ("s", Point2(1, 0)): ("j", 2),
    ("n", Point2(0, 1)): ("n", 1),
    ("n", Point2(1, 0)): ("n", 3),
    ("w", Point2(0, 1)): ("w", 1),
    ("w", Point2(1, 0)): ("w", 3),
    ("p", Point2(-1, 0)): ("g", 2),
    ("g", Point2(-1, 0)): ("p", 2),
    ("l", Point2(0, -1)): ("l", 1),
    ("l", Point2(-1, 0)): ("l", 3),
}


def test_warp() -> None:
    grid, instructions = parse(load_raw_lines("inputs/22_0"))
    cube_size = 4
    cube_map = get_cube_map(grid, cube_size)

    warped_p, warped_d = warp(Point2(12, 5), Point2(1, 0), cube_map, cube_size, SMALL_WARP_MAP)
    assert warped_p == Point2(14, 8)
    assert warped_d == Point2(0, 1)

    warped_p, warped_d = warp(Point2(10, 12), Point2(0, 1), cube_map, cube_size, SMALL_WARP_MAP)
    assert warped_p == Point2(1, 7)
    assert warped_d == Point2(0, -1)

    warped_p, warped_d = warp(Point2(6, 3), Point2(0, -1), cube_map, cube_size, SMALL_WARP_MAP)
    print(f"{warped_p=}, {warped_d=}")
    assert warped_p == Point2(8, 2)
    assert warped_d == Point2(1, 0)


if __name__ == "__main__":
    test_parse()
    test_warp()
    assert solve_first("inputs/22_0") == 6032
    assert solve_second("inputs/22_0", 4, SMALL_WARP_MAP) == 5031
    print(solve_second("inputs/22_1", 50, LARGE_WARP_MAP))
