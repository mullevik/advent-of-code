from typing import Callable
from aoc_python.utils import load_stripped_lines
from fractions import Fraction

OPERATIONS: dict[str, Callable] = {
    "+": lambda a, b: a + b,
    "-": lambda a, b: a - b,
    "*": lambda a, b: a * b,
    "/": lambda a, b: a / b,
}

INVERSE_OPERATIONS: dict[str, Callable] = {
    "+": OPERATIONS["-"],
    "-": OPERATIONS["+"],
    "*": OPERATIONS["/"],
    "/": OPERATIONS["*"],
}


def parse(lines: list[str]) -> dict[str, tuple[str, str, str] | int]:
    monkeys: dict[str, tuple[str, str, str] | int] = {}

    for line in lines:
        match line.split():
            case [m, v]:
                # monkeys[m.replace(":", "")] = Fraction(int(v), 1)
                monkeys[m.replace(":", "")] = int(v)
            case [m, lhs, op, rhs]:
                monkeys[m.replace(":", "")] = (lhs, rhs, op)
            case _:
                raise ValueError(f"unexpected {line=}")

    return monkeys


def test_parse() -> None:
    monkeys = parse(load_stripped_lines("inputs/21_0"))
    assert len(monkeys) == 15
    assert "root" in monkeys
    assert monkeys["root"][0] == "pppw"
    assert monkeys["hmdt"] == 32


def resolve(monkey_name: str, monkeys: dict[str, tuple[str, str, str] | int]) -> int:
    value = monkeys[monkey_name]
    if isinstance(value, (int, Fraction)):
        return value
    if not isinstance(value, tuple):
        raise ValueError(f"unexpected instance of {value=}, {type(value)=}")

    left = resolve(value[0], monkeys)
    right = resolve(value[1], monkeys)
    return OPERATIONS[value[2]](left, right)


def solve_first(path: str) -> int:
    monkeys = parse(load_stripped_lines(path))
    return resolve("root", monkeys)


def try_to_resolve(
    monkey_name: str, monkeys: dict[str, tuple[str, str, str] | int | None], resolved_monkeys: dict[str, int | None]
) -> int | None:
    value = monkeys[monkey_name]
    if value is None:
        resolved_monkeys[monkey_name] = None
        return None
    if isinstance(value, (int, Fraction)):
        resolved_monkeys[monkey_name] = value
        return value
    left = (
        try_to_resolve(value[0], monkeys, resolved_monkeys)
        if resolved_monkeys[value[0]] is None
        else resolved_monkeys[value[0]]
    )
    right = (
        try_to_resolve(value[1], monkeys, resolved_monkeys)
        if resolved_monkeys[value[1]] is None
        else resolved_monkeys[value[1]]
    )
    computed = OPERATIONS[value[2]](left, right) if left is not None and right is not None else None
    resolved_monkeys[monkey_name] = computed
    return computed


def inverse_resolve(
    monkey_name: str,
    monkeys: dict[str, tuple[str, str, str] | int | None],
    resolved_monkeys: dict[str, int | None],
    target_value: int,
) -> int:
    if monkey_name == "humn":
        return target_value
    value = monkeys[monkey_name]
    if isinstance(value, (int, Fraction)):
        raise ValueError(f"unexpected unresolved int: {value=}")
    if not isinstance(value, tuple):
        raise ValueError(f"unexpected type {value=}, {type(value)=}")
    resolved_name = value[0]
    unresolved_name = value[1]
    first_is_resolved = True
    if resolved_monkeys[resolved_name] is None:
        tmp = unresolved_name
        unresolved_name = resolved_name
        resolved_name = tmp
        first_is_resolved = False
    resolved_value = resolved_monkeys[resolved_name]
    assert resolved_value is not None
    if value[2] in ("*", "+"):
        first_is_resolved = True

    if value[2] in ("/",) and first_is_resolved:
        new_target_value = 1 / (target_value / resolved_value)
    elif value[2] in ("-",) and first_is_resolved:
        new_target_value = (-target_value) + resolved_value
    else:
        new_target_value = (
            INVERSE_OPERATIONS[value[2]](target_value, resolved_value)
            if first_is_resolved
            else INVERSE_OPERATIONS[value[2]](resolved_value, target_value)
        )
    return inverse_resolve(unresolved_name, monkeys, resolved_monkeys, new_target_value)


def solve_second(path: str) -> int:
    monkeys: dict[str, tuple[str, str, str] | int | None] = parse(load_stripped_lines(path))
    monkeys["humn"] = None
    resolved_monkeys: dict[str, int | None] = {m: None for m in monkeys.keys()}
    try_to_resolve("root", monkeys, resolved_monkeys)
    print(resolved_monkeys)

    resolved_branch_name = monkeys["root"][0]
    unresolved_branch_name = monkeys["root"][1]
    if resolved_monkeys[resolved_branch_name] is None:
        tmp = resolved_branch_name
        resolved_branch_name = unresolved_branch_name
        unresolved_branch_name = tmp

    target_value = resolved_monkeys[resolved_branch_name]
    assert target_value is not None
    assert resolved_monkeys[unresolved_branch_name] is None
    print(unresolved_branch_name)
    print(f"{target_value=}")
    return inverse_resolve(unresolved_branch_name, monkeys, resolved_monkeys, target_value)


if __name__ == "__main__":
    test_parse()
    assert solve_first("inputs/21_0") == 152
    assert solve_second("inputs/21_0") == 301
    assert solve_second("inputs/21_2") == 19
    print(solve_second("inputs/21_1"))
