from dataclasses import dataclass
from enum import Enum
from typing import Iterable

from aoc_python.utils import load_stripped_lines


class Material(Enum):
    ORE = 0
    CLAY = 1
    OBSIDIAN = 2
    GEODE = 3


@dataclass
class Blueprint:
    ore_cost: dict[Material, int]
    clay_cost: dict[Material, int]
    obsidian_cost: dict[Material, int]
    geode_cost: dict[Material, int]


def parse_blueprint(lines: list[str]) -> Blueprint:

    lines[1].split("costs ")[1].split("and")


MATERIAL_MAP = {"ore": Material.ORE, "clay": Material.CLAY, "obsidian": Material.OBSIDIAN, "geode": Material.GEODE}


def parse(lines: list[str]) -> Iterable[Blueprint]:
    for i, line in enumerate(lines):
        robot_lines = line.split(":")[1].split(".")
        ore_cost = {}
        clay_cost = {}
        obsidian_cost = {}
        geode_cost = {}
        for r_line in robot_lines:
            match r_line.split():
                case ["Each", "ore", "robot", "costs", x, "ore"]:
                    ore_cost = {Material.ORE: int(x)}
                case ["Each", "clay", "robot", "costs", x, "ore"]:
                    clay_cost = {Material.ORE: int(x)}
                case ["Each", "obsidian", "robot", "costs", x, "ore", "and", y, "clay"]:
                    obsidian_cost = {Material.ORE: int(x), Material.CLAY: int(y)}
                case ["Each", "geode", "robot", "costs", x, "ore", "and", y, "obsidian"]:
                    geode_cost = {Material.ORE: int(x), Material.OBSIDIAN: int(y)}
        yield Blueprint(ore_cost, clay_cost, obsidian_cost, geode_cost)


def solve_first(path: str) -> int:
    raise NotImplementedError


def test_parse() -> None:
    blueprints = list(parse(load_stripped_lines("inputs/19_0")))
    assert len(blueprints) == 2
    assert blueprints[0].ore_cost[Material.ORE] == 4
    assert blueprints[1].obsidian_cost[Material.ORE] == 3
    assert blueprints[1].obsidian_cost[Material.CLAY] == 8


if __name__ == "__main__":
    test_parse()
    assert solve_first("inputs/19_0") == 33
