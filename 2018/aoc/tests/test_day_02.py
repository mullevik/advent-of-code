from aoc.day_02 import p1, p2


def test_p1() -> None:
    inp = """
abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab
"""
    assert p1(inp) == "12"


def test_p2() -> None:
    inp = """
abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz
"""
    assert p2(inp) == "fgij"
