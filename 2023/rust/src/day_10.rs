use std::{ops::Index, collections::{VecDeque, HashSet, HashMap, hash_set}, vec, fmt};

use itertools::Itertools;


#[derive(Clone, Copy)]
enum Tile {
    DOT,
    CF,
    CJ,
    CL,
    C7,
    VERT,
    HOR,
    START,
}
impl Tile {
    fn from(c: char) -> Self {
        match c {
            '|' => Self::VERT,
            '-' => Self::HOR,
            'F' => Self::CF,
            'J' => Self::CJ,
            'L' => Self::CL,
            '7' => Self::C7,
            'S' => Self::START,
            '.' => Self::DOT,
            x => panic!("Unknown input ({x:?})")
        }
    }
}
impl Default for Tile {
    fn default() -> Self { Tile::DOT }
}
impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Tile::DOT => write!(f, "."),
            Tile::CF => write!(f, "F"),
            Tile::CJ => write!(f, "J"),
            Tile::CL => write!(f, "L"),
            Tile::C7 => write!(f, "7"),
            Tile::VERT => write!(f, "|"),
            Tile::HOR => write!(f, "-"),
            Tile::START => write!(f, "S")
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn four_adjacent(&self) -> [Point;4] {
        [
            Point{x: self.x + 1, y: self.y + 0},
            Point{x: self.x - 1, y: self.y + 0},
            Point{x: self.x + 0, y: self.y + 1},
            Point{x: self.x + 0, y: self.y - 1},
        ]
    }
}

fn parse_row(text: &str) -> Vec<Tile> {
    text.chars().map(Tile::from).collect()
}

fn extract_start(rows: &Vec<Vec<Tile>>) -> Point {
    rows.iter()
    .enumerate()
    .map(|(i, row)| {
        match row.iter().position(|p| matches!(p, Tile::START)) {
            Some(p) => Some(Point{x: p as i32, y: i as i32}),
            None => None
        }
    })
    .filter(|p| matches!(p, Some(..)))
    .next()
    .unwrap()
    .unwrap()
}

fn is_contained(point: &Point, rows: &Vec<Vec<Tile>>) -> bool {
    let w = rows.first().unwrap().len() as i32;
    let h = rows.len() as i32;
    point.x >= 0 && point.x < w && point.y >= 0 && point.y < h
}

fn get_connections(point: Point,  rows: &Vec<Vec<Tile>>) -> (bool, bool, bool, bool) {
    let empty: Vec<Tile> = vec![];

    let up: Tile = rows.get((point.y - 1) as usize)
    .unwrap_or(&empty)
    .get(point.x as usize)
    .unwrap_or(&Tile::default())
    .clone();

    let down: Tile = rows.get((point.y + 1) as usize)
    .unwrap_or(&empty)
    .get(point.x as usize)
    .unwrap_or(&Tile::default())
    .clone();

    let left: Tile = rows.get((point.y) as usize)
    .unwrap_or(&empty)
    .get((point.x - 1) as usize)
    .unwrap_or(&Tile::default())
    .clone();

    let right: Tile = rows.get((point.y) as usize)
    .unwrap_or(&empty)
    .get((point.x + 1) as usize)
    .unwrap_or(&Tile::default())
    .clone();

    let up_connects: bool = match up {
        Tile::VERT | Tile::C7 | Tile::CF | Tile::START => true,
        _ => false,
    };
    let down_connects: bool = match down {
        Tile::VERT | Tile::CJ | Tile::CL | Tile::START => true,
        _ => false,
    };
    let right_connects: bool = match right {
        Tile::HOR | Tile::CJ | Tile::C7 | Tile::START => true,
        _ => false,
    };
    let left_connects: bool = match left {
        Tile::HOR | Tile::CF | Tile::CL | Tile::START => true,
        _ => false,
    };

    (up_connects, right_connects, down_connects, left_connects)
}

fn fix_start(start: Point, rows: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let replacer: Tile = match get_connections(start, &rows) {
        (true, false, true, false) => Tile::VERT,
        (false, true, false, true) => Tile::HOR,
        (true, true, false, false) => Tile::CL,
        (false, true, true, false) => Tile::CF,
        (false, false, true, true) => Tile::C7,
        (true, false, false, true) => Tile::CJ,
        x => panic!("Cannot infer start for matching {x:?}")
    };

    let mut new_rows = rows.clone();
    new_rows[start.y as usize][start.x as usize] = replacer;
    new_rows
}

