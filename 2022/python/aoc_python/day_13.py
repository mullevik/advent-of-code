import itertools
from typing import TypeAlias
from collections.abc import Iterable
from aoc_python.utils import load_stripped_lines
import json

Something: TypeAlias = list | int


def generate_two_lines(lines: Iterable[str]) -> Iterable[str]:
    yield from itertools.takewhile(lambda x: len(x) > 0, lines)


def parse(lines: list[str]) -> list[tuple[Something, Something]]:
    it_lines = iter(lines)
    pairs = []
    while len((two_lines := list(generate_two_lines(it_lines)))) == 2:
        pairs.append((json.loads(two_lines[0]), json.loads(two_lines[1])))
    return pairs


def get_first(x: Something) -> Something | None:
    if isinstance(x, int):
        return x
    if len(x) == 0:
        return None
    return x[0]


def ensure_list(s: Something) -> list:
    return s if isinstance(s, list) else [s]


def is_in_right_order(left: Something, right: Something) -> bool | None:
    # print(f"comparing: {left=} vs. {right=}")
    if isinstance(left, int) and isinstance(right, int):
        if left < right:
            # print(f"{left} < {right} => True")
            return True
        if left > right:
            # print(f"{left} > {right} => False")
            return False
        return None

    if isinstance(left, list) and isinstance(right, list):
        for i in range(max(len(left), len(right))):
            if not i < len(left) and i < len(right):
                # print("left ran out first => True")
                return True
            if i < len(left) and not i < len(right):
                # print("right ran out first => False")
                return False
            if not i < len(left) and not i < len(right):
                # print("lists have the same lenght continue checking next part")
                return None
            verdict = is_in_right_order(left[i], right[i])
            if isinstance(verdict, bool):
                return verdict
        return None

    if isinstance(left, int):
        verdict = is_in_right_order([left], right)
        if isinstance(verdict, bool):
            return verdict
    if isinstance(right, int):
        verdict = is_in_right_order(left, [right])
        if isinstance(verdict, bool):
            return verdict
    return None


def solve_first(path: str) -> int:
    pairs = parse(load_stripped_lines(path))

    valid_indices = [i + 1 for i, pair in enumerate(pairs) if is_in_right_order(pair[0], pair[1])]
    print(valid_indices)
    return sum(valid_indices)


def merge_sort_packets(packets: list[Something]) -> list[Something]:
    if len(packets) < 2:
        return packets
    print(f"mergesort: {packets}")
    half = len(packets) // 2
    left_half = packets[:half]
    right_half = packets[half:]

    left = merge_sort_packets(left_half)
    right = merge_sort_packets(right_half)

    merged = []
    l_idx = 0
    r_idx = 0
    while l_idx < len(left) and r_idx < len(right):
        if is_in_right_order(left[l_idx], right[r_idx]):
            merged.append(left[l_idx])
            l_idx += 1
        else:
            merged.append(right[r_idx])
            r_idx += 1
    merged.extend(left[l_idx:])
    merged.extend(right[r_idx:])
    return merged


def index_of(divider: int, packets: list[Something]) -> int:
    for i, p in enumerate(packets):
        if isinstance(p, list) and len(p) == 1 and isinstance(p[0], list) and len(p[0]) == 1 and p[0][0] == divider:
            return i
    raise ValueError(f"{divider=} not found")


def solve_second(path: str) -> int:
    pairs = parse(load_stripped_lines(path))
    pairs.append(([[2]], [[6]]))
    packets = [packet for pair in pairs for packet in pair]
    print(f"{packets=}")
    sorted_packets = merge_sort_packets(packets)
    print(f"{sorted_packets=}")

    return (index_of(2, sorted_packets) + 1) * (index_of(6, sorted_packets) + 1)


def test_parse() -> None:
    pairs = parse(load_stripped_lines("inputs/13_0"))
    print(pairs)
    assert len(pairs) == 8
    assert len(pairs[0][0]) == 5
    assert len(pairs[0][1]) == 5
    assert len(pairs[1][0]) == 2
    assert len(pairs[1][1]) == 2


if __name__ == "__main__":
    test_parse()
    assert solve_first("inputs/13_0") == 13
    assert solve_second("inputs/13_0") == 140
    print(solve_second("inputs/13_1"))
