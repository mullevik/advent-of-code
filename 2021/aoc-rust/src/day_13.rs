use std::collections::HashSet;

use crate::grid::{Grid, Point};

pub fn first_part(input: &str) -> i32 {
    let (points, splits) = parse(input);

    let mut paper_points: HashSet<Point> = points.iter().cloned().collect::<HashSet<_>>();

    for split in &splits[0..1] {
        paper_points = fold(&paper_points, split);
    }

    paper_points.len() as i32
}

pub fn second_part(input: &str) -> i32 {
    let (points, splits) = parse(input);

    let mut paper_points: HashSet<Point> = points.iter().cloned().collect::<HashSet<_>>();

    for split in splits.iter() {
        paper_points = fold(&paper_points, split);
    }

    let max_x = paper_points.iter().map(|p| p.x).max().unwrap() + 1;
    let max_y = paper_points.iter().map(|p| p.y).max().unwrap() + 1;
    let grid = Grid::from_rows(
        (0..max_y)
            .map(|_| vec!["."; max_x as usize])
            .collect::<Vec<_>>()
            .iter()
            .cloned(),
    )
    .unwrap();

    for p in grid.iter_points() {
        if paper_points.contains(&p) {
            print!("#");
        } else {
            print!(" ");
        }
        if p.x == grid.width() as i32 - 1 {
            println!();
        }
    }

    paper_points.len() as i32
}

fn fold(points: &HashSet<Point>, split: &Split) -> HashSet<Point> {
    let mut addition: HashSet<Point> = HashSet::default();
    let mut deletion: HashSet<Point> = HashSet::default();
    if split.0 {
        // x

        for pp in points.iter() {
            if pp.x > split.1 {
                deletion.insert(*pp);
                let diff = pp.x - split.1;
                let new_x = split.1 - diff;
                addition.insert(Point::new(new_x, pp.y));
            }
        }
    } else {
        // y
        for pp in points.iter() {
            if pp.y > split.1 {
                deletion.insert(*pp);
                let diff = pp.y - split.1;
                let new_y = split.1 - diff;
                addition.insert(Point::new(pp.x, new_y));
            }
        }
    }

    points
        .union(&addition)
        .cloned()
        .collect::<HashSet<Point>>()
        .difference(&deletion)
        .cloned()
        .collect()
}

type Split = (bool, i32);

fn parse(input: &str) -> (Vec<Point>, Vec<Split>) {
    let (points, splits) = input.split_once("\n\n").unwrap();
    (parse_points(points), parse_splits(splits))
}

fn parse_points(input: &str) -> Vec<Point> {
    input
        .split('\n')
        .filter(|row| !row.trim().is_empty())
        .map(|row| {
            let (x, y) = row.split_once(',').unwrap();
            Point::new(x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        })
        .collect::<Vec<Point>>()
}

fn parse_splits(input: &str) -> Vec<Split> {
    input
        .split('\n')
        .filter(|row| !row.trim().is_empty())
        .map(|row| {
            if row.contains("x=") {
                let (_, x) = row.split_once("x=").unwrap();
                (true, x.parse::<i32>().unwrap())
            } else {
                let (_, y) = row.split_once("y=").unwrap();
                (false, y.parse::<i32>().unwrap())
            }
        })
        .collect::<Vec<Split>>()
}

mod tests_day_13 {
    use crate::{day_13::first_part, day_13::parse, day_13::second_part};

    #[test]
    fn test_parsing() {
        let (points, splits) = parse(include_str!("../inputs/13_example"));
        assert_eq!(points.len(), 18);
        assert_eq!(splits.len(), 2);
    }

    #[test]
    fn test_first_part_on_example() {
        assert_eq!(first_part(include_str!("../inputs/13_example")), 17);
    }

    #[test]
    fn test_first_part() {
        assert_eq!(first_part(include_str!("../inputs/13.in")), 775);
    }

    #[test]
    fn test_second_part() {
        assert_eq!(second_part(include_str!("../inputs/13.in")), 102);
    }
}
