from aoc_python.utils import load_lines_as_optional_integers


if __name__ == "__main__":
    calories_of_elves = load_lines_as_optional_integers("inputs/task_01_1.txt")

    elves_calories = []
    tmp_buffer = []
    for calories in calories_of_elves:
        if calories is None:
            elves_calories.append([x for x in tmp_buffer])
            tmp_buffer = []
        else:
            tmp_buffer.append(calories)
    elves_calories.append(tmp_buffer)
    elves_sums = sorted([sum(x) for x in elves_calories], reverse=True)

    print(elves_sums[0] + elves_sums[1] + elves_sums[2])
