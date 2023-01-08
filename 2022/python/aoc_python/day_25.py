from aoc_python.utils import load_stripped_lines


def snafu_to_decimal(snafu: str) -> int:
    n = len(snafu)
    decimal = 0
    for i in range(n):
        d = n - i - 1
        s = snafu[i]
        if s == "1":
            decimal += 5**d
        elif s == "2":
            decimal += 2 * (5**d)
        elif s == "-":
            decimal -= 5**d
        elif s == "=":
            decimal -= 2 * (5**d)
        elif s == "0":
            pass
        else:
            raise ValueError(f"unexpected {s=}")

    return decimal


FIVE_POWERS = [5**d for d in range(50)]


def decimal_to_snafu(decimal: int) -> str:
    snafu = ""
    five = []
    i = 1
    while decimal != 0:
        five.append(decimal % 5)
        decimal = decimal // 5
        i += 1

    snafu = ""
    carry = False
    for f in five:
        if carry:
            f += 1
        carry = False
        if f > 4:
            snafu += "0"
            carry = True
        elif f == 3:
            snafu += "="
            carry = True
        elif f == 4:
            snafu += "-"
            carry = True
        else:
            snafu += str(f)

    if carry:
        snafu += "1"

    return "".join(reversed(snafu))


def solve_first(path: str) -> str:
    snafu_numbers = load_stripped_lines(path)

    for snafu in snafu_numbers:
        print(f"{snafu=}\t\t{snafu_to_decimal(snafu)}")

    total = sum(snafu_to_decimal(s) for s in snafu_numbers)

    return decimal_to_snafu(total)


if __name__ == "__main__":
    assert solve_first("inputs/25_0") == "2=-1=0"
    print(solve_first("inputs/25_1"))
