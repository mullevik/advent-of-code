from aoc.day_01 import p1, p2


def test_p1() -> None:
    assert p1("+1, +1, +1") == "3"
    assert p1("+1, +1, -2") == "0"
    assert p1("-1, -2, -3") == "-6"


def test_p2() -> None:
    assert p2("+1, -1") == "0"
    assert p2("+3, +3, +4, -2, -4") == "10"
    assert p2("-6, +3, +8, +5, -6") == "5"
    assert p2("+7, +7, -2, -7, -4") == "14"
