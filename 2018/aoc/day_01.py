from aoc.commons import non_empty_lines


def p1(inp: str) -> str:
    return str(sum(int(p) for p in get_parts(inp)))


def p2(inp: str) -> str:
    parts = get_parts(inp)

    i = 0
    visited = {0}
    freq = 0

    while True:
        change = int(parts[i])
        freq += change
        if freq in visited:
            return str(freq)
        visited.add(freq)
        i = (i + 1) % len(parts)


def get_parts(inp: str) -> list[str]:
    if "\n" in inp:
        return non_empty_lines(inp)
    return [line for line in inp.split(",") if line.strip()]
