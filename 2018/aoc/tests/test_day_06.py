from aoc.day_06 import p1, p2


def test_p1() -> None:

    assert p1("1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9") == "17"


def test_p2() -> None:
    assert p2("1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9", distance_threshold=32) == "16"
