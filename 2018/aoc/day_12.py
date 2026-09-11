from collections import defaultdict, deque

STABILITY_THRESHOLD = 1000
N_GENERATIONS = 50000000000


def p1(inp: str) -> str:

    pots, rules = parse(inp)

    for _ in range(20):
        pots = simulate_generation(pots, rules)

    return str(sum(pos for pos, val in pots.items() if val))


def p2(inp: str) -> str:
    pots, rules = parse(inp)

    prev = 0
    diffs = deque()

    for i in range(N_GENERATIONS):
        pots = simulate_generation(pots, rules)
        res = sum(pos for pos, val in pots.items() if val)

        diff = res - prev
        prev = res
        diffs.append(diff)

        if len(diffs) >= STABILITY_THRESHOLD:
            diffs.popleft()
            if all(d == diffs[0] for d in diffs):
                # stability reached
                stable_diff = diffs[0]
                return str(res + ((N_GENERATIONS - i - 1) * stable_diff))

    raise ValueError(f"Stability not found")


def simulate_generation(
    pots: defaultdict[int, bool], rules: list[tuple[list[bool], bool]]
) -> defaultdict[int, bool]:

    new_data: defaultdict[int, bool] = defaultdict(lambda: False)

    min_pos = min(pos for pos, val in pots.items() if val)
    max_pos = max(pos for pos, val in pots.items() if val)

    for pos in range(min_pos - 2, max_pos + 3):
        neighborhood = [
            pots[pos - 2],
            pots[pos - 1],
            pots[pos],
            pots[pos + 1],
            pots[pos + 2],
        ]

        for condition, outcome in rules:
            if is_applicable(condition, neighborhood):
                new_data[pos] = outcome

    return new_data


def pprint(pots: defaultdict[int, bool]) -> None:
    min_pos = min(pos for pos, val in pots.items() if val)
    max_pos = max(pos for pos, val in pots.items() if val)
    print(f"Showing from {min_pos} to {max_pos}")
    print(["#" if pots[x] else "." for x in range(min_pos, max_pos + 1)])


def is_applicable(rule: list[bool], data: list[bool]) -> bool:
    return all(a == b for a, b in zip(rule, data, strict=True))


def parse(inp: str) -> tuple[defaultdict[int, bool], list[tuple[list[bool], bool]]]:
    lines = [line for line in inp.split("\n") if line]

    pots: defaultdict[int, bool] = defaultdict(lambda: False)
    for i, ch in enumerate(lines[0].replace("initial state: ", "")):
        pots[i] = ch == "#"

    rules = [parse_rule(line) for line in lines[1:]]
    return pots, rules


def parse_rule(line: str) -> tuple[list[bool], bool]:
    parts = line.split(" => ")
    return [ch == "#" for ch in parts[0]], parts[1][0] == "#"
