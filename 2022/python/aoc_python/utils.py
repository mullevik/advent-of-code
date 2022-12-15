import copy
from dataclasses import dataclass
import itertools
import os
from typing import Any, Generic, TypeVar, Union
from collections.abc import Iterable


def load_raw_lines(path: str) -> list[str]:
    with open(path, "r") as fp:
        return [line for line in fp.readlines()]


def load_stripped_lines(path: str) -> list[str]:
    return [line.strip() for line in load_raw_lines(path)]


def load_lines_as_integers(path: str) -> list[int]:
    return [int(line) for line in load_stripped_lines(path)]


def _parse_int_or_none(x: str) -> int | None:
    try:
        return int(x)
    except ValueError:
        return None


def load_lines_as_optional_integers(path: str) -> list[int | None]:
    return [_parse_int_or_none(line) for line in load_stripped_lines(path)]


def clear_outputs() -> None:
    os.system("cls" if os.name == "nt" else "clear")


T = TypeVar("T")


@dataclass(frozen=True)
class GenericVec2(Generic[T]):
    x: T
    y: T

    @property
    def tuple(self) -> tuple[T, T]:
        return self.x, self.y

    def __eq__(self, __o: object) -> bool:
        if isinstance(__o, tuple):
            return self.tuple == __o
        elif isinstance(__o, GenericVec2):
            return self.tuple == __o.tuple
        raise ValueError(f"unsupported eq for type {type(__o)}")

    def __hash__(self) -> int:
        return self.tuple.__hash__()


@dataclass(frozen=True)
class Point(GenericVec2[int]):
    def __add__(self, other: Union["Point", int]) -> "Point":
        if isinstance(other, Point):
            return Point(self.x + other.x, self.y + other.y)
        return Point(self.x + other, self.y + other)

    def __sub__(self, other: Union["Point", int]) -> "Point":
        if isinstance(other, Point):
            return Point(self.x - other.x, self.y - other.y)
        return Point(self.x - other, self.y - other)

    def __mul__(self, other: int) -> "Point":
        return Point(self.x * other, self.y * other)

    def __floordiv__(self, other: int) -> "Point":
        return Point(self.x // other, self.y // other)

    @property
    def four_neighbours(self) -> tuple["Point", ...]:
        return tuple(
            Point(self.x + x, self.y + y) for x, y in itertools.product([1, 0, -1], [1, 0, -1]) if (x == 0) ^ (y == 0)
        )

    @property
    def eight_neighbours(self) -> tuple["Point", ...]:
        return tuple(
            Point(self.x + x, self.y + y)
            for x, y in itertools.product([1, 0, -1], [1, 0, -1])
            if not (x == 0 and y == 0)
        )

    def __str__(self) -> str:
        return str((self.x, self.y))


def sign(x: int | float) -> int:
    if x < 0:
        return -1
    if x > 0:
        return 1
    return 0


@dataclass(frozen=True)
class Vec2Float(GenericVec2[float]):
    ...


@dataclass
class Grid2(Generic[T]):
    cells: list[list[T]]
    _iter_point: Point = Point(0, 0)

    @classmethod
    def filled_with(cls, w: int, h: int, fill_value: T) -> "Grid2":
        return cls([[fill_value for _ in range(w)] for _ in range(h)])

    @property
    def width(self) -> int:
        return len(self.cells[0])

    @property
    def height(self) -> int:
        return len(self.cells)

    def has(self, p: Point) -> bool:
        return 0 <= p.x < self.width and 0 <= p.y < self.height

    def four_neightbours(self, p: Point) -> tuple[Point, ...]:
        return tuple(n for n in p.four_neighbours if self.has(n))

    def eight_neighbours(self, p: Point) -> tuple[Point, ...]:
        return tuple(n for n in p.eight_neighbours if self.has(n))

    def next(self, p: Point) -> Point | None:
        next_p = p + Point(1, 0)
        if self.has(next_p):
            return next_p
        next_row_p = Point(0, p.y + 1)
        if self.has(next_row_p):
            return next_row_p
        return None

    def __getitem__(self, p: Point | tuple[int, int]) -> T:
        if isinstance(p, tuple):
            p = Point(p[0], p[1])
        if isinstance(p, Point) and self.has(p):
            return self.cells[p.y][p.x]
        raise IndexError(f"can not get item {type(p)} {p}")

    def __setitem__(self, p: Point | tuple[int, int], item: T) -> None:
        if isinstance(p, tuple):
            p = Point(p[0], p[1])
        if isinstance(p, Point) and self.has(p):
            self.cells[p.y][p.x] = item
        else:
            raise IndexError(f"can not set item at {type(p)} {p}")

    def __iter__(self) -> "Grid2":
        return Grid2(copy.deepcopy(self.cells))

    def __next__(self) -> tuple[Point, T]:
        next_p = self.next(self._iter_point)
        if next_p is None:
            raise StopIteration
        self._iter_point = next_p
        return self._iter_point, self[self._iter_point]

    def __str__(self) -> str:
        def draw(item: T) -> str:
            if isinstance(item, bool):
                return "T" if item else "f"
            if isinstance(item, int):
                return f"{item:>2} "
            return str(item)

        return "\n".join(["".join(draw(item) for item in row) for row in self.cells])


if __name__ == "__main__":

    grid = Grid2.filled_with(10, 10, 0)
    center = Point(5, 5)
    out = Point(10, 10)
    assert grid.has(center)
    assert not grid.has(out)

    for n in grid.eight_neighbours(center):
        grid[n] = 1
    for n in grid.four_neightbours(center):
        grid[n] = 2
    for n in grid.eight_neighbours(out):
        grid[n] = 3
    grid[0, 0] = 4
    assert grid[center] == 0
    assert grid[center + Point(1, 0)] == 2
    assert grid[center + Point(-1, -1)] == 1
    assert grid[Point(9, 9)] == 3
    assert grid[(0, 0)] == 4
