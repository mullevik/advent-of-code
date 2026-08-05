from collections.abc import Iterable
from itertools import pairwise


def p1(inp: str) -> str:
    polymer = [c for c in inp.strip()]

    return str(len(react(polymer)))


def p2(inp: str) -> str:
    polymer = [c for c in inp.strip()]
    unique_types = set(c.lower() for c in polymer)

    return str(
        min(len(react([c for c in polymer if c.lower() != t])) for t in unique_types)
    )


def react(polymer: list[str]) -> list[str]:
    while True:
        indices_for_removal = set(find_all_reacting_pairs(polymer))
        if indices_for_removal:
            polymer = [c for i, c in enumerate(polymer) if i not in indices_for_removal]
        else:
            break
    return polymer


def find_all_reacting_pairs(polymer: Iterable[str]) -> Iterable[int]:
    prev = -2
    for i, (a, b) in enumerate(pairwise(polymer)):
        if a != b and (a.lower() == b or b.lower() == a):
            if i > prev + 1:
                prev = i
                yield i
                yield i + 1
