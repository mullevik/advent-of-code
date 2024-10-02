use std::ops::Range;

use itertools::Itertools;

const WORLD_SIZE: usize = 101;
const WORLD_OFFSET: i32 = 50;

pub fn first_part(input: &str) -> i64 {
    let ops = parse(input);
    let mut world = vec![vec![vec![false; WORLD_SIZE]; WORLD_SIZE]; WORLD_SIZE];

    for (i, op) in ops.iter().enumerate() {
        for x in offset_to_usize(&op.ranges.0, WORLD_OFFSET) {
            for y in offset_to_usize(&op.ranges.1, WORLD_OFFSET) {
                for z in offset_to_usize(&op.ranges.2, WORLD_OFFSET) {
                    if x < WORLD_SIZE && y < WORLD_SIZE && z < WORLD_SIZE {
                        world[x][y][z] = op.value;
                    }
                }
            }
        }
    }
    world
        .iter()
        .flat_map(|v| v.iter().flat_map(|v| v.iter()))
        .filter(|x| **x)
        .count() as i64
}
pub fn second_part(input: &str) -> i32 {
    todo!()
}

struct Operation {
    value: bool,
    ranges: (Range<i32>, Range<i32>, Range<i32>),
}

fn offset_to_usize(r: &Range<i32>, offset: i32) -> Range<usize> {
    (r.start + offset) as usize..(r.end + offset + 1) as usize
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
                ranges: (
                    _x_from.split_once('=').unwrap().1.parse::<i32>().unwrap()
                        .._x_to.parse::<i32>().unwrap(),
                    _y_from.split_once('=').unwrap().1.parse::<i32>().unwrap()
                        .._y_to.parse::<i32>().unwrap(),
                    _z_from.split_once('=').unwrap().1.parse::<i32>().unwrap()
                        .._z_to.parse::<i32>().unwrap(),
                ),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests_day_22 {
    use super::*;

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
        assert_eq!(ops[0].ranges, (-20..26, -36..17, -47..7));
        assert!(ops[0].value);
        assert!(!ops[10].value);
        assert_eq!(
            ops[ops.len() - 1].ranges,
            (967..23432, 45373..81175, 27513..53682)
        );
    }

    #[test]
    fn test_example_first_part() {
        assert_eq!(first_part(EXAMPLE_INPUT), 590784);
    }
    #[test]
    fn test_first_part() {
        assert_eq!(first_part(include_str!("../inputs/22.in")), 647076);

        // fixme: terribly sloooow!
    }
    // #[test]
    // fn test_second_part() {
    //     assert_eq!(second_part(include_str!("../inputs/22.in")), -1);
    // }
}
