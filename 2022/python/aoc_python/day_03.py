from aoc_python.utils import load_stripped_lines


def get_priority(char: str) -> int:
    assert len(char) == 1
    return ord(char) - 96 if char.islower() else ord(char) - 38


if __name__ == "__main__":
    lines = load_stripped_lines("inputs/03_1")

    n_lines = len(lines)
    priorities = []

    for index in range(0, n_lines, 3):
        common = set(lines[index]).intersection(set(lines[index + 1])).intersection(set(lines[index + 2]))
        if len(common) != 1:
            raise ValueError("not a single common item found")
        priorities.append(get_priority(next(iter(common))))

    print(sum(priorities))
