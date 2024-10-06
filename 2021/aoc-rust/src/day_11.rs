use crate::grid::{Grid, Point};

pub fn first_part(input: &str) -> i32 {
    let mut octopus_map = parse(input);

    todo!();
}

pub fn second_part(input: &str) -> i32 {
    todo!()
}

fn get_flashes_from_step(octopus_map: &mut Grid<i32>) -> i32 {
    octopus_map.iter_mut().for_each(|(_, val)| *val += 1);

    let mut flash_map = Grid::full(octopus_map.width(), octopus_map.height(), false);

    let mut n_flashes = flash_check(octopus_map, &mut flash_map);
    loop {
        println!("{}", n_flashes);
        for (p, has_flashed) in flash_map.iter() {
            if has_flashed.to_owned() {
                let neighbors = octopus_map
                    .eight_neighborhood_at(&p)
                    .into_iter()
                    .map(|(p, _)| p)
                    .collect::<Vec<_>>();
                for adjacent in neighbors.iter() {
                    if !flash_map.at(adjacent).unwrap() {
                        *octopus_map.at_mut(adjacent).unwrap() += 1
                    }
                }
            }
        }

        let n_flashes_after = flash_check(octopus_map, &mut flash_map);
        if n_flashes_after > n_flashes {
            n_flashes = n_flashes_after;
        } else {
            break;
        }
    }

    // todo: stack based method is actually needed .... :(
    // todo: this wont work :(
    n_flashes
}

fn flash_check(octopus_map: &mut Grid<i32>, flash_map: &mut Grid<bool>) -> i32 {
    for (p, val) in octopus_map.iter_mut() {
        if *val > 9 {
            *flash_map.at_mut(&p).unwrap() = true;
            *val = 0;
        }
    }

    flash_map.iter_values().filter(|x| **x).count() as i32
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
    fn test_step() {
        let mut g = parse(EXAMPLE_INPUT);
        let n_step_1_flashes = get_flashes_from_step(&mut g);
        let n_step_2_flashes = get_flashes_from_step(&mut g);

        assert_eq!(n_step_1_flashes, 0);
        assert_eq!(n_step_2_flashes, 35);
    }

    #[test]
    fn test_first_part() {
        assert_eq!(first_part(include_str!("../inputs/11.in")), -1);
    }
    #[test]
    fn test_second_part() {
        assert_eq!(second_part(include_str!("../inputs/11.in")), -1);
    }
}
