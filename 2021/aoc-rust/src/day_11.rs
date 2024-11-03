use std::collections::VecDeque;

use crate::grid::{Grid, Point};

const N_STEPS_FIRST_PART: usize = 100;

pub fn first_part(input: &str) -> i32 {
    let mut octopus_map = parse(input);
    let mut n_flashes = 0;

    for (i, _) in std::iter::repeat(()).enumerate() {
        n_flashes += compute_flash_map(&mut octopus_map)
            .iter()
            .filter(|(_, &val)| val)
            .count() as i32;
        if i + 1 >= N_STEPS_FIRST_PART {
            break;
        }
    }
    n_flashes
}

pub fn second_part(input: &str) -> i32 {
    let mut octopus_map = parse(input);

    for (i, _) in std::iter::repeat(()).enumerate() {
        if compute_flash_map(&mut octopus_map)
            .iter()
            .all(|(_, &val)| val)
        {
            return i as i32 + 1;
        }
    }
    unreachable!()
}

fn compute_flash_map(octopus_map: &mut Grid<i32>) -> Grid<bool> {
    let mut flash_map = Grid::full(octopus_map.width(), octopus_map.height(), false);

    octopus_map.iter_mut().for_each(|(_, val)| *val += 1);

    let mut to_check_stack: VecDeque<Point> = VecDeque::default();

    for (p, val) in octopus_map.iter() {
        if *val > 9 {
            to_check_stack.push_back(p);
        }
    }

    while let Some(to_check) = to_check_stack.pop_front() {
        if *flash_map.at(&to_check).unwrap() {
            continue;
        }
        *flash_map.at_mut(&to_check).unwrap() = true;
        *octopus_map.at_mut(&to_check).unwrap() = 0;

        let adjacent_points = octopus_map
            .eight_neighborhood_at(&to_check)
            .iter()
            .map(|(p, _)| p)
            .cloned()
            .collect::<Vec<_>>();
        for adj in adjacent_points {
            if !(*flash_map.at(&adj).unwrap()) {
                *octopus_map.at_mut(&adj).unwrap() += 1;

                if *octopus_map.at(&adj).unwrap() > 9 {
                    to_check_stack.push_back(adj);
                }
            }
        }
    }

    flash_map
}

fn parse(input: &str) -> Grid<i32> {
    Grid::from_rows(input.lines().map(|line| {
        line.chars()
            .map(|c| c.to_string().parse::<i32>().unwrap())
            .collect()
    }))
    .unwrap()
}

#[cfg(test)]
mod tests_day_11 {
    use super::*;

    const EXAMPLE_INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";

    #[test]
    fn test_parse() {
        let g = parse(EXAMPLE_INPUT);

        assert_eq!(g.width(), 10);
        assert_eq!(g.height(), 10);
        assert_eq!(g.at_xy(0, 0).unwrap().to_owned(), 5);
        assert_eq!(g.at_xy(9, 9).unwrap().to_owned(), 6);
    }

    #[test]
    fn test_example_first_part() {
        assert_eq!(first_part(EXAMPLE_INPUT), 1656);
    }

    #[test]
    fn test_first_part() {
        assert_eq!(first_part(include_str!("../inputs/11.in")), 1627);
    }

    #[test]
    fn test_example_second_part() {
        assert_eq!(second_part(EXAMPLE_INPUT), 195);
    }
    #[test]
    fn test_second_part() {
        assert_eq!(second_part(include_str!("../inputs/11.in")), -1);
    }
}
