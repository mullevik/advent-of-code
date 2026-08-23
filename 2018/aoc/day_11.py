from functools import cache


def p1(inp: str) -> str:

    serial_no = int(inp)

    x, y, _ = find_highest_square(serial_no, 3, 4)

    return f"{x},{y}"


def p2(inp: str) -> str:
    serial_no = int(inp)

    x, y, s = find_highest_square(serial_no, 1, 300)

    return f"{x},{y},{s}"


GRID_SIZE = 300


def find_highest_square(
    serial_no: int, size_from: int, size_to: int
) -> tuple[int, int, int]:
    highest_level = -1e10
    highest_square = 0, 0, 0

    for y in range(GRID_SIZE):
        for x in range(GRID_SIZE):
            for s in range(
                size_from, min(size_to, GRID_SIZE - y + 1, GRID_SIZE - x + 1)
            ):
                level = square_power_level(x + 1, y + 1, s, serial_no)
                if level > highest_level:
                    highest_level = level
                    highest_square = x + 1, y + 1, s

    return highest_square


@cache
def square_power_level(x: int, y: int, s: int, serial_no: int) -> int:
    if s < 1:
        return 0

    level = square_power_level(x + 1, y + 1, s - 1, serial_no)

    for i_y in range(y + 1, y + s):
        level += square_power_level(x, i_y, 1, serial_no)

    for i_x in range(x + 1, x + s):
        level += square_power_level(i_x, y, 1, serial_no)

    level += power_level(x, y, serial_no)
    return level


def power_level(x: int, y: int, serial_no: int) -> int:
    rack_id = x + 10
    power_level = rack_id * y
    power_level += serial_no
    power_level = power_level * rack_id
    str_power_lvl = str(power_level)
    if len(str_power_lvl) >= 3:
        power_level = int(str_power_lvl[-3])
    else:
        power_level = 0
    return power_level - 5