fn get_adjacent_pipes(point: Point, rows: &Vec<Vec<Tile>>) -> Vec<Point> {
    let current = rows[point.y as usize][point.x as usize];

    match current {
        Tile::VERT => vec![Point{x: point.x + 0, y: point.y + 1}, Point{x: point.x + 0, y: point.y - 1}],
        Tile::HOR  => vec![Point{x: point.x + 1, y: point.y + 0}, Point{x: point.x - 1, y: point.y - 0}],
        Tile::C7   => vec![Point{x: point.x - 1, y: point.y + 0}, Point{x: point.x + 0, y: point.y + 1}],
        Tile::CF   => vec![Point{x: point.x + 1, y: point.y + 0}, Point{x: point.x + 0, y: point.y + 1}],
        Tile::CL   => vec![Point{x: point.x + 1, y: point.y + 0}, Point{x: point.x + 0, y: point.y - 1}],
        Tile::CJ   => vec![Point{x: point.x - 1, y: point.y + 0}, Point{x: point.x + 0, y: point.y - 1}],
        x => panic!("Unexpected path tile {x:?}")
    }
}

fn get_adjacent_dots(point: Point, rows: &Vec<Vec<Tile>>) -> Vec<Point> {
    point.four_adjacent()
    .iter()
    .filter(|p| is_contained(p, rows))
    .filter(|p| matches!(rows[p.y as usize][p.x as usize], Tile::DOT))
    .cloned()
    .collect_vec()
}

fn find_distances(starts: &Vec<Point>, rows: &Vec<Vec<Tile>>, adjacent_fn: fn(Point, &Vec<Vec<Tile>>) -> Vec<Point>) -> HashMap<Point, i32> {

    let mut queue: VecDeque<Point> = VecDeque::default();
    let mut visited: HashSet<Point> = HashSet::default();
    let mut distances: HashMap<Point, i32> = HashMap::default();

    for s in starts {
        distances.insert(*s, 0);
        queue.push_back(s.clone());
    }

    while ! queue.is_empty()
    {
        let current = queue.pop_front().unwrap();
        let current_distance = distances[&current];
        // println!("Current {current:?} with distance {current_distance:?}");
        for p in adjacent_fn(current, rows).iter() {
            if ! visited.contains(p) {
                // println!("Adj {p:?}");
                visited.insert(p.clone());
                distances.insert(*p, current_distance + 1);
                queue.push_back(p.clone());
            }
        }
    }

    distances
}

pub fn first_part(input: &str) -> i32 {
    let rows: Vec<Vec<Tile>> = input
    .split("\n")
    .filter(|l| !l.trim().is_empty())
    .map(parse_row)
    .collect();

    let start = extract_start(&rows);

    let fixed_rows = fix_start(start, rows);

    let distances = find_distances(&vec![start], &fixed_rows, get_adjacent_pipes);
    *distances.values().max().unwrap()
}

fn enlarge(rows: &Vec<Vec<Tile>>, distances: &HashMap<Point, i32>) -> Vec<Vec<Tile>> {
    let mut enlarged: Vec<Vec<Tile>> = vec![vec![Tile::default(); rows.first().unwrap().len() * 3];rows.len() * 3];
    
    for (y, row) in rows.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let origin_y = (y * 3 + 1) as usize;
            let origin_x = (x * 3 + 1) as usize;

            if distances.contains_key(&Point{x: x as i32, y: y as i32}) {
                enlarged[origin_y][origin_x] = tile.clone();
                match tile {
                    Tile::VERT => {
                        enlarged[origin_y + 1][origin_x + 0] = Tile::VERT;
                        enlarged[origin_y - 1][origin_x + 0] = Tile::VERT;
                    },
                    Tile::HOR => {
                        enlarged[origin_y + 0][origin_x + 1] = Tile::HOR;
                        enlarged[origin_y + 0][origin_x - 1] = Tile::HOR;
                    },
                    Tile::C7 => {
                        enlarged[origin_y + 0][origin_x - 1] = Tile::HOR;
                        enlarged[origin_y + 1][origin_x + 0] = Tile::VERT;
                    },
                    Tile::CJ => {
                        enlarged[origin_y + 0][origin_x - 1] = Tile::HOR;
                        enlarged[origin_y - 1][origin_x + 0] = Tile::VERT;
                    },
                    Tile::CL => {
                        enlarged[origin_y + 0][origin_x + 1] = Tile::HOR;
                        enlarged[origin_y - 1][origin_x + 0] = Tile::VERT;
                    },
                    Tile::CF => {
                        enlarged[origin_y + 0][origin_x + 1] = Tile::HOR;
                        enlarged[origin_y + 1][origin_x + 0] = Tile::VERT;
                    },
                    x => panic!("Unknown enlarging tile: {x:?}")       
                }
            } else {
                enlarged[origin_y][origin_x] = Tile::DOT;
            }
        }
    }
    enlarged
}


