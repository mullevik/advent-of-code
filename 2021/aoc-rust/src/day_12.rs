use std::collections::{HashMap, VecDeque};

use crate::graph::{Graph, SparseGraph};

struct PathState {
    vertex: String,
    visited_counts: HashMap<String, i32>,
}

pub fn first_part(input: &str) -> i32 {
    count_paths(&parse(input), 0)
}

pub fn second_part(input: &str) -> i32 {
    count_paths(&parse(input), 1)
}

fn count_paths(graph: &SparseGraph<String>, max_double_visits: i32) -> i32 {
    let mut dfs_stack: VecDeque<PathState> = VecDeque::new();

    dfs_stack.push_front(PathState {
        vertex: "start".to_string(),
        visited_counts: vec![("start".to_string(), 1)].into_iter().collect(),
    });

    let mut end_counter = 0;

    while !dfs_stack.is_empty() {
        let current = dfs_stack.pop_front().unwrap();

        for adjacent in graph.iter_adjacents(&current.vertex).unwrap() {
            if *adjacent == "end".to_string() {
                end_counter += 1;
            } else if is_reachable(&current, adjacent, max_double_visits) {
                let mut adjacent_visited_counts = current.visited_counts.clone();
                let adjcent_count_ref =
                    adjacent_visited_counts.entry(adjacent.clone()).or_insert(0);
                *adjcent_count_ref += 1;
                dfs_stack.push_front(PathState {
                    vertex: adjacent.clone(),
                    visited_counts: adjacent_visited_counts,
                });
            }
        }
    }

    end_counter
}

fn is_small_cave(vertex_key: &String) -> bool {
    vertex_key.chars().any(|c| c.is_lowercase())
}

fn is_reachable(current: &PathState, adjacent: &String, max_double_visits: i32) -> bool {
    if adjacent == "start" {
        return false;
    }

    if !is_small_cave(adjacent) {
        return true;
    }

    let double_visit_count = current
        .visited_counts
        .iter()
        .filter(|(k, &v)| is_small_cave(k) && v > 1)
        .count() as i32;

    if double_visit_count < max_double_visits {
        return true;
    }

    let adjacent_visited_count = match current.visited_counts.get(adjacent) {
        Some(val) => *val,
        None => 0,
    };

    adjacent_visited_count < 1
}

fn parse(input: &str) -> SparseGraph<String> {
    input
        .lines()
        .map(|row| row.split_once("-").unwrap())
        .collect()
}

#[cfg(test)]
mod tests_day_12 {
    use std::collections::HashSet;

    use super::{first_part, parse, second_part};
    use crate::graph::{Graph, Sizable, SparseGraph};

    #[test]
    fn test_parsing() {
        let g = parse(include_str!("../inputs/12_example"));

        assert_eq!(g.len(), 6);
        assert_eq!(
            g.iter_vertices()
                .map(|x| x.as_str())
                .collect::<HashSet<_>>(),
            HashSet::from_iter(vec!["start", "A", "b", "c", "d", "end"].iter().cloned())
        );
    }

    #[test]
    fn test_example_first_part() {
        assert_eq!(first_part(include_str!("../inputs/12_example")), 10);
        assert_eq!(
            first_part(include_str!("../inputs/12_even_larger_example")),
            226
        );
    }

    #[test]
    fn test_first_part() {
        assert_eq!(first_part(include_str!("../inputs/12.in")), 5958);
    }

    #[test]
    fn test_example_second_part() {
        assert_eq!(second_part(include_str!("../inputs/12_example")), 36);
        assert_eq!(
            second_part(include_str!("../inputs/12_even_larger_example")),
            3509
        );
    }

    #[test]
    fn test_second_part() {
        assert_eq!(second_part(include_str!("../inputs/12.in")), 150426);
    }
}
