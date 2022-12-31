from typing import Iterable
from aoc_python.utils import Point3, load_stripped_lines


def parse(lines: list[str]) -> Iterable[Point3]:
    for line in lines:
        values = line.split(",")
        yield Point3(int(values[0]), int(values[1]), int(values[2]))


def test_parse() -> None:
    points = list(parse(load_stripped_lines("inputs/18_0")))
    assert len(points) == 13
    assert points[0] == Point3(2, 2, 2)
    assert points[-1] == Point3(2, 3, 5)


def solve_first(path: str) -> int:
    droplets = set(parse(load_stripped_lines(path)))

    sides_exposed = 0

    for droplet in droplets:
        for adjacent in droplet.six_neighbors:
            if adjacent not in droplets:
                sides_exposed += 1

    return sides_exposed


def is_within_bounds(p: Point3) -> bool:
    LB = -4
    UB = 27  # hardcoded value for AOC input that could be computed dynamically as well

    if LB <= p.x <= UB and LB <= p.y <= UB and LB <= p.z <= UB:
        return True
    return False


def solve_second(path: str) -> int:
    obsidian_droplets = set(parse(load_stripped_lines(path)))

    first_droplet = Point3(0, 0, 0)
    if first_droplet in obsidian_droplets:
        raise ValueError(f"Chosen initial point is an obsidian")

    lava_droplets: set[Point3] = {first_droplet}
    stack: list[Point3] = [first_droplet]

    while stack:
        current = stack.pop()

        for adjacent in current.six_neighbors:
            if not is_within_bounds(adjacent):
                continue
            if adjacent not in lava_droplets and adjacent not in obsidian_droplets:
                lava_droplets.add(adjacent)
                stack.append(adjacent)

    sides_exposed = 0

    for droplet in obsidian_droplets:
        for adjacent in droplet.six_neighbors:
            if adjacent not in obsidian_droplets and adjacent in lava_droplets:
                sides_exposed += 1

    return sides_exposed


if __name__ == "__main__":
    test_parse()
    assert solve_first("inputs/18_0") == 64
    assert solve_second("inputs/18_0") == 58
    print(solve_second("inputs/18_1"))
