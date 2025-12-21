use std::collections::{HashMap, HashSet};

pub fn p1(input: &str) -> i64 {
    let g = parse(input);
    let paths = obtain_paths_to_goal(&g, "you");
    paths.iter().count() as i64
}

pub fn p2(input: &str) -> i64 {
    let g = parse(input);

    let n_dac = dfs("dac", "out", &g, &mut HashMap::new());
    let n_fft = dfs("fft", "dac", &g, &mut HashMap::new());
    let n_svr = dfs("svr", "fft", &g, &mut HashMap::new());

    n_dac * n_fft * n_svr
}

fn dfs<'a>(
    curr: &'a str,
    goal: &'a str,
    g: &HashMap<&'a str, Vec<&'a str>>,
    cache: &mut HashMap<&'a str, i64>,
) -> i64 {
    if cache.contains_key(&curr) {
        *cache.get(curr).unwrap()
    } else {
        if curr == goal {
            1
        } else {
            let empty = vec![];
            let adjacents = g.get(curr).unwrap_or(&empty);
            let count = adjacents.iter().map(|a| dfs(a, goal, g, cache)).sum();

            cache.insert(curr, count);
            count
        }
    }
}

const GOAL: &str = "out";

fn obtain_paths_to_goal<'a>(
    g: &HashMap<&'a str, Vec<&'a str>>,
    start: &'a str,
) -> Vec<HashSet<&'a str>> {
    let mut paths = vec![];
    let mut stack = vec![(start, HashSet::new())];

    while !stack.is_empty() {
        let (curr, curr_path) = stack.pop().unwrap();

        if curr == GOAL {
            paths.push(curr_path);
            continue;
        }

        let adjacents = g.get(curr).unwrap();

        for a in adjacents.iter() {
            if !curr_path.contains(a) {
                let mut new_path = curr_path.clone();
                new_path.insert(curr);
                stack.push((a, new_path));
            }
        }
    }
    paths
}

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (lhs, rhs) = l.split_once(":").unwrap();

            (lhs, rhs.split_whitespace().collect::<Vec<_>>())
        })
        .collect::<HashMap<&str, Vec<&str>>>()
}

mod tests {
    use std::fs;

    use crate::day_11::{p1, p2};

    #[test]
    fn test_p1() {
        assert_eq!(p1(&fs::read_to_string("inputs/11.example").unwrap()), 5);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&fs::read_to_string("inputs/11.example2").unwrap()), 2);
    }
}
