from aoc_python.utils import Point2, load_raw_lines, sign


def parse(line: str) -> tuple[Point2, int]:
    match line.split():
        case ["R", repeats]:
            return Point2(1, 0), int(repeats)
        case ["L", repeats]:
            return Point2(-1, 0), int(repeats)
        case ["U", repeats]:
            return Point2(0, -1), int(repeats)
        case ["D", repeats]:
            return Point2(0, 1), int(repeats)
        case _:
            raise ValueError(f"unknown line '{line}'")


def join(head: Point2, tail: Point2) -> Point2:
    dx = abs(head.x - tail.x)
    dy = abs(head.y - tail.y)

    if dx <= 1 and dy <= 1:
        return tail
    if dx == 2 and dy == 2:
        return tail + ((head - tail) // 2)
    if dx > 1:
        # match head on y and move by one on x
        return Point2(tail.x + sign(head.x - tail.x), head.y)
    if dy > 1:
        # match head on x and move by one on y
        return Point2(head.x, tail.y + sign(head.y - tail.y))
    raise ValueError(f"invalid distance {dx=} {dy=}")


def print_grid(positions: set[Point2], body: list[Point2]) -> None:
    positions = positions.union(set(body))
    min_x, max_x = min(p.x for p in positions), max(p.x for p in positions)
    min_y, max_y = min(p.y for p in positions), max(p.y for p in positions)

    snake = {pos: i for i, pos in reversed(list(enumerate(body)))}

    for y in range(min_y, max_y + 1):
        for x in range(min_x, max_x + 1):
            if (x, y) in snake:
                number = snake[(x, y)]
                if number == 0:
                    print("H", end="")
                elif number == len(body) - 1:
                    print("T", end="")
                else:
                    print(number, end="")
            elif (x, y) in positions:
                print("#", end="")
            else:
                print(" ", end="")
        print("")


def solve(path: str, n_parts: int) -> int:
    lines = load_raw_lines(path)
    moves_and_repeats = [parse(line) for line in lines]
    tail_positions: set[Point2] = set()

    body = [Point2(0, 0) for _ in range(n_parts)]

    for move, r in moves_and_repeats:
        for i in range(r):
            body[0] = body[0] + move

            for i in range(1, len(body)):
                body[i] = join(body[i - 1], body[i])

            tail_positions.add(body[-1])

    return len(tail_positions)


if __name__ == "__main__":
    assert solve("inputs/09_0", n_parts=2) == 13
    assert solve("inputs/09_0", n_parts=10) == 1
    assert solve("inputs/09_2", n_parts=10) == 36
