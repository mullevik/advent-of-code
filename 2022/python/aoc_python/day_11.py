from collections import defaultdict
from dataclasses import dataclass
from typing import Callable
from collections.abc import Iterable
import numpy as np
from aoc_python.utils import load_raw_lines


@dataclass(frozen=True)
class Monkey:
    index: int
    starting_items: tuple[int, ...]
    operation: Callable[[int], int]
    test_divider: int
    test_success: int
    test_failed: int


def parse_operation(line: str) -> Callable[[int], int]:
    match line.split(":")[1].split():
        case ["new", "=", "old", "*", "old"]:
            return lambda old: old * old
        case ["new", "=", "old", "+", "old"]:
            return lambda old: old + old
        case ["new", "=", "old", "*", x]:
            return lambda old: old * int(x)
        case ["new", "=", "old", "+", x]:
            return lambda old: old + int(x)
        case _:
            raise ValueError(f"unexpected operation: '{line}'")


def parse_test(line: str) -> int:
    match line.split(":")[1].split():
        case ["divisible", "by", x]:
            return int(x)
        case _:
            raise ValueError(f"unexpeced test: '{line}'")


def parse_target_monkey(line: str) -> int:
    match line.split(":")[1].split():
        case ["throw", "to", "monkey", x]:
            return int(x)
        case _:
            raise ValueError(f"unexpected target moneky: '{line}'")


def parse_monkey(lines: list[str], index: int) -> Monkey:
    return Monkey(
        index,
        starting_items=tuple(int(x) for x in lines[0].split(":")[1].split(",")),
        operation=parse_operation(lines[1]),
        test_divider=parse_test(lines[2]),
        test_success=parse_target_monkey(lines[3]),
        test_failed=parse_target_monkey(lines[4]),
    )


def parse(lines: Iterable[str]) -> Iterable[Monkey]:
    while (line := next(lines, None)) is not None:
        match line.split():
            case ["Monkey", index]:
                yield parse_monkey([next(lines) for _ in range(5)], int(index.replace(":", "")))
            case []:
                continue
            case _:
                raise ValueError(f"Unexpected line '{line}'")


def throw(monkey_bags: dict[int, list[int]], source_index: int, target_index: int, item: int) -> None:
    monkey_bags[source_index].pop(0)
    monkey_bags[target_index].append(item)


def solve(path: str, n_rounds: int, divide_by_three: bool) -> int:
    lines = load_raw_lines(path)
    monkeys = list(parse(iter(lines)))

    lcm = np.lcm.reduce([m.test_divider for m in monkeys])

    monkey_bags: dict[int, list[int]] = {m.index: list(m.starting_items) for m in monkeys}
    monkey_business: dict[int, int] = defaultdict(lambda: 0)
    for round_index in range(n_rounds):
        for monkey in monkeys:
            # print(f"\nround {round_index}, monkey {monkey.index}:")
            for item in tuple(monkey_bags[monkey.index]):
                # print(f"Monkey inspects {item}")
                monkey_business[monkey.index] += 1
                worry_level = monkey.operation(item)
                # print(f"Monkey does operation to get {worry_level}")
                worry_level = worry_level // 3 if divide_by_three else worry_level % lcm
                # print(f"Monkey divides by 3 to get {worry_level}")
                if worry_level % monkey.test_divider == 0:
                    # print(f"Monkey test succeeded")
                    target_monkey_index = monkey.test_success
                else:
                    # print(f"Monkey test failed")
                    target_monkey_index = monkey.test_failed
                throw(monkey_bags, monkey.index, target_monkey_index, worry_level)
                # print(f"Monkey threw item {worry_level} to {target_monkey_index}")
        print("round", round_index)
        # print(f"after round: {round_index}", monkey_bags)

    two_most_active_monkeys = sorted(
        [(monkey_index, n_inspects) for monkey_index, n_inspects in dict(monkey_business).items()],
        key=lambda x: x[1],
        reverse=True,
    )[:2]
    print("two most active monkeys:", two_most_active_monkeys)
    return two_most_active_monkeys[0][1] * two_most_active_monkeys[1][1]


def test_parse() -> None:
    monkeys = list(parse(iter(load_raw_lines("inputs/11_0"))))
    assert len(monkeys) == 4


if __name__ == "__main__":
    test_parse()
    assert solve("inputs/11_0", 20, True) == 10605
    assert solve("inputs/11_0", 10_000, False) == 2713310158

    # print(solve("inputs/11_1", 10000, False))
