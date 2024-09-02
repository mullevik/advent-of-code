use std::i32;

use itertools::Itertools;
use regex::Regex;

use crate::{grid::Point, p};

const r#const: i32 = 200;

pub fn first_part(input: &str) -> i32 {
    let target = parse(input);
    generate_possibilities(target)
        .map(|v| {
            if is_target_hit(v, target) {
                Some(ap_sum_from_1_to_n(v.y))
            } else {
                None
            }
        })
        .reduce(|a, b| std::cmp::max(a, b))
        .unwrap()
        .unwrap()
}

pub fn second_part(input: &str) -> i32 {
    let target = parse(input);
    generate_possibilities(target)
        .map(|v| {
            if is_target_hit(v, target) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn generate_possibilities(target: (Point, Point)) -> impl Iterator<Item = Point> {
    let min_x = std::cmp::min(0, target.0.x);
    let max_x = std::cmp::max(0, target.1.x);
    let min_y = std::cmp::min(0, target.0.y);

    (min_x..=max_x)
        .cartesian_product(min_y..=r#const).map(|(x, y)| p!(x, y))
}

fn is_target_hit(initial_velocity: Point, target: (Point, Point)) -> bool {
    let mut v = initial_velocity;
    let mut p = p!(0, 0);
    while p.y >= target.0.y {
        if r#match(p, target) {
            return true;
        }

        p = p + v;

        let new_x = if v.x > 0 {
            v.x - 1
        } else if v.x < 0 {
            v.x + 1
        } else {
            0
        };

        v = p!(new_x, v.y - 1);
    }
    return false;
}

fn ap_sum_from_1_to_n(n: i32) -> i32 {
    (n * (n + 1)) / 2
}

fn r#match(p: Point, target: (Point, Point)) -> bool {
    p.x >= target.0.x && p.x <= target.1.x && p.y >= target.0.y && p.y <= target.1.y
}

fn parse(r#str: &str) -> (Point, Point) {
    let re = Regex::new(r"x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)").unwrap();
    let captures = re.captures(r#str).unwrap();

    (
        Point::new(
            captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
            captures.get(3).unwrap().as_str().parse::<i32>().unwrap(),
        ),
        Point::new(
            captures.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            captures.get(4).unwrap().as_str().parse::<i32>().unwrap(),
        ),
    )
}

#[cfg(test)]
mod tests_day_17 {
    use crate::{
        day_17::{first_part, r#match, parse, second_part},
        grid::Point, p,
    };

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("target area: x=20..30, y=-10..-5"),
            (p!(20, -10), p!(30, -5))
        );
    }

    #[test]
    fn test_logic() {
        assert_eq!(
            r#match(p!(3, 5), (p!(0, 0), p!(10, 10))),
            true
        );
        assert_eq!(
            r#match(p!(-3, 5), (p!(0, 0), p!(10, 10))),
            false
        );
        assert_eq!(
            r#match(p!(10, 10), (p!(0, 0), p!(10, 10))),
            true
        );
    }

    #[test]
    fn test_example_first_part() {
        assert_eq!(first_part("target area: x=20..30, y=-10..-5"), 45);
    }
    #[test]
    fn test_first_part() {
        assert_eq!(first_part(include_str!("../inputs/17.in")), 8646);
    }
    #[test]
    fn test_example_second_part() {
        assert_eq!(second_part("target area: x=20..30, y=-10..-5"), 112);
    }
    #[test]
    fn test_second_part() {
        assert_eq!(second_part(include_str!("../inputs/17.in")), 5945);
    }
}
