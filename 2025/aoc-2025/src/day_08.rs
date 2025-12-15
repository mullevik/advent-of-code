use std::{cmp::min, collections::HashSet};

pub fn p1(input: &str, max_connections: usize) -> i64 {
    let points = parse(input);

    let mut edges = (0..points.len())
        .flat_map(|i| (0..points.len()).map(move |j| (i, j)))
        .map(|(i, j)| (i, j, dist(&points[i], &points[j])))
        .filter(|e| e.0 < e.1)
        .collect::<Vec<_>>();

    edges.sort_by_key(|e| e.2);

    let mut circits = (0..points.len()).collect::<Vec<_>>();

    for (i, e) in edges.iter().enumerate() {
        if i == max_connections {
            break;
        }

        let a_circuit = circits[e.0];
        let b_circuit = circits[e.1];
        if a_circuit != b_circuit {
            let new_circuit = min(a_circuit, b_circuit);

            circits.iter_mut().for_each(|c| {
                if *c == a_circuit || *c == b_circuit {
                    *c = new_circuit
                }
            });
        }
    }

    let unique_circuits = circits.iter().collect::<HashSet<_>>();
    let mut circuit_counts = unique_circuits
        .iter()
        .map(|uc| circits.iter().filter(|c| c == uc).count() as i64)
        .collect::<Vec<_>>();
    circuit_counts.sort();
    circuit_counts.reverse();
    circuit_counts[0] * circuit_counts[1] * circuit_counts[2]
}

pub fn p2(input: &str) -> i64 {
    let points = parse(input);

    let mut edges = (0..points.len())
        .flat_map(|i| (0..points.len()).map(move |j| (i, j)))
        .map(|(i, j)| (i, j, dist(&points[i], &points[j])))
        .filter(|e| e.0 < e.1)
        .collect::<Vec<_>>();

    edges.sort_by_key(|e| e.2);

    let mut circits = (0..points.len()).collect::<Vec<_>>();

    for e in edges.iter() {
        let a_circuit = circits[e.0];
        let b_circuit = circits[e.1];

        if a_circuit != b_circuit {
            let new_circuit = min(a_circuit, b_circuit);

            circits.iter_mut().for_each(|c| {
                if *c == a_circuit || *c == b_circuit {
                    *c = new_circuit
                }
            });

            if circits.iter().all(|c| *c == circits[0]) {
                return points[e.0][0] * points[e.1][0];
            }
        }
    }
    -1
}

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .split("\n")
        .filter(|x| !x.trim().is_empty())
        .map(|line| {
            line.split(",")
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn dist(a: &[i64], b: &[i64]) -> i64 {
    ((b[0] - a[0]) * (b[0] - a[0]))
        + ((b[1] - a[1]) * (b[1] - a[1]))
        + ((b[2] - a[2]) * (b[2] - a[2]))
}

mod test {
    use std::fs;

    use crate::day_08::{p1, p2};

    #[test]
    fn test_p1() {
        assert_eq!(
            p1(&fs::read_to_string("inputs/08.example").unwrap(), 10),
            40
        )
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&fs::read_to_string("inputs/08.example").unwrap()), 25272)
    }
}
