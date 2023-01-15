from dataclasses import dataclass
from enum import Enum
from functools import partial
import math
import multiprocessing
import time
from typing import Iterable

from aoc_python.utils import load_stripped_lines


class Material(Enum):
    ORE = 0
    CLAY = 1
    OBSIDIAN = 2
    GEODE = 3


@dataclass
class Blueprint:
    costs: tuple[dict[Material, int], ...]


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
        yield Blueprint((ore_cost, clay_cost, obsidian_cost, geode_cost))


@dataclass(frozen=True)
class State:
    minute: int
    robots: tuple[int, ...]
    resources: tuple[int, ...]
    geodes: int

    def __repr__(self) -> str:
        return f"State({self.minute=}, {self.robots=}, {self.resources=}, {self.geodes=})"

    def __str__(self) -> str:
        return self.__repr__()


def collect_resources(resources: Iterable[int], robots: Iterable[int], n_minutes: int) -> Iterable[int]:
    for robot, resource in zip(robots, resources):
        yield resource + (robot * n_minutes)


def pay_for_resources(resources: list[int], blueprint: Blueprint, material: Material) -> list[int]:
    new_resources = [r for r in resources]
    for m, c in blueprint.costs[material.value].items():
        new_resources[m.value] -= c
    return new_resources


def extended_robots(robots: Iterable[int], material: Material) -> Iterable[int]:
    for i, robot in enumerate(robots):
        if i == material.value:
            yield robot + 1
        else:
            yield robot


def expand_material(
    state: State, blueprint: Blueprint, end_time: int, material: Material, max_geodes_so_far: int
) -> Iterable[State]:
    missing_resources = {m: c - state.resources[m.value] for m, c in blueprint.costs[material.value].items()}
    if all(r <= 0 for r in missing_resources.values()):
        minutes_left = end_time - (state.minute + 1)
        if state.minute + 1 > end_time or state.geodes + sum(range(minutes_left + 1)) <= max_geodes_so_far:
            return
        subtracted_resources = pay_for_resources(list(state.resources), blueprint, material)
        collected_resources = tuple(collect_resources(subtracted_resources, state.robots, 1))
        if material == Material.GEODE:
            yield State(
                state.minute + 1,
                state.robots,
                collected_resources,
                state.geodes + minutes_left,
            )
        else:
            yield State(
                state.minute + 1, tuple(extended_robots(state.robots, material)), collected_resources, state.geodes
            )
    else:
        available_in_minutes = {
            m: (math.ceil(m_r / n_robots)) if (n_robots := state.robots[m.value]) > 0 else -1
            for m, m_r in missing_resources.items()
        }
        if any(x == -1 for x in available_in_minutes.values()):
            return  # this robot can never be constructed with current set of robots
        minute_addition = max(available_in_minutes.values())
        minutes_left = end_time - (state.minute + minute_addition + 1)
        # print(f"{missing_resources=}")
        # print(f"{available_in_minutes=}")
        assert minute_addition >= 0
        if (
            state.minute + minute_addition + 1 > end_time
            or state.geodes + sum(range(minutes_left + 1)) <= max_geodes_so_far
        ):
            return
        _collected_resources = list(collect_resources(state.resources, state.robots, minute_addition))
        subtracted_resources = pay_for_resources(_collected_resources, blueprint, material)
        final_resources = tuple(collect_resources(subtracted_resources, state.robots, 1))
        if material == Material.GEODE:
            yield State(
                state.minute + minute_addition + 1,
                state.robots,
                final_resources,
                state.geodes + minutes_left,
            )
        else:
            yield State(
                state.minute + minute_addition + 1,
                tuple(extended_robots(state.robots, material)),
                final_resources,
                state.geodes,
            )


def expand(state: State, blueprint: Blueprint, end_time: int, max_geodes_so_far: int) -> Iterable[State]:

    for material in [Material.ORE, Material.CLAY, Material.OBSIDIAN, Material.GEODE]:
        yield from expand_material(state, blueprint, end_time, material, max_geodes_so_far)