fn get_edge_points(rows: &Vec<Vec<Tile>>) -> Vec<Point> {
    let w = rows.first().unwrap().len() as i32;
    let h = rows.len() as i32;
    
    let mut edge_points: Vec<Point> = vec![];
    for y in 0..h {
        edge_points.push(Point { x: 0, y: y});
        edge_points.push(Point { x: w - 1, y});
    }
    for x in 1..(w - 1) {
        edge_points.push(Point { x: x, y: 0});
        edge_points.push(Point { x: x, y: h - 1});    
    }
    edge_points
}

fn print_rows(rows: &Vec<Vec<Tile>>, marked_points: Option<&HashSet<&Point>>) {
    for (y, row) in rows.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            let t = rows[y as usize][x as usize];
            let p = Point{x: x as i32, y: y as i32};
            if marked_points.unwrap_or(&HashSet::new()).contains(&p) {
                print!("x");
            } else {
                print!("{t:?}");
            }
        }
        println!("")
    }
}

pub fn second_part(input: &str) -> i32 {
    let rows: Vec<Vec<Tile>> = input
    .split("\n")
    .filter(|l| !l.trim().is_empty())
    .map(parse_row)
    .collect();
    println!("rows:");
    print_rows(&rows, None);


    let start = extract_start(&rows);
    let fixed_rows = fix_start(start, rows);
    println!("fixed rows:");
    print_rows(&fixed_rows, None);

    let pipe_distances = find_distances(&vec![start], &fixed_rows, get_adjacent_pipes);

    let enlarged: Vec<Vec<Tile>> = enlarge(&fixed_rows, &pipe_distances);
    println!("enlarged:");
    print_rows(&enlarged, None);


    let reachable_from_edges: HashMap<Point, i32> = find_distances(&get_edge_points(&enlarged), &enlarged, get_adjacent_dots);
    // println!("reachable_from_edges: {reachable_from_edges:?}");

    println!("reachable:");
    print_rows(&enlarged, Some(&HashSet::from_iter(reachable_from_edges.keys())));


    let mut dot_sum = 0;
    for (y, row) in fixed_rows.iter().enumerate() {
        for (x, original_tile) in row.iter().enumerate() {
            let enlarged_point = Point{x: (x as i32 * 3) + 1, y: (y as i32 * 3) + 1};

            if !pipe_distances.contains_key(&Point { x: x as i32, y: y as i32}) && !reachable_from_edges.contains_key(&enlarged_point) {
                dot_sum += 1;
            }       
        }
    }
    dot_sum
}

#[cfg(test)]
mod tests {
    use crate::day_10::*;
    
    #[test]
    fn test_example() {
        assert_eq!(first_part(include_str!("inputs/10_example_1.txt")), 4);
        assert_eq!(first_part(include_str!("inputs/10_example_2.txt")), 8);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(second_part(include_str!("inputs/10_example_1.txt")), 1);
        assert_eq!(second_part(include_str!("inputs/10_example_2.txt")), 1);
        assert_eq!(second_part(include_str!("inputs/10_example_3.txt")), 4);
        assert_eq!(second_part(include_str!("inputs/10_example_4.txt")), 10);
    }

    #[test]
    fn test_parts() {
        assert_eq!(first_part(include_str!("inputs/10.secret")), 6725);
        assert_eq!(second_part(include_str!("inputs/10.secret")), 383);
    }
}