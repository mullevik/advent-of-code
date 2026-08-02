
def non_empty_lines(inp: str) -> list[str]:
    return [line for line in inp.split("\n") if line.strip()]
