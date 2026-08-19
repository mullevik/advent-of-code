from aoc.day_07 import p1, p2, parse_graph, task_duration

EXAMPLE_INP = (
    "Step C must be finished before step A can begin.\n"
    "Step C must be finished before step F can begin.\n"
    "Step A must be finished before step B can begin.\n"
    "Step A must be finished before step D can begin.\n"
    "Step B must be finished before step E can begin.\n"
    "Step D must be finished before step E can begin.\n"
    "Step F must be finished before step E can begin."
)


def test_p1() -> None:
    assert p1(EXAMPLE_INP) == "CABDFE"


def test_p2() -> None:
    assert p2(EXAMPLE_INP, n_workers=2, task_duration_penalty=0) == "15"


def test_task_duration() -> None:
    assert task_duration("A") == 1
    assert task_duration("C") == 3
    assert task_duration("Z") == 26

def test_parse() -> None:
    assert parse_graph(EXAMPLE_INP) == {
        "C": ["A", "F"],
        "A": ["B", "D"],
        "B": ["E"],
        "D": ["E"],
        "F": ["E"],
    }
