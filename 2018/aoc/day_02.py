from collections import Counter
from collections.abc import Iterable


def p1(inp: str) -> str:
    lines = [line for line in inp.split("\n") if line]
    counters = [Counter(line) for line in lines]

    return str(count_occurrences(counters, 2) * count_occurrences(counters, 3))


def count_occurrences(counters: Iterable[Counter], amount: int) -> int:
    return len([c for c in counters if any(count == amount for count in c.values())])


def p2(inp: str) -> str:
    lines = [line for line in inp.split("\n") if line]

    for line in lines:
        for other in lines:
            same_chars = [a for a, b in zip(line, other) if a == b]

            if len(same_chars) == len(line) - 1:
                return "".join(same_chars)

    raise ValueError("no two lines differ at just one char")
