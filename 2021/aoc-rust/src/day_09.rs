use std::collections::{HashSet, VecDeque};

use rustc_hash::FxHashMap;

use crate::grid::{Grid, Point};

const WALL_VALUE: i32 = 9;

pub fn first_part(input: &str) -> i32 {
    let g = parse(input);

    g.iter()
        .filter(|(p, v)| is_local_minimum(p, &g))
        .map(|(_, v)| v + 1)
        .sum::<i32>()
}

pub fn second_part(input: &str) -> i32 {
    let g = parse(input);
    let mut dfs_stack = g
        .iter_points()
        .filter(|p| is_local_minimum(p, &g))
        .collect::<VecDeque<_>>();

    let mut visited = dfs_stack.iter().copied().collect::<HashSet<_>>();
    let mut basin_ids = dfs_stack
        .iter()
        .copied()
        .enumerate()
        .map(|(i, p)| (p, i))
        .collect::<FxHashMap<_, _>>();

    while !dfs_stack.is_empty() {
        let current: Point = dfs_stack.pop_back().expect("is not empty");
        for (ad_p, ad_v) in g.four_neighborhood_at(&current) {
            if !visited.contains(&ad_p) && (*ad_v < WALL_VALUE) {
                visited.insert(ad_p);
                dfs_stack.push_back(ad_p);

                let current_basin_id = *basin_ids.get(&current).expect("must have basin id");

                if let Some(ad_basin_id) = basin_ids.get(&ad_p) {
                    if *ad_basin_id != current_basin_id {
                        let dummy = *ad_basin_id;  // ask: why can't I put this into the argument
                        re_assign_basin(&mut basin_ids, dummy, current_basin_id);
                    }
                } else {
                    basin_ids.insert(ad_p, current_basin_id);
                }
            }
        }
    }

    multiply_top_three_basins(&basin_ids)
}

fn multiply_top_three_basins(basin_ids: &FxHashMap<Point, usize>) -> i32 {
    let mut counter: FxHashMap<usize, usize> = FxHashMap::default();

    for (p, basin_id) in basin_ids.iter() {
        if let Some(count) = counter.get(basin_id) {
            counter.insert(*basin_id, *count + 1);
        } else {
            counter.insert(*basin_id, 1);
        }
    }

    let mut sizes: Vec<usize> = counter.iter().map(|(k, v)| *v).collect::<Vec<_>>();

    sizes.sort();
    sizes
        .iter()
        .rev()
        .take(3)
        .product::<usize>() as i32
}

fn re_assign_basin(basin_ids: &mut FxHashMap<Point, usize>, from: usize, to: usize) {
    basin_ids.iter_mut().for_each(|(_, v)| {
        if v == &from {
            *v = to
        }
    });
}

fn is_local_minimum(p: &Point, g: &Grid<i32>) -> bool {
    if let Ok(v) = g.at(p) {
        g.four_neighborhood_at(p).iter().all(|(_, n)| *n > v)
    } else {
        false
    }
}

fn parse(input: &str) -> Grid<i32> {
    Grid::from_rows(
        input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            }),
    )
    .unwrap()
}

mod tests_day_09 {

    use super::*;

    #[test]
    fn test_first_part_example() {
        assert_eq!(first_part(include_str!("../inputs/09_example")), 15);
    }

    #[test]
    fn test_first_part() {
        assert_eq!(first_part(include_str!("../inputs/09.in")), 528)
    }

    #[test]
    fn test_second_part_example() {
        assert_eq!(second_part(include_str!("../inputs/09_example")), 1134);
    }
    #[test]
    fn test_second_part() {
        assert_eq!(second_part(include_str!("../inputs/09.in")), 920448);
    }
}
