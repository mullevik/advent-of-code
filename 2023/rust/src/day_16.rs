use std::collections::{HashSet, HashMap};
use tqdm::Iter;


type V2 = (i32, i32);

fn show(mp: &Vec<Vec<bool>>) {
    for (y, row) in mp.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            match cell {
                true => print!("#"),
                false => print!(".")
            }
        }
        println!();
    }
}

fn simulate_beams(world: &Vec<Vec<char>>, start_beam: (V2, V2)) -> i64 {

    let (w, h) = (world.iter().next().unwrap().len() as i32, world.len() as i32);

    let mut beams: Vec<(V2, V2)> = vec![];
    beams.push(start_beam.clone());

    let mut energized_map: Vec<Vec<bool>> = vec![vec![false; w as usize]; h as usize];
    let mut locked_splitters: HashSet<V2> = HashSet::new();

    let mut n_changeless = 0;
    loop {
        if n_changeless > 10 {
            break;
        }
        // let some_count = energized_map.iter().flatten().filter(|x| **x).count();
        // println!("{some_count:?}");
        // println!("{n_iterations}");
        // show(&energized_map);

        let mut change = false;

        for b in beams.iter() {
            let (b_pos, b_dir) = b;

            let e = energized_map[b_pos.1 as usize][b_pos.0 as usize];
            if ! e {
                change = true;
            }
            energized_map[b_pos.1 as usize][b_pos.0 as usize] = true;
        }
        if change {
            n_changeless = 0;
        } else {
            n_changeless += 1;
        }
        

        let mut tmp_beams: Vec<(V2, V2)> = vec![];
        for b in beams.iter() {
            let (b_pos, b_dir) = b;

            let cell = world[b_pos.1 as usize][b_pos.0 as usize];

            let mut new_beams: Vec<(V2, V2)> = match (cell, b_dir) {
                ('.', _) => vec![b.clone()],
                ('-', (_, 0)) => vec![b.clone()],
                ('-', (0, _)) => {
                    if locked_splitters.contains(b_pos) {
                        vec![]
                    } else {
                        locked_splitters.insert(b_pos.clone());
                        vec![(b_pos.clone(), (1, 0)), (b_pos.clone(), (-1, 0))]
                    }
                },
                ('|', (0, _)) => vec![b.clone()],
                ('|', (_, 0)) => {
                    if locked_splitters.contains(b_pos) {
                        vec![]
                    } else {
                        locked_splitters.insert(b_pos.clone());
                        vec![(b_pos.clone(), (0, 1)), (b_pos.clone(), (0, -1))]
                    }
                },
                ('/', (dx, dy)) => vec![(b_pos.clone(), (-dy.clone(), -dx.clone()))],
                ('\\', (dx, dy)) => vec![(b_pos.clone(), (dy.clone(), dx.clone()))],
                _ => panic!("Unexpected state ({cell:?}, {b_dir:?})")
            };
            tmp_beams.append(&mut new_beams);
        }

        let moved_beams: Vec<(V2, V2)> = tmp_beams
        .into_iter()
        .map(
            |(p, d)| ((p.0 + d.0, p.1 + d.1), d)
        )
        .filter(|(p, d)| 0 <= p.0 && p.0 < w && 0 <= p.1 && p.1 < h)
        .collect();
        
        beams = moved_beams;
    }

    energized_map.iter().flatten().filter(|x| **x).count() as i64
}

fn parse_world(text: &str) -> Vec<Vec<char>> {
    text.split("\n").filter(|l| !l.is_empty()).map(
        |line| line.chars().collect()
    ).collect()
}

pub fn first_part(input: &str) -> i64 {
    let world = parse_world(input);
    simulate_beams(&world, ((0, 0), (1, 0)))
}

pub fn second_part(input: &str) -> i64 {
    let world = parse_world(input);

    let (w, h) = (world.iter().next().unwrap().len() as i32, world.len() as i32);

    let mut beams: Vec<(V2, V2)> = vec![];

    for x in 0..w {
        beams.push(((x, 0), (0, 1)));
        beams.push(((x, h - 1), (0, -1)));
    }
    for y in 0..h {
        beams.push(((0, y), (1, 0)));
        beams.push(((w - 1, y), (-1, 0)));
    }

    beams.iter().tqdm().map(
        |b| simulate_beams(&world, b.clone())
    ).max().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day_16::{first_part, second_part};
    
    #[test]
    fn test_example() {
        assert_eq!(first_part(include_str!("inputs/16_example_1.txt")), 46);
        assert_eq!(second_part(include_str!("inputs/16_example_1.txt")), 51);
    }
    
    #[test]
    fn test_parts() {
        assert_eq!(first_part(include_str!("inputs/16.secret")), 7996);
        // assert_eq!(second_part(include_str!("inputs/16.secret")), 0);
    }
}