from collections import defaultdict
import time
from aoc_python.utils import load_raw_lines, clear_outputs


def load_stacks(lines: list[str]) -> dict[int, list[str]]:
    columns: dict[int, list[str]] = defaultdict(lambda: [])

    for line in reversed(lines):

        for column_id, i in enumerate(range(0, len(line), 4)):
            create_text = line[i : i + 3]
            if "[" in create_text:
                columns[column_id + 1].append(create_text.replace("[", "").replace("]", "").strip())
    return dict(columns)


def load_instructions(lines: list[str]) -> list[tuple[int, int, int]]:
    return [
        tuple(
            int(x) for x in line.replace("move", "").replace("from", "").replace("to", "").split()
        )
        for line in lines
        if "move" in line
    ]


def print_stacks(stacks: dict[list[str]]):
    print(f"  {'    '.join(str(k) for k in stacks.keys())} ")
    for i in range(max(len(v) for v in stacks.values())):
        out = ""
        for stack in stacks.values():
            if i < len(stack):
                out += f" [{stack[i]}] "
            else:
                out += f"     "
        print(out)


if __name__ == "__main__":
    lines = load_raw_lines("inputs/05_1")
    create_lines = [line for line in lines if "]" in line]
    instruction_lines = lines[len(create_lines) :]
    stacks = load_stacks(create_lines)
    instructions = load_instructions(instruction_lines)
    print_stacks(stacks)

    for ins in instructions:
        for i in range(ins[0]):
            stacks[ins[2]].append(stacks[ins[1]].pop())
        time.sleep(0.015)
        clear_outputs()
        print_stacks(stacks)
        # # stacks[ins[2]].extend(stacks[ins[1]][-ins[0] :])
        # stacks[ins[1]] = stacks[ins[1]][: -ins[0]]

    print("".join(v[-1] for v in stacks.values()))
