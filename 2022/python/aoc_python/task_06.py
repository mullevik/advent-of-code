from aoc_python.utils import load_stripped_lines

if __name__ == "__main__":
    lines = load_stripped_lines("inputs/06_1")
    first_line = lines[0]

    stream_size = 14

    for i in range(len(first_line)):
        substring = first_line[max(0, i - (stream_size - 1)) : i + 1]
        # print(substring)
        if len(substring) == stream_size:
            # print(substring)
            if len(set(substring)) == stream_size:
                print(i + 1)
                exit(0)
