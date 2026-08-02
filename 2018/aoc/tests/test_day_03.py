from aoc.day_03 import p1, p2, parse


def test_parse() -> None:
    assert list(parse("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2\n")) == [
        (1, (1, 3), (4, 4)),
        (2, (3, 1), (4, 4)),
        (3, (5, 5), (2, 2)),
    ]


def test_p1() -> None:
    assert p1("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2\n") == "4"


def test_p2() -> None:
    assert p2("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2\n") == "3"
