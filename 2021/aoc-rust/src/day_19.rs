use itertools::Itertools;
use rayon::prelude::*;
use rustc_hash::FxHashSet;
use std::collections::VecDeque;

use crate::space::{Mat3, Vec3};

pub fn first_part(input: &str) -> i32 {
    let scanners = parse(input);

    let (base, _) = build_ocean(&scanners);
    base.len() as i32
}

pub fn second_part(input: &str) -> i32 {
    let scanners = parse(input);

    let (_, locations) = build_ocean(&scanners);

    locations
        .iter()
        .cartesian_product(locations.iter())
        .map(|(&a, &b)| manhattan_distance(a, b))
        .max()
        .unwrap()
}

fn build_ocean(scanners: &Vec<Vec<Vec3>>) -> (FxHashSet<Vec3>, Vec<Vec3>) {
    let mut base: FxHashSet<Vec3> = scanners[0].iter().cloned().collect();

    let mut remaining_scanners: VecDeque<(usize, Vec<Vec3>)> = VecDeque::new();
    for i in 1..scanners.len() {
        remaining_scanners.push_back((i, scanners[i].clone()));
    }

    let all_rotations: Vec<Mat3> = generate_rotation_matrices().collect();

    let mut found_scanners = vec![];

    while let Some((scanner_id, scanner_data)) = remaining_scanners.pop_front() {
        let (n, rotation, offset) =
            find_best_rotation_and_offset(&scanner_data, &base, &all_rotations);

        if n < 12 {
            remaining_scanners.push_back((scanner_id, scanner_data));
        } else {
            for beacon in scanner_data.iter() {
                base.insert((rotation * (*beacon)) + offset);
            }
            let location = rotation * Vec3::from(vec![0, 0, 0]) + offset;
            found_scanners.push(location);
        }
    }

    (base, found_scanners)
}

fn manhattan_distance(a: Vec3, b: Vec3) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs()
}

fn find_best_rotation_and_offset<'a>(
    beacons: &[Vec3],
    base: &FxHashSet<Vec3>,
    possible_rotations: &'a [Mat3],
) -> (usize, &'a Mat3, Vec3) {
    possible_rotations
        .par_iter()
        .map(|rot| {
            let rotated_beacons = beacons.iter().map(|&b| rot * b).collect::<Vec<_>>();

            let (n, offset) = find_best_offset(&rotated_beacons, base);

            (n, rot, offset)
        })
        .max_by_key(|x| x.0)
        .unwrap()
}

fn find_best_offset(beacons: &[Vec3], base: &FxHashSet<Vec3>) -> (usize, Vec3) {
    beacons
        .iter()
        .cartesian_product(base.iter())
        .map(|(&b, &base_b)| {
            let offset = base_b - b;

            let n_matches = match_beacons(
                &beacons.iter().map(|&_b| _b + offset).collect::<Vec<_>>(),
                base,
            );

            (n_matches, offset)
        })
        .max_by_key(|x| x.0)
        .unwrap()
}

fn match_beacons(beacons: &[Vec3], base: &FxHashSet<Vec3>) -> usize {
    beacons.iter().filter(|b| base.contains(b)).count()
}

fn generate_rotation_matrices() -> impl Iterator<Item = Mat3> {
    [0, 90, 180, 270]
        .into_iter()
        .map(|rot| {
            [
                Mat3::rotation_x(rot),
                &Mat3::rotation_y(rot) * &Mat3::rotation_z(90),
                &Mat3::rotation_z(rot) * &Mat3::rotation_y(270),
                
                &Mat3::rotation_x(rot) * &Mat3::rotation_y(180),
                &Mat3::rotation_y(rot) * &Mat3::rotation_z(270),
                &Mat3::rotation_z(rot) * &Mat3::rotation_y(90),

            ]
            .into_iter()
        })
        .flatten()
}

fn parse(input: &str) -> Vec<Vec<Vec3>> {
    input
        .split("--- scanner")
        .map(|part| parse_scanner(part))
        .filter(|scanner| scanner.len() > 0)
        .collect()
}

fn parse_scanner(input: &str) -> Vec<Vec3> {
    input
        .lines()
        .filter(|line| line.contains(","))
        .map(|line| {
            line.split(",")
                .map(|part| part.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
                .into()
        })
        .collect()
}

#[cfg(test)]
mod tests_day_19 {

    use super::*;

    #[test]
    fn test_parse() {
        let scanners = parse(include_str!("../inputs/19_example"));

        assert_eq!(scanners.len(), 5);
        assert_eq!(scanners[0][0], vec![404, -588, -901].into());
        assert_eq!(scanners[4][25], vec![30, -46, -14].into());
    }

    #[test]
    fn test_match_beacons() {
        let base: FxHashSet<Vec3> = vec![
            vec![1, 0, 0].into(),
            vec![3, 0, 0].into(),
            vec![10, 0, 10].into(),
        ]
        .iter()
        .cloned()
        .collect();

        let beacons = vec![
            vec![0, 0, 0].into(),
            vec![0, -2, 0].into(),
            vec![0, -9, 10].into(),
        ];

        let rots: Vec<Mat3> = generate_rotation_matrices().collect();

        assert_eq!(
            find_best_rotation_and_offset(&beacons, &base, &rots),
            (3, &Mat3::rotation_z(90), vec![1, 0, 0].into())
        );
    }

    #[test]
    fn test_rotations() {
        assert_eq!(
            generate_rotation_matrices().collect::<FxHashSet<_>>().len(),
            24
        )
    }

    #[test]
    fn test_example_first_part() {
        assert_eq!(first_part(include_str!("../inputs/19_example")), 79);
    }
    #[test]
    fn test_example_second_part() {
        assert_eq!(second_part(include_str!("../inputs/19_example")), 3621);
    }

    #[test]
    fn test_first_part() {
        assert_eq!(first_part(include_str!("../inputs/19.in")), 491);
    }
    #[test]
    fn test_second_part() {
        assert_eq!(second_part(include_str!("../inputs/19.in")), 13374);
    }
}
