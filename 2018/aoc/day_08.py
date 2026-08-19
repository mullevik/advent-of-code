from collections.abc import Iterable
from dataclasses import dataclass

from more_itertools import take


def p1(inp: str) -> str:
    _inp = [int(x.strip()) for x in inp.split()]
    return str(parse(iter(_inp)).meta_sum())


def p2(inp: str) -> str:
    _inp = [int(x.strip()) for x in inp.split()]
    return str(parse(iter(_inp)).value())


@dataclass
class Node:
    children: list["Node"]
    metadata: list[int]

    def meta_sum(self) -> int:
        return sum(self.metadata) + sum(ch.meta_sum() for ch in self.children)

    def value(self) -> int:
        if self.children:
            return sum(
                self.children[meta_val - 1].value()
                for meta_val in self.metadata
                if 0 <= meta_val - 1 < len(self.children)
            )
        else:
            return sum(self.metadata)


def parse(inp: Iterable[int]) -> Node:
    header = take(2, inp)

    n_children = header[0]
    n_metadata = header[1]

    children = [parse(inp) for _ in range(n_children)]

    meta = list(take(n_metadata, inp))

    return Node(children, meta)
