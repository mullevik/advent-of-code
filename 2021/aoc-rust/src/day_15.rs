use std::collections::HashMap;

use crate::grid::{Grid, Point};

pub fn first_part(input: &str) -> i32 {
    let g = parse(input);
    let start = Point::new(0, 0);
    let end = Point::new(g.width() as i32 - 1, g.height() as i32 - 1);
    let mut predecessors: HashMap<Point, Point> = HashMap::new();

    let mut queue = vec![start];

    let mut distances_from_start: HashMap<Point, i32> = HashMap::new();
    distances_from_start.insert(start, 0);

    while !queue.is_empty() {
        let current = extract_min(&mut queue, &distances_from_start);

        let dist_to_current = distances_from_start.get(&current).unwrap().to_owned();

        for (adj_point, adj_val) in g.four_neighborhood_at(&current) {
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
        .map(|p| g.at(p).unwrap())
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
pub fn second_part(input: &str) -> i32 {
    todo!();
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
    use crate::{day_15::first_part, day_15::parse};

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
        assert_eq!(first_part(include_str!("../inputs/15.in")), -1);
    }
}
