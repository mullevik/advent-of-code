import pytest

from aoc.day_11 import p1, p2, power_level, square_power_level


def test_p1() -> None:
    assert p1("18") == "33,45"


@pytest.mark.slow()
def test_p2() -> None:
    assert p2("18") == "90,269,16"
    assert p2("42") == "232,251,12"


def test_square_power_level() -> None:
    assert square_power_level(33, 45, 3, 18) == 29
    assert square_power_level(21, 61, 3, 42) == 30


def test_power_level() -> None:
    assert power_level(122, 79, 57) == -5
    assert power_level(217, 196, 39) == 0
    assert power_level(101, 153, 71) == 4
