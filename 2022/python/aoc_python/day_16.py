from collections import defaultdict
from dataclasses import dataclass
import itertools
import math
from collections.abc import Iterable
from aoc_python.utils import load_stripped_lines
from tqdm import tqdm


@dataclass(frozen=True)
class Node:
    name: str
    flow_rate: int
    adjacent: list[str]


def parse_line(line: str) -> Node:
    parts = line.split()
    name = parts[1]
    flow_rate = int(parts[4].replace("rate=", "").replace(";", ""))
    adjacent = [x.replace(",", "") for x in parts[9:]]
    return Node(name, flow_rate, adjacent)


def parse(lines: list[str]) -> dict[str, Node]:
    return {parse_line(line).name: parse_line(line) for line in lines}


def compute_distances(node_name: str, nodes: dict[str, Node]) -> dict[str, int]:
    queue: list[tuple[str, int]] = [(node_name, 0)]
    distances: dict[str, int] = {node_name: 0}
    while queue:
        current_name, dist = queue.pop(0)
        for adj_name in nodes[current_name].adjacent:
            if adj_name not in distances:
                queue.append((adj_name, dist + 1))
                distances[adj_name] = dist + 1
    return distances


def get_relevant_valves(nodes: dict[str, Node]) -> dict[str, Node]:
    relevant_names = {n.name for n in nodes.values() if n.flow_rate > 0}
    return {
        n.name: Node(n.name, n.flow_rate, [adj for adj in n.adjacent if adj in relevant_names])
        for n in nodes.values()
        if n.flow_rate > 0
    }


def get_relevant_paths(
    valves: dict[str, Node], time: int, start_node_name: str, distances: dict[str, dict[str, int]]
) -> Iterable[tuple[str, ...]]:

    stack: list[tuple[tuple[str, ...], int]] = [((start_node_name,), time)]
    valve_names = set(valves.keys())

    while stack:
        path, remaining_time = stack.pop()
        if len(path) == len(valve_names) + 1:
            yield path
            continue
        for adjacent in valve_names:
            if adjacent in path:
                continue
            distance = distances[path[-1]][adjacent]
            adjacent_time = remaining_time - distance - 1
            if adjacent_time <= 0:
                yield path
                continue
            stack.append((path + (adjacent,), adjacent_time))


def compute_path_cost(
    path: tuple[str, ...], time: int, start_node_name: str, nodes: dict[str, Node], distances: dict[str, dict[str, int]]
) -> int:
    previous = start_node_name
    path_it = iter(path)
    total_flow = 0
    while (current := next(path_it, None)) is not None:
        distance = distances[previous][current]
        time = time - distance - 1
        total_flow += (time + 1) * nodes[current].flow_rate
        previous = current
    return total_flow


def solve_first(path: str, time: int, start_node_name: str) -> int:
    nodes = parse(load_stripped_lines(path))
    distances: dict[str, dict[str, int]] = {name: compute_distances(name, nodes) for name, n in nodes.items()}

    valves = get_relevant_valves(nodes)
    _max = -1
    _arg_max = None
    for _path in get_relevant_paths(valves, time, start_node_name, distances):
        cost = compute_path_cost(_path, time, start_node_name, nodes, distances)
        if cost > _max:
            _max = cost
            _arg_max = _path
            print(f"{_max=}, {_arg_max=}")

    if _arg_max is None:
        ValueError("no solution")
    return _max


def compute_paths_cost(
    path_a: tuple[str, ...],
    path_b: tuple[str, ...],
    time: int,
    nodes: dict[str, Node],
    distances: dict[str, dict[str, int]],
) -> int:
    visited_valves: set[str] = set()
    time_a = time
    time_b = time
    a_index = 1
    b_index = 1
    total_flow = 0

    def open(valve_name: str, remaining_time: int) -> int:
        if valve_name not in visited_valves:
            visited_valves.add(valve_name)
            return (remaining_time + 0) * nodes[valve_name].flow_rate
        return 0

    while a_index < len(path_a) or b_index < len(path_b):
        current_a = path_a[a_index] if a_index < len(path_a) else None
        current_b = path_b[b_index] if b_index < len(path_b) else None
        if current_a is None and current_b is not None:
            remaining_time_b = time_b - distances[current_b][path_b[b_index - 1]] - 1 if current_b is not None else None
            total_flow += open(current_b, remaining_time_b)
            # print(f"B opens {current_b} at {remaining_time_b}")
            # print("B done")
            b_index += 1
            time_b = remaining_time_b
        elif current_a is not None and current_b is None:
            remaining_time_a = time_a - distances[current_a][path_a[a_index - 1]] - 1 if current_a is not None else None
            total_flow += open(current_a, remaining_time_a)
            # print(f"A opens {current_a} at {remaining_time_a}")
            # print("A done")
            a_index += 1
            time_a = remaining_time_a
        elif current_a is not None and current_b is not None:
            remaining_time_a = time_a - distances[current_a][path_a[a_index - 1]] - 1 if current_a is not None else None
            remaining_time_b = time_b - distances[current_b][path_b[b_index - 1]] - 1 if current_b is not None else None
            if remaining_time_b > remaining_time_a:
                addition = open(current_b, remaining_time_b)
                # if addition == 0 and b_index < 6:
                #     return 0
                total_flow += addition
                b_index += 1
                time_b = remaining_time_b
                # print(f"B opens {current_b} at {remaining_time_b}")
            else:
                addition = open(current_a, remaining_time_a)
                # if addition == 0 and a_index < 6:
                #     return 0
                total_flow += addition
                a_index += 1
                time_a = remaining_time_a
                # print(f"A opens {current_a} at {remaining_time_a}")
        else:
            break

    return total_flow


def solve_second(path: str, time: int, start_node_name: str) -> int:
    nodes = parse(load_stripped_lines(path))
    distances: dict[str, dict[str, int]] = {name: compute_distances(name, nodes) for name, n in nodes.items()}

    valves = get_relevant_valves(nodes)

    # cost = compute_paths_cost((""), ("DD", "HH", "EE"), time, nodes, distances)
    # print(f"{cost=}")
    _max = -1
    _arg_max = None
    relevant_paths = list(get_relevant_paths(valves, time, start_node_name, distances))
    for p_a, p_b in tqdm(itertools.combinations(relevant_paths, r=2), total=math.comb(len(relevant_paths), 2)):
        intersect = set.intersection(set(p_a[1:]), set(p_b[1:]))
        if intersect:
            continue
        cost = compute_paths_cost(p_a, p_b, time, nodes, distances)
        if cost > _max:
            _max = cost
            _arg_max = (p_a, p_b)
            print(f"{_max=}, {_arg_max=}")

    if _arg_max is None:
        raise ValueError("no solution")
    return _max


def test_parse() -> None:
    nodes = parse(load_stripped_lines("inputs/16_0"))
    assert len(nodes) == 10
    assert nodes["AA"].name == "AA"
    assert nodes["HH"].flow_rate == 22
    assert nodes["II"].adjacent == ["AA", "JJ"]


if __name__ == "__main__":
    test_parse()
    # assert solve_first("inputs/16_0", 30, "AA") == 1651
    # assert solve_second("inputs/16_0", 26, "AA") == 1707
    print(solve_second("inputs/16_1", 26, "AA"))
