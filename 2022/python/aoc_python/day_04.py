from aoc_python.utils import load_stripped_lines


def does_completely_overlap(line: str) -> bool:
    first_elf, second_elf = line.split(",")

    fe_start, fe_end = [int(x) for x in first_elf.split("-")]
    se_start, se_end = [int(x) for x in second_elf.split("-")]

    if fe_start <= se_start and fe_end >= se_start:
        return True

    if se_start <= fe_start and se_end >= fe_start:
        return True
    return False


if __name__ == "__main__":
    lines = load_stripped_lines("inputs/04_1")

    overlaps = [does_completely_overlap(line) for line in lines]
    # print(overlaps)
    print(sum(overlaps))
