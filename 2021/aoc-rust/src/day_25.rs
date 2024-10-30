use itertools::Itertools;
use rustc_hash::FxHashSet;

use crate::grid::{Grid, Point};
use crate::p;

pub fn first_part(input: &str) -> i32 {
    let mut world = parse(input);

    for (i, _) in std::iter::repeat(()).enumerate() {
        let x_has_moved = r#move(&mut world, &p!(1, 0), '>');
        let y_has_moved = r#move(&mut world, &p!(0, 1), 'v');
        if !(x_has_moved || y_has_moved) {
            return i as i32 + 1;
        }
    }
    unreachable!()
}

pub fn second_part(input: &str) -> i32 {
    -1
}

fn r#move(world: &mut Grid<char>, direction: &Point, target_cucumber: char) -> bool {
    let mut can_move: FxHashSet<Point> = FxHashSet::default();

    // let mut can_move = Grid::full(world.width(), world.height(), false);

    for (p, &cucumber) in world.iter() {
        if cucumber == target_cucumber && is_unoccupied(&bound(&(p + *direction), world), &world) {
            can_move.insert(p);
        }
    }

    for (x, y) in (0..world.width()).cartesian_product(0..world.height()) {
        let p = Point::new(x as i32, y as i32);
        if *world.at(&p).unwrap() == target_cucumber && can_move.contains(&p) {
            let target_p = bound(&(p + *direction), world);
            *world.at_mut(&target_p).unwrap() = target_cucumber;
            *world.at_mut(&p).unwrap() = '.';
        }
    }
    !can_move.is_empty()
}

fn bound(p: &Point, g: &Grid<char>) -> Point {
    Point::new(p.x % g.width() as i32, p.y % g.height() as i32)
}

fn is_unoccupied(p: &Point, g: &Grid<char>) -> bool {
    g.at(p).unwrap() == &'.'
}

fn parse(input: &str) -> Grid<char> {
    Grid::from_rows(input.lines().map(|line| {
        line.chars()
            .filter(|&c| c == '.' || c == '>' || c == 'v')
            .collect::<Vec<_>>()
    }))
    .unwrap()
}
#[cfg(test)]
mod tests_day_25 {
    use super::*;

    const EXAMPLE_INPUT: &str = "..........
.>v....v..
.......>..
..........";

    const EXAMPLE_INPUT_2: &str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    #[test]
    fn test_parse() {
        let g = parse(EXAMPLE_INPUT);

        assert_eq!(g.width(), 10);
        assert_eq!(g.height(), 4);
    }
    #[test]
    fn test_first_part_example() {
        assert_eq!(first_part(EXAMPLE_INPUT_2), 58);
    }

    #[test]
    fn test_first_part() {
        assert_eq!(first_part(include_str!("../inputs/25.in")), 516);
    }
    #[test]
    fn test_second_part() {
        assert_eq!(second_part(include_str!("../inputs/25.in")), -1);
    }
}
