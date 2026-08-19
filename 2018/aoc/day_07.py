from collections import defaultdict
from collections.abc import Iterable
from itertools import chain


def p1(inp: str) -> str:
    g = parse_graph(inp)
    ordered_ops = ""
    remaining_tasks = set(
        chain(g.keys(), {adj for adjacents in g.values() for adj in adjacents})
    )

    while remaining_tasks:
        free_tasks = get_free_tasks_in_order(g, remaining_tasks)
        to_remove = free_tasks[0]
        ordered_ops += to_remove
        if to_remove in g:
            del g[to_remove]
        remaining_tasks.remove(to_remove)

    return ordered_ops


def p2(inp: str, n_workers: int = 5, task_duration_penalty: int = 60) -> str:
    g = parse_graph(inp)
    workers: list[tuple[str, int] | None] = [None for _ in range(n_workers)]
    time = 0

    remaining_tasks = set(
        chain(g.keys(), {adj for adjacents in g.values() for adj in adjacents})
    )

    waiting_tasks = set()
    assigned_tasks = set()

    while remaining_tasks:
        completed_tasks = list(advance_workers_inplace(workers))

        for t in completed_tasks:
            remaining_tasks.remove(t)
            if t in g:
                del g[t]
            waiting_tasks.discard(t)

        free_tasks = sorted(
            set(chain(get_free_tasks_in_order(g, remaining_tasks), waiting_tasks))
        )

        for ft in free_tasks:
            if ft in assigned_tasks:
                continue
            task_assigned = assign_task_inplace(ft, workers, task_duration_penalty)
            if task_assigned:
                assigned_tasks.add(ft)
            else:
                waiting_tasks.add(ft)

        time += 1
    return str(time - 1)


def advance_workers_inplace(workers: list[tuple[str, int] | None]) -> Iterable[str]:
    for i, w in enumerate(workers):
        if w is not None:
            if w[1] <= 1:
                workers[i] = None
                yield w[0]
            else:
                workers[i] = (w[0], w[1] - 1)


def assign_task_inplace(
    task: str, workers: list[tuple[str, int] | None], task_duration_penalty: int
) -> bool:
    task_assigned = False

    for i, w in enumerate(workers):
        if workers[i] is None:
            task_assigned = True
            workers[i] = (task, task_duration(task) + task_duration_penalty)
            break

    return task_assigned


def task_duration(t: str) -> int:
    return ord(t) - ord("A") + 1


def get_free_tasks_in_order(g: dict[str, list[str]], all_tasks: set[str]) -> list[str]:

    in_degree_map = {t: 0 for t in all_tasks}

    for adjacents in g.values():
        for adj in adjacents:
            in_degree_map[adj] += 1

    return sorted([v for v, deg in in_degree_map.items() if deg == 0])


def parse_graph(inp: str) -> dict[str, list[str]]:
    g = defaultdict(list)

    for line in inp.split("\n"):
        if line.strip():
            parts = line.split(" must be finished before step ")
            _from = parts[0].split(" ")[-1]
            _to = parts[1].split(" ")[0]

            g[_from].append(_to)

    return g
