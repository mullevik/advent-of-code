from dataclasses import dataclass
from time import sleep
from aoc_python.utils import Point2, clear_outputs, load_raw_lines, sign


@dataclass
class Instruction:
    value: int
    cycles: int


def parse(line: str) -> Instruction:
    match line.split():
        case ["noop"]:
            return Instruction(0, 1)
        case ["addx", value_string]:
            return Instruction(int(value_string), 2)
        case _:
            raise ValueError(f"unknown line '{line}'")


def solve_first(path: str) -> int:
    lines = load_raw_lines(path)
    instructions: list[Instruction] = [parse(line) for line in lines]

    current_register_value = 1
    register_values: list[int] = []

    for ins in instructions:
        register_values.extend([current_register_value] * ins.cycles)
        current_register_value += ins.value

    return sum(register_values[c - 1] * c if c - 1 < len(register_values) else 0 for c in [20, 60, 100, 140, 180, 220])


@dataclass
class CRTScreen:
    data: list[list[bool]]
    width: int
    sprite_size: int

    @classmethod
    def build(cls) -> "CRTScreen":
        return cls([[]], 40, 3)

    def draw(self, sprite_middle: int) -> None:
        cursor = len(self.data[-1])
        if cursor >= self.width:
            self.data.append([])
            cursor = 0
        half_sprite_size = self.sprite_size // 2
        if sprite_middle - half_sprite_size <= cursor <= sprite_middle + half_sprite_size:
            self.data[-1].append(True)
        else:
            self.data[-1].append(False)

    def __str__(self) -> str:
        out = "    " + "".join(str(x % 10) for x in range(self.width)) + "\n"
        for i, row in enumerate(self.data):
            out += f"{i:>3}|"
            for j, pixel in enumerate(row):
                if pixel:
                    out += "#"
                else:
                    out += "."
            out += "\n"
        return out


def solve_second(path: str) -> bool:
    lines = load_raw_lines(path)
    instructions: list[Instruction] = [parse(line) for line in lines]

    current_register_value = 1
    register_values: list[int] = []

    crt = CRTScreen.build()

    for ins in instructions:
        for _ in range(ins.cycles):
            register_values.append(current_register_value)
            crt.draw(current_register_value)
            clear_outputs()
            print(crt)
            sleep(0.01)
        current_register_value += ins.value

    return True


if __name__ == "__main__":
    assert solve_first("inputs/10_0") == 0
    assert solve_first("inputs/10_2") == 13140
    assert solve_second("inputs/10_2")

    # print(solve_first("inputs/10_1"))
    # solve_second("inputs/10_1")
