from dataclasses import dataclass

type Vec2 = tuple[int, int]


@dataclass(frozen=True)
class Cart:
    pos: Vec2
    dir: Vec2
    intersections: int


def p1(inp: str) -> str:

    world, carts = parse(inp)

    while True:
        carts, collisions = simulate(world, carts)
        if collisions:
            return f"{collisions[0][0]},{collisions[0][1]}"


def p2(inp: str) -> str:
    world, carts = parse(inp)

    while True:
        carts, _ = simulate(world, carts)

        if len(carts) == 1:
            return f"{carts[0].pos[0]},{carts[0].pos[1]}"


def simulate(
    world: list[list[str]], carts: list[Cart]
) -> tuple[list[Cart], list[Vec2]]:

    remaining_carts = sorted(carts, key=lambda c: (c.pos[1], c.pos[0]))

    new_carts = []
    collisions = []

    while remaining_carts:
        cart = remaining_carts.pop(0)

        new_pos = (cart.pos[0] + cart.dir[0], cart.pos[1] + cart.dir[1])

        new_pos_char = world[new_pos[1]][new_pos[0]]
        new_intersection = cart.intersections
        if new_pos_char in ("\\", "/"):
            new_dir = turn_direction(cart.dir, new_pos_char)
        elif new_pos_char in ("+"):
            new_dir = turn_at_intersection(cart.dir, cart.intersections)
            new_intersection = cart.intersections + 1
        else:
            new_dir = cart.dir

        colliding_carts = [c for c in new_carts + remaining_carts if c.pos == new_pos]

        if colliding_carts:
            try:
                remaining_carts.remove(colliding_carts[0])
            except ValueError:
                ...
            try:
                new_carts.remove(colliding_carts[0])
            except ValueError:
                ...
            collisions.append(new_pos)
        else:
            new_carts.append(Cart(new_pos, new_dir, new_intersection))

    return new_carts, collisions


def rotate_cw(x: Vec2) -> Vec2:
    return (-x[1], x[0])


def rotate_ccw(x: Vec2) -> Vec2:
    return (x[1], -x[0])


def turn_at_intersection(previous_dir: Vec2, intersection_state: int) -> Vec2:

    if intersection_state % 3 == 0:
        return rotate_ccw(previous_dir)

    elif intersection_state % 3 == 1:
        return previous_dir

    else:
        return rotate_cw(previous_dir)


def turn_direction(previous_dir: Vec2, new_pos_char: str) -> Vec2:
    if new_pos_char == "\\" and previous_dir == (1, 0):
        return rotate_cw(previous_dir)
    elif new_pos_char == "\\" and previous_dir == (-1, 0):
        return rotate_cw(previous_dir)
    elif new_pos_char == "\\" and previous_dir == (0, 1):
        return rotate_ccw(previous_dir)
    elif new_pos_char == "\\" and previous_dir == (0, -1):
        return rotate_ccw(previous_dir)
    elif new_pos_char == "/" and previous_dir == (-1, 0):
        return rotate_ccw(previous_dir)
    elif new_pos_char == "/" and previous_dir == (1, 0):
        return rotate_ccw(previous_dir)
    elif new_pos_char == "/" and previous_dir == (0, 1):
        return rotate_cw(previous_dir)
    elif new_pos_char == "/" and previous_dir == (0, -1):
        return rotate_cw(previous_dir)
    else:
        raise ValueError(
            f"invalid combination for turning {previous_dir=}, {new_pos_char=}"
        )


def parse(inp: str) -> tuple[list[list[str]], list[Cart]]:
    lines = [line for line in inp.split("\n") if line.strip()]

    x_max = max(len(line) for line in lines)

    world = [["" for _ in range(x_max)] for _ in range(len(lines))]

    carts = []

    for y, line in enumerate(lines):
        for x, char in enumerate(line):
            if char == ">":
                carts.append(Cart((x, y), (1, 0), 0))
                world[y][x] = "-"
            elif char == "<":
                carts.append(Cart((x, y), (-1, 0), 0))
                world[y][x] = "-"
            elif char == "v":
                carts.append(Cart((x, y), (0, 1), 0))
                world[y][x] = "|"
            elif char == "^":
                carts.append(Cart((x, y), (0, -1), 0))
                world[y][x] = "|"
            else:
                world[y][x] = char

    return world, carts