def compute_geode_score(blueprint_pair: tuple[int, Blueprint], end_time: int) -> int:
    index, blueprint = blueprint_pair
    # if index == 0:
    #     return 56
    # if index == 1:
    #     return 62
    initial_state = State(0, (1, 0, 0), (0, 0, 0), 0)
    dfs_stack = [initial_state]
    visited = {initial_state}
    # predecessors: dict[State, State | None] = {initial_state: None}
    t0 = time.perf_counter_ns()

    best_state: State = initial_state
    while dfs_stack:
        current = dfs_stack.pop()
        if current.geodes > best_state.geodes:
            best_state = current
            print(f"{current=}")

        for adjacent in expand(current, blueprint, end_time, best_state.geodes):
            if adjacent not in visited:
                visited.add(adjacent)
                dfs_stack.append(adjacent)
                # predecessors[adjacent] = current
    t1 = time.perf_counter_ns()

    # print("Printing predecessor path from best state to initial state")
    # s: State | None = best_state
    # while s is not None:
    #     print(s)
    #     s = predecessors[s]
    print(f"Blueprint {index + 1} ({index=}) done in {(t1 - t0)=}: {best_state=}")
    return best_state.geodes


def solve_first(path: str) -> int:
    blueprints = list(parse(load_stripped_lines(path)))

    with multiprocessing.Pool(multiprocessing.cpu_count()) as pool:
        geode_scores = pool.map(partial(compute_geode_score, end_time=24), enumerate(blueprints))

    return sum((i + 1) * s for i, s in enumerate(geode_scores))


def solve_second(path: str) -> int:
    blueprints = list(parse(load_stripped_lines(path)))
    with multiprocessing.Pool(multiprocessing.cpu_count()) as pool:
        geode_scores = pool.map(partial(compute_geode_score, end_time=32), enumerate(blueprints[:3]))

    scores = [1, 1, 1]
    for i, s in enumerate(geode_scores):
        scores[i] = s
    return scores[0] * scores[1] * scores[2]


def test_parse() -> None:
    blueprints = list(parse(load_stripped_lines("inputs/19_0")))
    assert len(blueprints) == 2
    assert blueprints[0].costs[0][Material.ORE] == 4
    assert blueprints[1].costs[-2][Material.ORE] == 3
    assert blueprints[1].costs[-2][Material.CLAY] == 8


def test_expand() -> None:

    ex_01 = list(
        expand(
            State(0, (1, 0, 0), (0, 0, 0), 0),
            Blueprint(({Material.ORE: 5}, {Material.ORE: 100}, {Material.ORE: 100}, {Material.ORE: 100})),
            100,
        )
    )
    assert len(ex_01) == 1
    assert ex_01[0] == State(6, (2, 0, 0), (1, 0, 0), 0)

    ex_02 = list(
        expand(
            State(10, (1, 1, 0), (0, 0, 1), 0),
            Blueprint(({Material.ORE: 5}, {Material.ORE: 10}, {Material.OBSIDIAN: 1}, {Material.ORE: 100})),
            100,
        )
    )
    print(f"{ex_02=}")
    assert len(ex_02) == 3
    assert ex_02[0] == State(16, (2, 1, 0), (1, 6, 1), 0)
    assert ex_02[1] == State(21, (1, 2, 0), (1, 11, 1), 0)
    assert ex_02[2] == State(11, (1, 1, 1), (1, 1, 0), 0)

    ex_03 = list(
        expand(
            State(10, (0, 0, 1), (0, 0, 0), 0),
            Blueprint(({Material.ORE: 100}, {Material.ORE: 100}, {Material.ORE: 100}, {Material.OBSIDIAN: 5})),
            24,
        )
    )
    print(f"{ex_03=}")
    assert len(ex_03) == 1
    assert ex_03[0] == State(16, (0, 0, 1), (0, 0, 1), 8)


if __name__ == "__main__":
    # test_parse()
    # test_expand()
    # assert solve_first("inputs/19_0") == 33
    # print(solve_first("inputs/19_1"))

    assert solve_second("inputs/19_0") == 56 * 62
    print(solve_second("inputs/19_1"))
