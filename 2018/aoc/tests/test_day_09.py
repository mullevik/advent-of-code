from aoc.day_09 import p1, p2


def test_p1() -> None:
    assert p1("9 players; last marble is worth 25 points") == "32"
    assert p1("10 players; last marble is worth 1618 points") == "8317"
    assert p1("13 players; last marble is worth 7999 points") == "146373"
    assert p1("17 players; last marble is worth 1104 points") == "2764"
    assert p1("21 players; last marble is worth 6111 points") == "54718"
    assert p1("30 players; last marble is worth 5807 points") == "37305"

def test_p2() -> None:
    assert int(p2("9 players; last marble is worth 25 points")) > 32
