from typing import Iterable
from aoc_python.utils import Point3, load_raw_lines
from tqdm import tqdm


def parse(lines: list[str]) -> Iterable[int]:
    for line in lines:
        yield int(line)


def mix(number_pairs: list[tuple[int, int]], initial_numbers: list[int]) -> None:
    for i, number in tqdm(enumerate(initial_numbers)):
        src_idx = number_pairs.index((i, number))
        number_pairs.remove((i, number))
        number_pairs.insert((src_idx + number) % len(number_pairs), (i, number))


def solve_first(path: str) -> int:
    initial_numbers = list(parse(load_raw_lines(path)))
    number_pairs: list[tuple[int, int]] = [(i, n) for i, n in enumerate(initial_numbers)]

    mix(number_pairs, initial_numbers)

    zero_idx = number_pairs.index((initial_numbers.index(0), 0))
    return (
        number_pairs[(zero_idx + 1000) % len(number_pairs)][1]
        + number_pairs[(zero_idx + 2000) % len(number_pairs)][1]
        + number_pairs[(zero_idx + 3000) % len(number_pairs)][1]
    )


def solve_second(path: str) -> int:
    initial_numbers = [x * 811589153 for x in parse(load_raw_lines(path))]
    number_pairs: list[tuple[int, int]] = [(i, n) for i, n in enumerate(initial_numbers)]

    for _ in range(10):
        mix(number_pairs, initial_numbers)

    zero_idx = number_pairs.index((initial_numbers.index(0), 0))
    return (
        number_pairs[(zero_idx + 1000) % len(number_pairs)][1]
        + number_pairs[(zero_idx + 2000) % len(number_pairs)][1]
        + number_pairs[(zero_idx + 3000) % len(number_pairs)][1]
    )


if __name__ == "__main__":
    assert solve_first("inputs/20_0") == 3
    assert solve_second("inputs/20_0") == 1623178306
    print(solve_second("inputs/20_1"))
