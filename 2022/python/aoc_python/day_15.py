from dataclasses import dataclass
import itertools
from aoc_python.utils import Point, load_stripped_lines


def parse(lines: list[str]) -> dict[Point, Point]:
    sensor_data: dict[Point, Point] = {}

    for line in lines:
        sensor_string, beacon_string = (
            line.replace("Sensor at x=", "").replace("y=", " ").replace("closest beacon is at x=", "").split(":")
        )
        sensor_coordinates = sensor_string.split(",")
        beacon_coordinates = beacon_string.split(",")
        sensor_data[Point(int(sensor_coordinates[0]), int(sensor_coordinates[1]))] = Point(
            int(beacon_coordinates[0]), int(beacon_coordinates[1])
        )
    return sensor_data


def manhattan_distance(a: Point, b: Point) -> int:
    return abs(a.x - b.x) + abs(a.y - b.y)


def is_covered_by_any_sensor(p: Point, distances: dict[Point, int]) -> bool:
    for sensor, closest_distance in distances.items():
        if manhattan_distance(p, sensor) <= closest_distance:
            return True
    return False


def covered_interval_for_sensor(sensor: Point, target_y: int, distance: int) -> tuple[Point, Point] | None:
    p = Point(sensor.x, target_y)
    y_dist = manhattan_distance(sensor, p)
    x_dist = distance - y_dist
    if x_dist < 0:
        return None
    return Point(p.x - x_dist, target_y), Point(p.x + x_dist, target_y)


@dataclass
class IntervalPoint:
    index: int | None
    point: Point
    is_closing: bool


def count_covered_points(intervals: list[tuple[Point, Point]], beacons: set[Point], target_y: int) -> int:
    points = sorted(
        [IntervalPoint(i, interval[0], False) for i, interval in enumerate(intervals)]
        + [IntervalPoint(i, interval[1], True) for i, interval in enumerate(intervals)]
        + [IntervalPoint(None, b, False) for b in beacons if b.y == target_y],
        key=lambda int_p: int_p.point.x,
    )

    open_interval_indices: set[int] = set()
    first_opened_point: Point = Point(0, 0)
    n_encountered_beacons = 0
    acc_sum = 0

    for int_p in points:
        if int_p.index is None:
            if open_interval_indices:
                n_encountered_beacons += 1
            continue
        if not int_p.is_closing:
            if not open_interval_indices:
                first_opened_point = int_p.point
                n_encountered_beacons = 0
            open_interval_indices.add(int_p.index)
            continue
        if int_p.is_closing:
            open_interval_indices.remove(int_p.index)
            if not open_interval_indices:
                acc_sum += (int_p.point.x - first_opened_point.x + 1) - n_encountered_beacons
            continue
        raise ValueError(f"unexpected situation at interval {int_p=}")
    return acc_sum


@dataclass(frozen=True)
class Line:
    slope: int
    shift: int


def extract_lines_for_sensor(center: Point, size: int) -> list[Line]:
    p1 = Point(center.x - size, center.y)
    p2 = Point(center.x + size, center.y)

    return [Line(1, p1.y - p1.x), Line(-1, p1.y + p1.x), Line(1, p2.y - p2.x), Line(-1, p2.y + p2.x)]


def intersect_lines(a: Line, b: Line) -> Point | None:
    if a.slope == b.slope:
        return None
    positive = a if a.slope > 0 else b
    negative = a if a.slope < 0 else b
    x = (negative.shift - positive.shift) // 2
    y = x + positive.shift
    return Point(x, y)


def intersect_sensors(center_a: Point, dist_a: int, center_b: Point, dist_b: int) -> tuple[Point, ...]:
    a_lines = extract_lines_for_sensor(center_a, dist_a)
    b_lines = extract_lines_for_sensor(center_b, dist_b)
    points = {
        p
        for (line_a, line_b) in itertools.combinations(a_lines + b_lines, 2)
        if (p := intersect_lines(line_a, line_b)) is not None
    }
    return tuple(
        p for p in points if manhattan_distance(p, center_a) == dist_a and manhattan_distance(p, center_b) == dist_b
    )


def is_point_covered_by_sensors(p: Point, sensors: dict[Point, int]) -> bool:
    return any(manhattan_distance(p, center_s) <= size_s for center_s, size_s in sensors.items())


def solve_first(path: str, target_y: int) -> int:
    sensor_data = parse(load_stripped_lines(path))
    distances = {sensor: manhattan_distance(sensor, beacon) for sensor, beacon in sensor_data.items()}
    intervals = [
        interval
        for s in sensor_data.keys()
        if (interval := covered_interval_for_sensor(s, target_y, distances[s])) is not None
    ]
    print(f"{intervals=}")
    return count_covered_points(intervals, set(sensor_data.values()), target_y)


def solve_second(path: str, upper_bound: int) -> int:
    sensor_data = parse(load_stripped_lines(path))
    distances = {sensor: manhattan_distance(sensor, beacon) for sensor, beacon in sensor_data.items()}

    intersect_points: set[Point] = {
        p
        for (a_center, a_size), (b_center, b_size) in itertools.combinations(
            ((s_center, s_size) for s_center, s_size in distances.items()), 2
        )
        for p in intersect_sensors(a_center, a_size, b_center, b_size)
    }
    uncovered_points = {
        adj for p in intersect_points for adj in p.four_neighbors if not is_covered_by_any_sensor(adj, distances)
    }
    valid_points = {p for p in uncovered_points if 0 <= p.x <= upper_bound and 0 <= p.y <= upper_bound}
    if len(valid_points) != 1:
        raise ValueError(f"Number of valid points is not exactly one... {valid_points=}")
    return (next(iter(valid_points)).x * 4_000_000) + next(iter(valid_points)).y


def test_parse() -> None:
    sensor_data = parse(load_stripped_lines("inputs/15_0"))
    assert len(sensor_data) == 14
    assert sensor_data[Point(2, 18)] == Point(-2, 15)
    assert sensor_data[Point(20, 14)] == Point(25, 17)


def test_manhattan() -> None:
    assert manhattan_distance(Point(0, 0), Point(0, 5)) == 5
    assert manhattan_distance(Point(0, 0), Point(5, 5)) == 10


def test_lines() -> None:
    assert set(extract_lines_for_sensor(Point(1, 0), 1)) == {Line(-1, 0), Line(1, 0), Line(1, -2), Line(-1, 2)}
    assert set(extract_lines_for_sensor(Point(-2, -2), 2)) == {Line(1, 2), Line(1, -2), Line(-1, -2), Line(-1, -6)}


def test_intersect() -> None:
    assert intersect_lines(Line(1, 123), Line(1, -123)) == None
    assert intersect_lines(Line(1, -1), Line(-1, 1)) == Point(1, 0)
    assert intersect_lines(Line(-1, -2), Line(1, 0)) == Point(-1, -1)


if __name__ == "__main__":
    test_parse()
    test_manhattan()
    test_lines()
    test_intersect()
    assert solve_first("inputs/15_0", 10) == 26
    assert solve_second("inputs/15_0", 20) == 56000011
    print(solve_second("inputs/15_1", 4000000))
    # print(solve("inputs/15_1", 2000000))
