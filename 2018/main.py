import argparse
import importlib
import pathlib
import pkgutil
import time
from datetime import timedelta
from types import ModuleType
from typing import Callable

import aoc

DISCOVERED_DAILY_MODULE_NAMES: list[str] = sorted(
    [
        module_info.name
        for module_info in pkgutil.iter_modules(aoc.__path__)
        if "day_" in module_info.name
    ]
)

AUTO_IMPORTED_DAILY_MODULES: list[ModuleType] = [
    importlib.import_module(f"{aoc.__name__}.{module_name}") for module_name in DISCOVERED_DAILY_MODULE_NAMES
]


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "day",
        nargs="*",
        type=int,
        help="which days to evaluate (eg. 1 3) (or leave empty to evaluate all days)",
    )

    args = parser.parse_args()
    days = args.day

    if not days:
        days = [i + 1 for i in range(len(AUTO_IMPORTED_DAILY_MODULES))]

    for d in days:
        evaluate_day(d)


def evaluate_day(day: int) -> None:
    mod = AUTO_IMPORTED_DAILY_MODULES[day - 1]
    inp = (pathlib.Path("inputs") / f"{day:02d}.in").read_text()
    p1_out, p1_dt = measure_fn(getattr(mod, "p1"), inp)
    p2_out, p2_dt = measure_fn(getattr(mod, "p2"), inp)
    print(f"day {day:02d} p1: {p1_out} (in {p1_dt})")
    print(f"day {day:02d} p2: {p2_out} (in {p2_dt})")


def measure_fn(fn: Callable, inp: str) -> tuple[str, timedelta]:
    t0 = time.perf_counter()
    out = fn(inp)
    t1 = time.perf_counter()
    return out, timedelta(seconds=t1 - t0)


if __name__ == "__main__":
    main()
