use std::fmt::Debug;
use tqdm::Iter;

use crate::utils::{show, get_w_h_dimensions, Vec2};


#[derive(Clone, Copy)]
enum Cell {
    Empty,
    Roller,
    Static
}
impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Roller => write!(f, "O"),
            Self::Static => write!(f, "#"),
        }
    }
}

type World = Vec<Vec<Cell>>;

fn parse(input: &str) -> World {
    input.split("\n").filter(|l| ! l.is_empty()).map(
        |line| line.chars().map(
            |c| match c {
                '.' => Cell::Empty,
                'O' => Cell::Roller,
                '#' => Cell::Static,
                _ => panic!("Unknown cell {c:?}")
            }
        ).collect::<Vec<Cell>>()
    ).collect::<World>()
}

fn evaluate(world: &World) -> i32 {
    let (w, h) = get_w_h_dimensions(world);
    world.iter().enumerate().map(
        |(y, row)| row.iter().map(
            |cell| match cell {
                Cell::Roller => (h - y) as i32,
                _ => 0,
            }
        ).sum::<i32>()
    ).sum()
}

fn simulate(world: &mut World, origin: Vec2<i32>, side_vector: Vec2<i32>, roll_vector: Vec2<i32>) {
    let (w, h) = get_w_h_dimensions(world);
    assert!(w == h);

    let mut wall_cursors = Vec::new();
    let mut wall_cursor = origin.clone();
    for i in 0..w {
        wall_cursors.push(wall_cursor.clone());
        wall_cursor = wall_cursor + side_vector
    }

    let mut roll_cursor = origin.clone();

    for roll_index in 0..w {
        let mut precise_cursor = roll_cursor.clone();
        for side_index in 0..w {
            // println!("prec: {precise_cursor:?}");
            
            let cell = world[precise_cursor.y as usize][precise_cursor.x as usize];
            match cell {
                Cell::Static => {
                    wall_cursors[side_index] = precise_cursor + roll_vector
                },
                Cell::Roller => {
                    let fall_to = wall_cursors[side_index];
                    world[precise_cursor.y as usize][precise_cursor.x as usize] = Cell::Empty;
                    world[fall_to.y as usize][fall_to.x as usize] = Cell::Roller;
                    wall_cursors[side_index] = fall_to + roll_vector;
                },
                _ => ()
            }
            precise_cursor = precise_cursor + side_vector;
        }
        roll_cursor = roll_cursor + roll_vector;
        
    }

}

pub fn first_part(input: &str) -> i32 {
    let mut world: World = parse(input);
    
    simulate(&mut world, Vec2 { x: 0, y: 0 }, Vec2 { x: 1, y: 0 }, Vec2 { x: 0, y: 1 });
    
    evaluate(&world)
}

pub fn second_part(input: &str) -> i32 {
    let mut world: World = parse(input);
    let (w, h) = get_w_h_dimensions(&world);
    
    let total_iterations = 1_000_000_000;

    let n: i32 = 1_500;
    let ds = (0..n).tqdm().map(|i| {
        // println!("----");
        simulate(&mut world, Vec2 { x: 0, y: 0 }, Vec2 { x: 1, y: 0 }, Vec2 { x: 0, y: 1 });
        simulate(&mut world, Vec2 { x: 0, y: h as i32 - 1 }, Vec2 { x: 0, y: -1 }, Vec2 { x: 1, y: 0 });
        simulate(&mut world, Vec2 { x: w as i32 - 1, y: h as i32 - 1 }, Vec2 { x: -1, y: 0 }, Vec2 { x: 0, y: -1 });
        simulate(&mut world, Vec2 { x: w as i32 - 1, y: 0 }, Vec2 { x: 0, y: 1 }, Vec2 { x: -1, y: 0 });
        // show(&world);
        evaluate(&world)
    }).collect::<Vec<i32>>();

    let offset = 1000;
    let clean_ds: Vec<i32> = ds[offset as usize..n as usize].to_vec();
    let max_pattern_len = 100;

    for i in 0..max_pattern_len {
        let pattern = clean_ds[0..1+i].to_vec();
        
        let repeated_pattern = vec![pattern.clone(); 5].iter().flatten().cloned().collect::<Vec<_>>();
        
        let differs = clean_ds.iter().zip(repeated_pattern.iter()).any(|(a, b)| a != b);
        if ! differs {
            println!("{repeated_pattern:?} does not differ");
            
            let n_repetitions = (total_iterations - offset) / pattern.len();
            println!("reps: {n_repetitions:?}, p_len: {}", pattern.len());
            println!("final_index {}", (n_repetitions * pattern.len()));
            let correct_offset = (total_iterations - offset) - (n_repetitions * pattern.len());
            println!("correct_offset {}", correct_offset);
            return pattern[(pattern.len() - 1) - correct_offset]
        } else {
            println!("{repeated_pattern:?} does differ");
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use crate::day_14::{first_part, second_part};
    
    #[test]
    fn test_example() {
        assert_eq!(first_part(include_str!("inputs/14_example_1.txt")), 136);
        assert_eq!(second_part(include_str!("inputs/14_example_1.txt")), 64);
    }
    
    #[test]
    fn test_parts() {
        assert_eq!(first_part(include_str!("inputs/14.secret")), 113078);
        assert_eq!(second_part(include_str!("inputs/14.secret")), 94255);
    }
}