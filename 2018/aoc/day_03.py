from collections.abc import Iterable


def p1(inp: str) -> str:
    space = build_space(list(parse(inp)))
    return str(sum(1 for row in space for cell in row if cell > 1))


def p2(inp: str) -> str:
    claims = list(parse(inp))
    space = build_space(claims)

    for i, c, s in claims:
        if not has_any_overlap(c, s, space):
            return str(i)

    raise ValueError("no claim found")


def has_any_overlap(c: tuple[int, int], s: tuple[int, int], space: list[list[int]]) -> bool:

    for y in range(c[1], c[1] + s[1]):
        for x in range(c[0], c[0] + s[0]):
            if space[y][x] > 1:
                return True
    return False


def build_space(
    claims: list[tuple[int, tuple[int, int], tuple[int, int]]],
) -> list[list[int]]:
    max_x = max(c[0] + s[0] for _, c, s in claims)
    max_y = max(c[1] + s[1] for _, c, s in claims)

    space = [[0 for _ in range(max_x)] for _ in range(max_y)]

    for _, c, s in claims:
        for y in range(c[1], c[1] + s[1]):
            for x in range(c[0], c[0] + s[0]):
                space[y][x] += 1

    return space


def parse(inp: str) -> Iterable[tuple[int, tuple[int, int], tuple[int, int]]]:

    for line in inp.split("\n"):
        match line.split(" "):
            case [_id, "@", _corner, _dim]:
                yield (
                    int(_id.replace("#", "")),
                    (
                        int(_corner.split(",")[0]),
                        int(_corner.replace(":", "").split(",")[1]),
                    ),
                    (int(_dim.split("x")[0]), int(_dim.split("x")[1])),
                )
