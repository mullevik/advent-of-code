from collections import Counter
from collections.abc import Iterable

OWNERSHIP_EDGE = -1


def p1(inp: str) -> str:

    coords = list(parse_coords(inp))

    x_max = max(c[0] for c in coords)
    y_max = max(c[1] for c in coords)

    queue = [c for c in coords]

    ownership_map = {c: i for i, c in enumerate(coords)}
    distance_map = {c: 0 for c in coords}
    out_of_bounds = set()

    while queue:
        curr = queue.pop(0)
        curr_dst = distance_map[curr]

        for adj in adjacent4(curr):
            adj_dst = curr_dst + 1

            if not ((0 <= adj[0] <= x_max) and (0 <= adj[1] <= y_max)):
                out_of_bounds.add(ownership_map[curr])
            elif (
                adj in distance_map
                and distance_map[adj] == adj_dst
                and ownership_map[adj] != ownership_map[curr]
            ):
                ownership_map[adj] = OWNERSHIP_EDGE
            elif adj not in distance_map:
                queue.append(adj)
                ownership_map[adj] = ownership_map[curr]
                distance_map[adj] = adj_dst

    _, area = Counter(
        val for val in ownership_map.values() if val not in out_of_bounds
    ).most_common(1)[0]
    return str(area)


def p2(inp: str, distance_threshold: int = 10000) -> str:

    coords = set(parse_coords(inp))
    x_max = max(c[0] for c in coords)
    y_max = max(c[1] for c in coords)

    n_valid_coords = 0

    for y in range(y_max):
        for x in range(x_max):
            curr = (x, y)
            if (
                sum(manhattan_dist(curr, other) for other in coords)
                < distance_threshold
            ):
                n_valid_coords += 1

    return str(n_valid_coords)


def adjacent4(
    p: tuple[int, int],
) -> tuple[tuple[int, int], tuple[int, int], tuple[int, int], tuple[int, int]]:
    return (
        (p[0] + 1, p[1] + 0),
        (p[0] - 1, p[1] + 0),
        (p[0] + 0, p[1] + 1),
        (p[0] + 0, p[1] - 1),
    )


def manhattan_dist(a: tuple[int, int], b: tuple[int, int]) -> int:
    return abs(a[0] - b[0]) + abs(a[1] - b[1])


def parse_coords(inp: str) -> Iterable[tuple[int, int]]:
    for line in inp.split("\n"):
        if line.strip():
            parts = line.split(",")
            yield int(parts[0]), int(parts[1])
