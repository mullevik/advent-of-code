from collections import Counter
from typing import Iterable
from aoc_python.utils import Point2, load_stripped_lines


def parse(lines: list[str]) -> Iterable[Point2]:
    for y, row in enumerate(lines):
        for x, cell in enumerate(row):
            if cell == "#":
                yield Point2(x, y)


N = Point2(0, -1)
S = Point2(0, 1)
W = Point2(-1, 0)
E = Point2(1, 0)

COLLISIONS = {N: (N, W + N, E + N), S: (S, W + S, E + S), W: (W, N + W, S + W), E: (E, N + E, S + E)}


def print_elves(elves: set[Point2], unique_propositions: set[Point2]) -> None:
    min_x = min(e.x for e in set.union(elves, unique_propositions))
    min_y = min(e.y for e in set.union(elves, unique_propositions))
    max_x = max(e.x for e in set.union(elves, unique_propositions))
    max_y = max(e.y for e in set.union(elves, unique_propositions))

    for y in range(min_y, max_y + 1):
        for x in range(min_x, max_x + 1):
            if Point2(x, y) in elves:
                print("#", end="")
            elif Point2(x, y) in unique_propositions:
                print("p", end="")
            else:
                print(".", end="")
        print("")


def count_empty_tiles(elves: set[Point2]) -> int:
    min_x = min(e.x for e in elves)
    min_y = min(e.y for e in elves)
    max_x = max(e.x for e in elves)
    max_y = max(e.y for e in elves)
    w = (max_x + 1) - min_x
    h = (max_y + 1) - min_y
    return (w * h) - len(elves)


def solve(path: str, check_movement: bool) -> int:
    elves = set(parse(load_stripped_lines(path)))
    print(f"{elves=}")

    directions = [N, S, W, E]
    print_elves(elves, set())
    i = 0
    while True:
        propositions = {}
        for e in elves:
            if all(a not in elves for a in e.eight_neighbors):
                continue

            for j in range(len(directions)):
                direction = directions[(i + j) % len(directions)]
                if all(e + p not in elves for p in COLLISIONS[direction]):
                    propositions[e] = e + direction
                    break

        proposition_counts = Counter(propositions.values())
        elves_moved = False
        for e, p in propositions.items():
            if proposition_counts[p] == 1:
                elves.remove(e)
                elves.add(p)
                if p != e:
                    elves_moved = True
        if check_movement and not elves_moved:
            return i + 1
        i += 1
        if not check_movement and i >= 10:
            return count_empty_tiles(elves)


if __name__ == "__main__":
    assert solve("inputs/23_0", False) == 110
    assert solve("inputs/23_0", True) == 20
    print(solve("inputs/23_1", True))
