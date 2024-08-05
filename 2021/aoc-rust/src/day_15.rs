use std::{collections::HashMap, usize};

use crate::grid::{Grid, Point};

pub fn first_part(input: &str) -> i32 {
    let g = parse(input);
    find_value_of_minimal_path(&g, g.width(), g.height())
}
pub fn second_part(input: &str) -> i32 {
    let g = parse(input);

    find_value_of_minimal_path(&g, 5 * g.width(), 5 * g.height())
}

fn find_value_of_minimal_path(grid: &Grid<i32>, total_width: usize, total_height: usize) -> i32 {
    let start = Point::new(0, 0);
    let end = Point::new(total_width as i32 - 1, total_height as i32 - 1);
    let mut predecessors: HashMap<Point, Point> = HashMap::new();

    let mut queue = vec![start];

    let mut distances_from_start: HashMap<Point, i32> = HashMap::new();
    distances_from_start.insert(start, 0);

    while !queue.is_empty() {
        let current = extract_min(&mut queue, &distances_from_start);

        let dist_to_current = distances_from_start.get(&current).unwrap().to_owned();

        for (adj_point, adj_val) in get_adjacents(current, &grid, total_width, total_height) {
            let dist_to_adjacent = distances_from_start
                .get(&adj_point)
                .unwrap_or(&i32::MAX)
                .to_owned();

            if dist_to_current + adj_val < dist_to_adjacent {
                distances_from_start.insert(adj_point, dist_to_current + adj_val);
                predecessors.insert(adj_point, current);
                queue.push(adj_point);
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


fn extract_min(queue: &mut Vec<Point>, distances_from_start: &HashMap<Point, i32>) -> Point {
    let min = queue
        .iter()
        .map(|p| distances_from_start.get(p).unwrap_or(&i32::MAX))
        .min()
        .unwrap();
    let pos = queue
        .iter()
        .position(|p| distances_from_start.get(p).unwrap_or(&i32::MAX) == min)
        .unwrap();

    queue.remove(pos)
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
        day_15::{first_part, second_part, get_value, parse},
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
