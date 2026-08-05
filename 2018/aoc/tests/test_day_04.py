import random

from aoc.day_04 import p1, p2, parse_in_order

EXAMPLE_INP = (
    "[1518-11-01 00:00] Guard #10 begins shift\n"
    "[1518-11-01 00:05] falls asleep\n"
    "[1518-11-01 00:25] wakes up\n"
    "[1518-11-01 00:30] falls asleep\n"
    "[1518-11-01 00:55] wakes up\n"
    "[1518-11-01 23:58] Guard #99 begins shift\n"
    "[1518-11-02 00:40] falls asleep\n"
    "[1518-11-02 00:50] wakes up\n"
    "[1518-11-03 00:05] Guard #10 begins shift\n"
    "[1518-11-03 00:24] falls asleep\n"
    "[1518-11-03 00:29] wakes up\n"
    "[1518-11-04 00:02] Guard #99 begins shift\n"
    "[1518-11-04 00:36] falls asleep\n"
    "[1518-11-04 00:46] wakes up\n"
    "[1518-11-05 00:03] Guard #99 begins shift\n"
    "[1518-11-05 00:45] falls asleep\n"
    "[1518-11-05 00:55] wakes up\n"
)


def test_parse() -> None:
    input_lines = [line for line in EXAMPLE_INP.split("\n") if line.strip()]
    random.shuffle(input_lines)
    assert parse_in_order("\n".join(input_lines)) == {
        10: {
            (1518, 11, 1): [(5, 25), (30, 55)],
            (1518, 11, 3): [(24, 29)],
        },
        99: {
            (1518, 11, 2): [(40, 50)],
            (1518, 11, 4): [(36, 46)],
            (1518, 11, 5): [(45, 55)],
        },
    }


def test_p1() -> None:
    assert p1(EXAMPLE_INP) == "240"

def test_p2() -> None:
    assert p2(EXAMPLE_INP) == "4455"
