import time
from aoc_python.utils import Point, clear_outputs, load_stripped_lines

SAND_ORIGIN = Point(500, 0)


def parse_vertex(word: str) -> Point:
    x, y = word.split(",")
    return Point(int(x), int(y))


def parse(lines: list[str]) -> set[Point]:
    rocks = set()
    for line in lines:
        vertices = line.split("->")
        current = parse_vertex(vertices[0])
        for next_word in vertices[1:]:
            next = parse_vertex(next_word)
            if next.x == current.x:
                rocks.update([Point(next.x, y) for y in range(min(next.y, current.y), max(next.y, current.y) + 1)])
            elif next.y == current.y:
                rocks.update([Point(x, next.y) for x in range(min(next.x, current.x), max(next.x, current.x) + 1)])
            else:
                raise ValueError(f"unexpected input: {current=} and {next=}")
            current = next
    return rocks


def simulate_sand(grain: Point, collisions: set[Point], y_threshold: int, botomless: bool) -> Point | None:
    while grain.y < y_threshold:
        if Point(grain.x, grain.y + 1) in collisions:
            if Point(grain.x - 1, grain.y + 1) in collisions:
                if Point(grain.x + 1, grain.y + 1) in collisions:
                    return grain
                grain = Point(grain.x + 1, grain.y + 1)
                continue
            grain = Point(grain.x - 1, grain.y + 1)
            continue
        grain = Point(grain.x, grain.y + 1)
    return None if botomless else grain


def print_cave(rocks: set[Point], sand: set[Point], start_at_y: int | None = None) -> None:
    min_x = min(min(r.x for r in rocks), min(s.x for s in sand))
    max_x = max(max(r.x for r in rocks), max(s.x for s in sand))
    min_y = min(min(r.y for r in rocks), min(s.y for s in sand))
    max_y = max(max(r.y for r in rocks), max(s.y for s in sand))

    if start_at_y is not None:
        min_y = start_at_y

    for y in range(min_y, max_y + 1):
        for x in range(min_x, max_x + 1):

            if Point(x, y) in rocks:
                print("#", end="")
            elif Point(x, y) in sand:
                print(".", end="")
            else:
                print(" ", end="")
        print(f"| {y:>3}")


def solve_first(path: str) -> int:
    rocks = parse(load_stripped_lines(path))
    sand: set[Point] = set()

    y_threshold = max(r.y for r in rocks)

    while (
        stable_grain := simulate_sand(Point(SAND_ORIGIN.x, SAND_ORIGIN.y), set.union(rocks, sand), y_threshold, True)
    ) is not None:
        sand.add(stable_grain)
        print_cave(rocks, sand)
        print("")

    return len(sand)


def solve_second(path: str) -> int:
    rocks = parse(load_stripped_lines(path))
    sand: set[Point] = set()

    y_threshold = max(r.y for r in rocks) + 1

    while SAND_ORIGIN not in sand:
        stable_grain = simulate_sand(Point(SAND_ORIGIN.x, SAND_ORIGIN.y), set.union(rocks, sand), y_threshold, False)
        if stable_grain is None:
            print_cave(rocks, sand)
            raise ValueError("unexpected falling grain")
        sand.add(stable_grain)
        clear_outputs()
        print_cave(rocks, sand, start_at_y=135)
        time.sleep(0.01)

    return len(sand)


def test_parse() -> None:
    rocks = parse(load_stripped_lines("inputs/14_0"))
    assert Point(498, 4) in rocks
    assert Point(501, 9) in rocks
    assert not Point(501, 10) in rocks


if __name__ == "__main__":
    test_parse()
    assert solve_first("inputs/14_0") == 24
    assert solve_second("inputs/14_0") == 93
    print(solve_second("inputs/14_1"))
