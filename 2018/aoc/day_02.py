from collections import Counter
from collections.abc import Iterable

from aoc.commons import non_empty_lines


def p1(inp: str) -> str:
    counters = [Counter(line) for line in non_empty_lines(inp)]
    return str(count_occurrences(counters, 2) * count_occurrences(counters, 3))


def count_occurrences(counters: Iterable[Counter], amount: int) -> int:
    return len([c for c in counters if any(count == amount for count in c.values())])


def p2(inp: str) -> str:
    lines = non_empty_lines(inp)

    for line in lines:
        for other in lines:
            same_chars = [a for a, b in zip(line, other) if a == b]

            if len(same_chars) == len(line) - 1:
                return "".join(same_chars)

    raise ValueError("no two lines differ at just one char")
