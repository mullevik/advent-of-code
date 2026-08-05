import re
from collections import Counter, defaultdict
from collections.abc import Mapping
from typing import Iterable

from aoc.commons import non_empty_lines

type _Interval = tuple[int, int]
type _Date = tuple[int, int, int]


def p1(inp: str) -> str:
    guard_map = parse_in_order(inp)
    guard_sleep_times = {
        g: sum(_int[1] - _int[0] for intervals in days.values() for _int in intervals)
        for g, days in guard_map.items()
    }

    most_asleep_guard_id, _ = max(guard_sleep_times.items(), key=lambda x: x[1])

    guard_data = guard_map[most_asleep_guard_id]

    most_frequent_minute = minute_histogram(guard_data).most_common(1)[0][0]

    return str(most_asleep_guard_id * most_frequent_minute)


def p2(inp: str) -> str:
    guard_map = parse_in_order(inp)

    minute_hist_by_guard_id = {
        g_id: minute_histogram(g_data) for g_id, g_data in guard_map.items()
    }
    most_freq_minute_by_guard_id: dict[int, tuple[int, int]] = {
        g_id: g_hist.most_common(1)[0]
        for g_id, g_hist in minute_hist_by_guard_id.items()
    }

    most_freq_g_id, (most_freq_min, _) = max(
        most_freq_minute_by_guard_id.items(), key=lambda x: x[1][1]
    )
    return str(most_freq_g_id * most_freq_min)


def minute_histogram(guard_data: dict[_Date, list[_Interval]]) -> Counter[int]:
    return Counter(
        [
            x
            for intervals in guard_data.values()
            for int in intervals
            for x in range(int[0], int[1])
        ]
    )


def parse_in_order(inp: str) -> dict[int, dict[_Date, list[_Interval]]]:

    entries = sorted(parse(inp))

    guard_map: Mapping[int, dict[_Date, list[_Interval]]] = defaultdict(
        lambda: defaultdict(list)
    )

    current_guard = 0
    current_guard_sleep_min = 0
    for _date, _minute, _msg in entries:
        if "Guard" in _msg:
            guard_id = int(_msg.split("#")[1].split()[0])
            current_guard = guard_id
        elif "falls asleep" in _msg:
            current_guard_sleep_min = _minute
        elif "wakes up" in _msg:
            guard_map[current_guard][_date].append((current_guard_sleep_min, _minute))
        else:
            raise ValueError(f"unexpected msg '{_msg}'")

    return {k: dict(v) for k, v in guard_map.items()}


LINE_RE = re.compile(r"\[(\d+)-(\d+)-(\d+) \d+:(\d+)\](.+)")


def parse(inp: str) -> Iterable[tuple[_Date, int, str]]:
    for line in non_empty_lines(inp):
        m = LINE_RE.search(line)
        if m is None:
            raise ValueError("regex did not match")

        _y, _m, _d, _min, _msg = m.groups()

        _minute = int(_min)
        _date = int(_y), int(_m), int(_d)
        yield _date, _minute, _msg
