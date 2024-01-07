use std::{i32, collections::{HashSet, VecDeque, vec_deque, HashMap}};

use itertools::Itertools;


#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
struct Operation {
    direction: Direction,
    amount: i32,
    html_color: String,
}


fn parse(text: &str) -> Vec<Operation> {
    text
    .split("\n")
    .filter(|t| !t.is_empty())
    .map(|line| {
        let parts: Vec<&str> = line.split(" ").take(3).collect();

        let direction = match parts[0].chars().next().unwrap() {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Unknown direction in {parts:?}")
        };

        let amount = parts[1].parse::<i32>().unwrap();

        let color = parts[2].replace("(", "").replace(")", "");

        Operation{direction: direction, amount: amount, html_color: color}
    }).collect()
}

fn parse_from_color(text: &str) -> Vec<Operation> {
    text
    .split("\n")
    .filter(|t| !t.is_empty())
    .map(|line| {
        let parts: Vec<&str> = line.split(" ").take(3).collect();

        let color = parts[2].replace("(", "").replace(")", "");

        let amount = i32::from_str_radix(&color[1..6], 16).unwrap();

        let direction = match &color[color.len() - 1..color.len()] {
            "0" => Direction::Right,
            "1" => Direction::Down,
            "2" => Direction::Left,
            "3" => Direction::Up,
            _ => panic!("Unknown color {color:?}")
        };
        Operation{direction: direction, amount: amount, html_color: "?".to_string()}
    }).collect()
}

fn build_world(operations: &Vec<Operation>) -> Vec<Vec<bool>> {
    let mut walls = HashSet::new();
    let mut cursor = (0, 0);

    walls.insert(cursor.clone());
    for op in operations.iter() {
        let vector: (i32, i32) = match op.direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        for i in 0..op.amount {
            cursor = (cursor.0 + vector.0, cursor.1 + vector.1);
            walls.insert(cursor);
        }
    }

    let min_x = walls.iter().map(|x| x.0).min().unwrap();
    let min_y = walls.iter().map(|x| x.1).min().unwrap();
    let max_x = walls.iter().map(|x| x.0).max().unwrap();
    let max_y = walls.iter().map(|x| x.1).max().unwrap();

    let offset_x = 0 - min_x;
    let offset_y = 0 - min_y;

    let w = max_x - min_x + 1;
    let h = max_y - min_y + 1;
    dbg!((w, h));
    let mut world = vec![vec![false; w as usize]; h as usize];

    for wall in walls.iter() {
        world[(wall.1 + offset_y) as usize][(wall.0 + offset_x) as usize] = true;
    }
    world
}

fn print_world<T: std::fmt::Debug>(world: &Vec<Vec<T>>) {
    for (y, row) in world.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            print!(" {cell:?} ")
        }
        println!();
    }
}

fn in_world(p: &(i32, i32), world: &Vec<Vec<bool>>) -> bool {
    0 <= p.1 && p.1 < world.len() as i32 && 0 <= p.0 && p.0 < world.iter().next().unwrap().len() as i32
}

fn get_four_adjacent(p: (i32, i32), world: &Vec<Vec<bool>>) -> Vec<(i32, i32)> {
    vec![
        (p.0 + 1, p.1 + 0),
        (p.0 - 1, p.1 + 0),
        (p.0 + 0, p.1 + 1),
        (p.0 + 0, p.1 - 1),
    ].iter().filter(|x| in_world(x, world)).cloned().collect()
}

fn find_connected_components(world: &Vec<Vec<bool>>) -> Vec<Vec<i32>> {

    let mut dfs_stack: VecDeque<(i32, i32)> = VecDeque::new();
    let mut components = vec![vec![-1; world.iter().next().unwrap().len()]; world.len()];
    let mut component_id = 0;
    for (y, row) in world.iter().enumerate() {
        for (x, is_wall) in row.iter().enumerate() {
            if ! is_wall {
                dfs_stack.push_front((x as i32, y as i32));
                components[y][x] = component_id;
                component_id += 1;
            }
        }
    }

    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    while ! dfs_stack.is_empty() {
        let current = dfs_stack.pop_front().unwrap();
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);
        let current_component = components[current.1 as usize][current.0 as usize];
        
        for adjacent in get_four_adjacent(current, world).iter() {
            let is_wall = world[adjacent.1 as usize][adjacent.0 as usize];
            if is_wall {
                continue;
            }
            components[adjacent.1 as usize][adjacent.0 as usize] = current_component;
            dfs_stack.push_front(adjacent.clone());
        }
    }
    components
}

fn extract_vertices(operations: &Vec<Operation>) -> Vec<(i32, i32)> {
    let mut vertices: Vec<(i32, i32)> = Vec::new();
    let mut cursor = (0, 0);
    for op in operations.iter() {
        let d: (i32, i32) = match op.direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        let vector = (d.0 * (op.amount + 1), d.1 * (op.amount + 1));
        cursor = (cursor.0 + vector.0, cursor.1 + vector.1);
        vertices.push(cursor.clone());
    }
    vertices
}

fn compute_area(vertices: &Vec<(i32, i32)>) -> i64 {
    let mut area: i64 = 0;
    for i in 0..vertices.len() {
        let j = (i + 1) % vertices.len();
        area += (vertices[i].0 as i64) * (vertices[j].1 as i64);
        area -= (vertices[i].1 as i64) * (vertices[j].0 as i64);
    }
    area.abs()
}

fn compute_perimeter(vertices: &Vec<(i32, i32)>) -> i64 {
    let mut s: i64 = 0;
    for i in 0..vertices.len() {
        let j = (i + 1) % vertices.len();
        let d = (vertices[i].0 - vertices[j].0).abs() as i64 + (vertices[i].1 - vertices[j].1).abs() as i64;
        s += d - 1;
    }
    s
}

pub fn first_part(input: &str) -> i64 {
    
    let operations = parse(input);
    let vertices = extract_vertices(&operations);
    dbg!(&vertices);
    let area = compute_area(&vertices);
    let half_area = area / 2;
    let perimeter = compute_perimeter(&vertices);
    dbg!((area, half_area, perimeter));
    half_area + perimeter
}

pub fn second_part(input: &str) -> i64 {
    let operations = parse_from_color(input);
    let vertices = extract_vertices(&operations);
    dbg!(&vertices);
    compute_area(&vertices) / 2
}

#[cfg(test)]
mod tests {
    use crate::day_18::{first_part, second_part, compute_area};
    

    #[test]
    fn test_area() {
        assert_eq!(compute_area(&vec![(0, 0), (2, 0), (2, 2), (0, 2)]), 4 * 2);
        assert_eq!(compute_area(&vec![(0, 0), (-2, 0), (-2, -2), (0, -2)]), 4 * 2);
        assert_eq!(compute_area(&vec![(0, 0), (2, 0), (2, -2), (0, -2)]), 4 * 2);

    }

    #[test]
    fn test_example() {
        assert_eq!(first_part(include_str!("inputs/18_example_1.txt")), 62);
        // assert_eq!(second_part(include_str!("inputs/18_example_1.txt")), 952408144115);
    }
    
    // #[test]
    // fn test_parts() {
    //     assert_eq!(first_part(include_str!("inputs/18.secret")), 0);
    //     assert_eq!(second_part(include_str!("inputs/18.secret")), 0);
    // }
}