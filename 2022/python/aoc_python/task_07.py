from ctypes import Union
from dataclasses import dataclass, field
from collections.abc import Iterable
from functools import cached_property
from typing import Optional
from aoc_python.utils import load_raw_lines


@dataclass
class FSObject:
    name: str


@dataclass
class File(FSObject):
    size: int

    def __str__(self) -> str:
        return f"{self.name} [{self.size}]"


@dataclass
class Directory(FSObject):
    depth: int
    parent: Optional["Directory"] = None
    contents: dict[str, FSObject] = field(default_factory=lambda: {})

    def __str__(self) -> str:
        out = f"+ {self.name}/\n"
        for fs_object in self.contents.values():
            out += f'{"  " * (self.depth)}- {fs_object}\n'
        return out[:-1] if self.contents else out

    @property
    def size(self):
        return sum(_obj.size for _obj in self.contents.values())

    @property
    def directories(self) -> list["Directory"]:
        return [o for o in self.contents.values() if isinstance(o, Directory)]

    @property
    def files(self) -> list[File]:
        return [o for o in self.contents.values() if isinstance(o, File)]


def parse_ls(line_stack: list[str], cwd: Directory) -> Iterable[FSObject | str]:
    while line_stack:
        line = line_stack.pop()
        print(f"ls parsing line: '{line.strip()}'")
        match line.split():
            case ["$", *_]:
                line_stack.append(line)  # put last line back
                return
            case ["dir", name]:
                yield Directory(name, cwd.depth + 1, cwd, {})
            case [size_sting, name]:
                yield File(name, int(size_sting))


def parse(line_stack: list[str], root: Directory, cwd: Directory) -> Directory:
    while line_stack:
        line = line_stack.pop()
        print(f"parsing line: '{line.strip()}'")
        n_parsed_lines = 0
        match line.split():
            case ["$", "cd", "/"]:
                cwd = root
                n_parsed_lines += 1
            case ["$", "cd", ".."]:
                if cwd.parent is None:
                    raise ValueError(f"encountered None parent")
                cwd = cwd.parent
                n_parsed_lines += 1
            case ["$", "cd", name]:
                if name in cwd.contents and isinstance(cwd.contents[name], Directory):
                    cwd = cwd.contents[name]
                else:
                    cwd.contents[name] = Directory(name, cwd.depth + 1, cwd, {})
                n_parsed_lines += 1
            case ["$", "ls"]:
                for _obj in parse_ls(line_stack, cwd):
                    cwd.contents[_obj.name] = _obj if _obj.name not in cwd.contents else cwd.contents[_obj.name]
            case _:
                raise ValueError(f"parsing error on line: '{line}'")
    return cwd


def iter_dirs_top_down(cwd: Directory) -> Iterable[Directory]:
    yield cwd
    for dir in cwd.directories:
        yield from iter_dirs_top_down(dir)


if __name__ == "__main__":
    lines = load_raw_lines("inputs/07_1")

    _root = Directory("", 0, None, {})
    parse(list(reversed(lines)), _root, _root)

    large_enough_dir_sizes = [size for dir in iter_dirs_top_down(_root) if (size := dir.size) >= 8381165]
    print(sorted(large_enough_dir_sizes)[0])
