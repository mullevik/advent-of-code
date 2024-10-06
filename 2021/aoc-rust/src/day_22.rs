use std::{
    cmp::{max, min},
    ops::Range,
};

use itertools::Itertools;

const WORLD_OFFSET: i32 = 50;

pub fn first_part(input: &str) -> i64 {
    build_world(&parse(input))
        .into_iter()
        .map(|c| clamp(&c))
        .map(|c| volume(&c))
        .sum::<i64>()
}
pub fn second_part(input: &str) -> i64 {
    build_world(&parse(input))
        .into_iter()
        .map(|c| volume(&c))
        .sum::<i64>()
}

type Cuboid = (Range<i32>, Range<i32>, Range<i32>);

struct Operation {
    value: bool,
    cuboid: Cuboid,
}

fn offset_to_usize(r: &Range<i32>, offset: i32) -> Range<usize> {
    (r.start + offset) as usize..(r.end + offset + 1) as usize
}

fn volume(c: &Cuboid) -> i64 {
    let x =
        (c.0.end - c.0.start) as i64 * (c.1.end - c.1.start) as i64 * (c.2.end - c.2.start) as i64;
    if x < 0 {
        println!("negative cuboid: {:?}", c);
    }
    x
}

fn clamp_range(r: &Range<i32>) -> Range<i32> {
    min(max(r.start, -WORLD_OFFSET), WORLD_OFFSET + 1)
        ..min(max(r.end, -WORLD_OFFSET), WORLD_OFFSET + 1)
}

fn clamp(c: &Cuboid) -> Cuboid {
    (clamp_range(&c.0), clamp_range(&c.1), clamp_range(&c.2))
}

enum Intersection {
    Double(Range<i32>),
    Single(i32),
}

fn is_range_overlap(a: &Range<i32>, b: &Range<i32>) -> bool {
    (b.start >= a.start && b.start < a.end)
        || (b.end > a.start && b.end <= a.end)
        || (a.start >= b.start && a.start < b.end)
        || (a.end > b.start && a.end <= b.end)
}

fn is_cuboid_overlap(a: &Cuboid, b: &Cuboid) -> bool {
    is_range_overlap(&a.0, &b.0) && is_range_overlap(&a.1, &b.1) && is_range_overlap(&a.2, &b.2)
}

fn cut_range(base: &Range<i32>, to_cut: &Range<i32>) -> (Range<i32>, Range<i32>) {
    (
        min(to_cut.start, base.start)..base.start,
        base.end..max(to_cut.end, base.end),
    )
}

// fn build_world(ops: &[Operation]) -> i32 {
//     let mut world = vec![];

//     for op in ops {
//         let mut stack = vec![];

//         let overlapping = world
//             .iter()
//             .filter(|b| is_cuboid_overlap(b, &op.cuboid))
//             .collect::<Vec<_>>();

//         for overlapping_cuboid in overlapping {}
//     }

//     todo!();
// }

fn modify_world_by_adding(world: &mut Vec<Cuboid>, new: &Cuboid) {
    let mut stack = vec![new.clone()];
    // let mut accumulator = vec![];

    while let Some(current) = stack.pop() {
        let world_overlap = world.iter().find(|b| is_cuboid_overlap(b, &current));

        if let Some(overlap) = world_overlap {
            cut_into_non_empty_cuboids(overlap, &current).for_each(|c| stack.push(c));
        } else {
            world.push(current);
        }
    }
}

fn cut_into_non_empty_cuboids(base: &Cuboid, to_cut: &Cuboid) -> impl Iterator<Item = Cuboid> {
    let new_x = cut_range(&base.0, &to_cut.0);
    let new_y = cut_range(&base.1, &to_cut.1);
    let new_z = cut_range(&base.2, &to_cut.2);

    [
        (new_x.0, to_cut.1.clone(), to_cut.2.clone()),
        (new_x.1, to_cut.1.clone(), to_cut.2.clone()),
        (to_cut.0.clone(), new_y.0, to_cut.2.clone()),
        (to_cut.0.clone(), new_y.1, to_cut.2.clone()),
        (to_cut.0.clone(), to_cut.1.clone(), new_z.0),
        (to_cut.0.clone(), to_cut.1.clone(), new_z.1),
    ]
    .into_iter()
    .filter(|c| volume(c) > 0)
}

