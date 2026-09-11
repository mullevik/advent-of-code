from aoc.day_13 import Cart, p1, p2, parse

EXAMPLE_TEXT = "\n".join(
    [
        "/->-\\",
        "|   |  /----\\",
        "| /-+--+-\  |",
        "| | |  | v  |",
        "\-+-/  \-+--/",
        "    \------/",
    ]
)

EXAMPLE_TEST_2 = "\n".join(
    [
        "/>-<\\  ",
        "|   |  ",
        "| /<+-\\",
        "| | | v",
        "\\>+</ |",
        "  |   ^",
        "  \\<->/",
    ]
)


def test_p1() -> None:
    assert p1(EXAMPLE_TEXT) == "7,3"


def test_p2() -> None:
    assert p2(EXAMPLE_TEST_2) == "6,4"


def test_parse() -> None:
    world, carts = parse(EXAMPLE_TEXT)
    assert len(world) == 6
    assert all(len(r) == 13 for r in world)
    assert carts == [Cart((2, 0), (1, 0), 0), Cart((9, 3), (0, 1), 0)]
