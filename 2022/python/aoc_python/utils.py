import copy
from dataclasses import dataclass, field
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


class GenericVec(Generic[T]):
    def __init__(self, data: tuple[T, ...]) -> None:
        super().__init__()
        self.data = data

    @property
    def tuple(self) -> tuple[T, ...]:
        return self.data

    def __eq__(self, __o: object) -> bool:
        if isinstance(__o, tuple):
            return self.tuple == __o
        elif isinstance(__o, GenericVec):
            return self.tuple == __o.tuple
        raise ValueError(f"unsupported eq for type {type(__o)}")

    def __hash__(self) -> int:
        return self.tuple.__hash__()


class GenericVec2(GenericVec[T]):
    def __init__(self, x: T, y: T) -> None:
        super().__init__((x, y))

    @property
    def x(self) -> T:
        return self.data[0]

    @property
    def y(self) -> T:
        return self.data[1]

    @property
    def tuple(self) -> tuple[T, T]:
        return self.data[0], self.data[1]


class GenericVec3(GenericVec[T]):
    def __init__(self, x: T, y: T, z: T) -> None:
        super().__init__((x, y, z))

    @property
    def x(self) -> T:
        return self.data[0]

    @property
    def y(self) -> T:
        return self.data[1]

    @property
    def z(self) -> T:
        return self.data[2]

    @property
    def tuple(self) -> tuple[T, T, T]:
        return self.data[0], self.data[1], self.data[2]


class Point2(GenericVec2[int]):
    def __init__(self, x: int, y: int) -> None:
        super().__init__(x, y)

    def __add__(self, other: Union["Point2", int]) -> "Point2":
        if isinstance(other, Point2):
            return Point2(self.x + other.x, self.y + other.y)
        return Point2(self.x + other, self.y + other)

    def __sub__(self, other: Union["Point2", int]) -> "Point2":
        if isinstance(other, Point2):
            return Point2(self.x - other.x, self.y - other.y)
        return Point2(self.x - other, self.y - other)

    def __mul__(self, other: int) -> "Point2":
        return Point2(self.x * other, self.y * other)

    def __floordiv__(self, other: int) -> "Point2":
        return Point2(self.x // other, self.y // other)

    @property
    def four_neighbors(self) -> tuple["Point2", ...]:
        return tuple(
            Point2(self.x + x, self.y + y) for x, y in itertools.product([1, 0, -1], [1, 0, -1]) if (x == 0) ^ (y == 0)
        )

    @property
    def eight_neighbors(self) -> tuple["Point2", ...]:
        return tuple(
            Point2(self.x + x, self.y + y)
            for x, y in itertools.product([1, 0, -1], [1, 0, -1])
            if not (x == 0 and y == 0)
        )

    def __str__(self) -> str:
        return str((self.x, self.y))


class Point3(GenericVec3[int]):
    def __init__(self, x: int, y: int, z: int) -> None:
        super().__init__(x, y, z)

    def __add__(self, other: Union["Point3", int]) -> "Point3":
        if isinstance(other, Point3):
            return Point3(self.x + other.x, self.y + other.y, self.z + other.z)
        return Point3(self.x + other, self.y + other, self.z + other)

    def __sub__(self, other: Union["Point3", int]) -> "Point3":
        if isinstance(other, Point3):
            return Point3(self.x - other.x, self.y - other.y, self.z - other.z)
        return Point3(self.x - other, self.y - other, self.z - other)

    def __mul__(self, other: int) -> "Point3":
        return Point3(self.x * other, self.y * other, self.z * other)

    def __floordiv__(self, other: int) -> "Point3":
        return Point3(self.x // other, self.y // other, self.z // other)

    @property
    def six_neighbors(self) -> tuple["Point3", ...]:
        return tuple(
            [
                self + Point3(1, 0, 0),
                self + Point3(-1, 0, 0),
                self + Point3(0, 1, 0),
                self + Point3(0, -1, 0),
                self + Point3(0, 0, 1),
                self + Point3(0, 0, -1),
            ]
        )

    def __str__(self) -> str:
        return str((self.x, self.y, self.z))


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
    _iter_point: Point2 = Point2(0, 0)

    @classmethod
    def filled_with(cls, w: int, h: int, fill_value: T) -> "Grid2":
        return cls([[fill_value for _ in range(w)] for _ in range(h)])

    @property
    def width(self) -> int:
        return len(self.cells[0])

    @property
    def height(self) -> int:
        return len(self.cells)

    def has(self, p: Point2) -> bool:
        return 0 <= p.x < self.width and 0 <= p.y < self.height

    def four_neighbors(self, p: Point2) -> tuple[Point2, ...]:
        return tuple(n for n in p.four_neighbors if self.has(n))

    def eight_neighbors(self, p: Point2) -> tuple[Point2, ...]:
        return tuple(n for n in p.eight_neighbors if self.has(n))

    def next(self, p: Point2) -> Point2 | None:
        next_p = p + Point2(1, 0)
        if self.has(next_p):
            return next_p
        next_row_p = Point2(0, p.y + 1)
        if self.has(next_row_p):
            return next_row_p
        return None

    def __getitem__(self, p: Point2 | tuple[int, int]) -> T:
        if isinstance(p, tuple):
            p = Point2(p[0], p[1])
        if isinstance(p, Point2) and self.has(p):
            return self.cells[p.y][p.x]
        raise IndexError(f"can not get item {type(p)} {p}")

    def __setitem__(self, p: Point2 | tuple[int, int], item: T) -> None:
        if isinstance(p, tuple):
            p = Point2(p[0], p[1])
        if isinstance(p, Point2) and self.has(p):
            self.cells[p.y][p.x] = item
        else:
            raise IndexError(f"can not set item at {type(p)} {p}")

    def __iter__(self) -> "Grid2":
        return Grid2(copy.deepcopy(self.cells))

    def __next__(self) -> tuple[Point2, T]:
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
    center = Point2(5, 5)
    out = Point2(10, 10)
    assert grid.has(center)
    assert not grid.has(out)

    for n in grid.eight_neighbors(center):
        grid[n] = 1
    for n in grid.four_neighbors(center):
        grid[n] = 2
    for n in grid.eight_neighbors(out):
        grid[n] = 3
    grid[0, 0] = 4
    assert grid[center] == 0
    assert grid[center + Point2(1, 0)] == 2
    assert grid[center + Point2(-1, -1)] == 1
    assert grid[Point2(9, 9)] == 3
    assert grid[(0, 0)] == 4