fn subtract_from_world(world: Vec<Cuboid>, new: &Cuboid) -> Vec<Cuboid> {
    let mut new_world = Vec::with_capacity(world.len());

    for existing in world.iter() {
        if is_cuboid_overlap(new, existing) {
            cut_into_non_empty_cuboids(new, existing)
                .for_each(|c| modify_world_by_adding(&mut new_world, &c));
        } else {
            new_world.push(existing.clone());
        }
    }

    new_world
}

fn build_world(ops: &[Operation]) -> Vec<Cuboid> {
    let mut world = vec![];

    for op in ops {
        if op.value {
            modify_world_by_adding(&mut world, &op.cuboid);
        } else {
            world = subtract_from_world(world, &op.cuboid);
        }
    }

    world
}

fn parse(input: &str) -> Vec<Operation> {
    input
        .lines()
        .map(|line| {
            let (_x, _y, _z) = line.split(',').collect_tuple().unwrap();
            let is_on = _x.starts_with("on");
            let (_x_from, _x_to) = _x.split_once("..").unwrap();
            let (_y_from, _y_to) = _y.split_once("..").unwrap();
            let (_z_from, _z_to) = _z.split_once("..").unwrap();

            Operation {
                value: is_on,
                cuboid: (
                    _x_from.split_once('=').unwrap().1.parse::<i32>().unwrap()
                        .._x_to.parse::<i32>().unwrap() + 1,
                    _y_from.split_once('=').unwrap().1.parse::<i32>().unwrap()
                        .._y_to.parse::<i32>().unwrap() + 1,
                    _z_from.split_once('=').unwrap().1.parse::<i32>().unwrap()
                        .._z_to.parse::<i32>().unwrap() + 1,
                ),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests_day_22 {
    use super::*;

    const SMALL_EXAMPLE_INPUT: &str = "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";

    const EXAMPLE_INPUT: &str = "on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682";

    #[test]
    fn test_parse() {
        let ops = parse(EXAMPLE_INPUT);
        assert_eq!(ops.len(), 22);
        assert_eq!(ops[0].cuboid, (-20..27, -36..18, -47..8));
        assert!(ops[0].value);
        assert!(!ops[10].value);
        assert_eq!(
            ops[ops.len() - 1].cuboid,
            (967..23433, 45373..81176, 27513..53683)
        );
    }

    #[test]
    fn test_overlaps() {
        let a = (0..3, 0..3, 0..3);
        let b = (1..5, 0..3, 0..3);
        let c = (-1..2, -300..-200, -300..-200);

        assert_eq!(volume(&a), 27);
        assert_eq!(volume(&b), 36);
        assert_eq!(volume(&c), 30000);
        assert!(is_cuboid_overlap(&a, &b));
        assert!(!is_cuboid_overlap(&a, &c))
    }

    #[test]
    fn test_cut_by_world() {
        let mut world = vec![(0..3, 0..3, 0..3)];

        modify_world_by_adding(&mut world, &(2..5, 2..5, 2..5));

        assert_eq!(
            world,
            vec![
                (0..3, 0..3, 0..3),
                (2..5, 2..5, 3..5),
                (2..5, 3..5, 2..3),
                (3..5, 2..3, 2..3)
            ]
        );
        assert_eq!(world.iter().map(volume).sum::<i64>(), 53);
    }

    #[test]
    fn test_subtract_from_world() {
        let world = vec![(0..3, 0..3, 0..3)];

        let new_world = subtract_from_world(world, &(2..5, 2..5, 2..5));

        assert_eq!(
            new_world,
            vec![(0..2, 0..3, 0..3), (2..3, 0..2, 0..3), (2..3, 2..3, 0..2)]
        );
        assert_eq!(new_world.iter().map(volume).sum::<i64>(), 26);
    }

    #[test]
    fn test_example_first_part() {
        assert_eq!(first_part(SMALL_EXAMPLE_INPUT), 39);
        assert_eq!(first_part(EXAMPLE_INPUT), 590784);
    }
    #[test]
    fn test_first_part() {
        assert_eq!(first_part(include_str!("../inputs/22.in")), 647076);
    }
    #[test]
    fn test_second_part() {
        assert_eq!(second_part(include_str!("../inputs/22.in")), -1);
    }
}
