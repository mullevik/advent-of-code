

from aoc.day_08 import p1, p2


def test_p1() -> None:
    assert p1("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2") == "138"


def test_p2() -> None:
    assert p2("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2") == "66"
