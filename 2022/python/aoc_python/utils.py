def load_lines(path: str) -> list[str]:
    with open(path, "r") as fp:
        return [line.strip() for line in fp.readlines()]


def load_lines_as_integers(path: str) -> list[int]:
    return [int(line) for line in load_lines(path)]


def _parse_int_or_none(x: str) -> int | None:
    try:
        return int(x)
    except ValueError:
        return None


def load_lines_as_optional_integers(path: str) -> list[int | None]:
    return [_parse_int_or_none(line) for line in load_lines(path)]
