from aoc.day_12 import p1, parse

EXAMPLE_INPUT = """
initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #
"""


def test_p1() -> None:
    assert p1(EXAMPLE_INPUT) == "325"


def test_parse() -> None:

    pots, rules = parse(EXAMPLE_INPUT)

    plant_indices = [i for i, p in pots.items() if p]
    assert plant_indices == [0, 3, 5, 8, 9, 16, 17, 18, 22, 23, 24]

    assert len(rules) == 14
    assert all(r[1] for r in rules)
    assert all(any(not x for x in r[0]) for r in rules)
