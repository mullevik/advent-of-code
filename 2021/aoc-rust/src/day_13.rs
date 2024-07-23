use rustc_hash::FxHashSet;

use crate::grid::{Grid, Point};

pub fn first_part(input: &str) -> i32 {
    let (points, folds) = parse(input);
    points
        .iter()
        .map(|p| fold_point(p, &folds[0..1]))
        .collect::<FxHashSet<Point>>()
        .len() as i32
}

pub fn second_part(input: &str) -> String {
    let (points, folds) = parse(input);

    let paper_points = points
        .iter()
        .map(|p| fold_point(p, &folds))
        .collect::<FxHashSet<_>>();

    let max_x = paper_points.iter().map(|p| p.x).max().unwrap() + 1;
    let max_y = paper_points.iter().map(|p| p.y).max().unwrap() + 1;

    let mut out: String = "".to_string();
    for y in 0..max_y {
        for x in 0..max_x {
            if paper_points.contains(&Point::new(x, y)) {
                out += "#";
            } else {
                out += " ";
            }
        }
        out += "\n";
    }
    out
}

fn fold_point(point: &Point, folds: &[Fold]) -> Point {
    let mut folded_point = *point;

    for fold in folds.iter() {
        match fold {
            (FoldAxis::X, amount) => {
                if folded_point.x > *amount {
                    folded_point = Point::new(amount - (folded_point.x - amount), folded_point.y);
                }
            }
            (FoldAxis::Y, amount) => {
                if folded_point.y > *amount {
                    folded_point = Point::new(folded_point.x, amount - (folded_point.y - amount));
                }
            }
        }
    }

    folded_point
}

enum FoldAxis {
    X,
    Y,
}

type Fold = (FoldAxis, i32);

fn parse(input: &str) -> (Vec<Point>, Vec<Fold>) {
    let (points, splits) = input.split_once("\n\n").unwrap();
    (parse_points(points), parse_folds(splits))
}

fn parse_points(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter(|row| !row.trim().is_empty())
        .map(|row| {
            let (x, y) = row.split_once(',').unwrap();
            Point::new(x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        })
        .collect::<Vec<Point>>()
}

fn parse_folds(input: &str) -> Vec<Fold> {
    input
        .lines()
        .filter(|row| !row.trim().is_empty())
        .map(|row| {
            if row.contains("x=") {
                let (_, x) = row.split_once("x=").unwrap();
                (FoldAxis::X, x.parse::<i32>().unwrap())
            } else {
                let (_, y) = row.split_once("y=").unwrap();
                (FoldAxis::Y, y.parse::<i32>().unwrap())
            }
        })
        .collect::<Vec<Fold>>()
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
        let out = second_part(include_str!("../inputs/13.in"));
        println!("{}", out);

        assert_eq!(out.lines().count(), 6);
    }
}
