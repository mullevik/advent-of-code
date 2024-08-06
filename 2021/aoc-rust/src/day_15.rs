use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    usize,
};

use crate::grid::{Grid, Point};

pub fn first_part(input: &str) -> i32 {
    let g = parse(input);
    find_value_of_minimal_path(&g, g.width(), g.height())
}

pub fn second_part(input: &str) -> i32 {
    let g = parse(input);

    find_value_of_minimal_path(&g, 5 * g.width(), 5 * g.height())
}
#[derive(Eq, PartialEq)]
struct State {
    point: Point,
    distance_from_start: i32,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.distance_from_start.cmp(&self.distance_from_start)  // reverse order (descending)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

fn find_value_of_minimal_path(grid: &Grid<i32>, total_width: usize, total_height: usize) -> i32 {
    let start = Point::new(0, 0);
    let end = Point::new(total_width as i32 - 1, total_height as i32 - 1);
    let mut predecessors: HashMap<Point, Point> = HashMap::new();

    let mut queue: BinaryHeap<State> = BinaryHeap::new();

    let mut distances_from_start: HashMap<Point, i32> = HashMap::new();
    distances_from_start.insert(start, 0);
    //
    queue.push(State {
        point: start,
        distance_from_start: 0,
    });

    while !queue.is_empty() {
        let current = queue.pop().unwrap();


        for (adj_point, adj_val) in get_adjacents(current.point, &grid, total_width, total_height) {
            let dist_to_adjacent = distances_from_start
                .get(&adj_point)
                .unwrap_or(&i32::MAX)
                .to_owned();

            if current.distance_from_start + adj_val < dist_to_adjacent {
                let new_dist_to_adjacent = current.distance_from_start + adj_val;
                distances_from_start.insert(adj_point, new_dist_to_adjacent);
                predecessors.insert(adj_point, current.point);
                queue.push(State {
                    point: adj_point,
                    distance_from_start: new_dist_to_adjacent,
                });
            }
        }
    }

    let path = extract_path_from_end(&predecessors, &end);
    path[0..path.len() - 1]
        .iter()
        .map(|p| get_value(p.to_owned(), grid))
        .sum::<i32>()
}

fn extract_path_from_end(predecessors: &HashMap<Point, Point>, end: &Point) -> Vec<Point> {
    let mut possible_current: Option<&Point> = Some(end);
    let mut path: Vec<Point> = vec![];

    while let Some(current) = possible_current {
        path.push(current.to_owned());

        possible_current = predecessors.get(&current);
    }

    path
}

fn get_value(p: Point, grid: &Grid<i32>) -> i32 {
    if grid.contains(&p) {
        grid.at(&p).unwrap().to_owned()
    } else {
        let grid_val = grid
            .at_xy(
                (p.x % grid.width() as i32) as usize,
                (p.y % grid.height() as i32) as usize,
            )
            .unwrap()
            .to_owned();
        let x_inc = p.x / grid.width() as i32;
        let y_inc = p.y / grid.height() as i32;

        let val = grid_val + x_inc + y_inc;
        ((val - 1) % 9) + 1
    }
}

fn get_adjacents<'a>(
    p: Point,
    grid: &Grid<i32>,
    total_width: usize,
    total_height: usize,
) -> Vec<(Point, i32)> {
    [
        Point::new(p.x - 1, p.y - 0),
        Point::new(p.x - 0, p.y - 1),
        Point::new(p.x + 1, p.y + 0),
        Point::new(p.x + 0, p.y + 1),
    ]
    .iter()
    .filter(|a| a.x >= 0 && a.x < total_width as i32 && a.y >= 0 && a.y < total_height as i32)
    .map(|a| (a.to_owned(), get_value(a.to_owned(), grid)))
    .collect()
}


fn parse(input: &str) -> Grid<i32> {
    input
        .lines()
        .map(|row| {
            row.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
        .into_iter()
        .collect()
}
#[cfg(test)]
mod tests_day_15 {
    use crate::{
        day_15::{first_part, get_value, parse, second_part},
        grid::Point,
    };

    #[test]
    fn test_get_value() {
        let g = parse(include_str!("../inputs/15_example"));

        assert_eq!(get_value(Point::new(12, 0), &g), 7);
        assert_eq!(get_value(Point::new(11, 11), &g), 5);
        assert_eq!(get_value(Point::new(12, 3), &g), 1);
    }

    #[test]
    fn test_parse() {
        let g = parse(include_str!("../inputs/15_example"));

        assert_eq!(g.width(), 10);
        assert_eq!(g.height(), 10);
        assert_eq!(g.at_xy(0, 0).unwrap(), &1);
        assert_eq!(g.at_xy(2, 0).unwrap(), &6);
    }

    #[test]
    fn test_example_first_part() {
        assert_eq!(first_part(include_str!("../inputs/15_example")), 40);
    }
    #[test]
    fn test_first_part() {
        assert_eq!(first_part(include_str!("../inputs/15.in")), 626);
    }

    #[test]
    fn test_example_second_part() {
        assert_eq!(second_part(include_str!("../inputs/15_example")), 315);
    }
    #[test]
    fn test_second_part() {
        assert_eq!(second_part(include_str!("../inputs/15.in")), 2966);
    }
}
