def p1(inp: str) -> str:
    return str(sum(int(p) for p in get_parts(inp) if p.strip()))


def p2(inp: str) -> str:
    parts = [p for p in get_parts(inp) if p.strip()]

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
    return inp.split("\n") if "\n" in inp else inp.split(",")
