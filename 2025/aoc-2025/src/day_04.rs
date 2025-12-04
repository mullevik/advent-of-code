struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    fn new(x: T, y: T) -> Self {
        Vec2 { x: x, y: y }
    }
}

pub fn p1(input: &str) -> i32 {
    let grid = parse(input);
    let dim = Vec2::new(grid.first().unwrap().len() as i32, grid.len() as i32);
    let mut n_accessible_papers = 0;

    for y in 0..dim.y {
        for x in 0..dim.x {
            let p = Vec2::new(x, y);
            if grid[p.y as usize][p.x as usize] == '@' {
                let adjacents = eight_adjacents(&p, &dim);

                let n_papers_around = adjacents
                    .iter()
                    .filter(|a| grid[a.y as usize][a.x as usize] == '@')
                    .count();

                if n_papers_around < 4 {
                    n_accessible_papers += 1;
                }
            }
        }
    }
    n_accessible_papers
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .split("\n")
        .filter(|x| x.trim().len() > 0)
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn eight_adjacents(p: &Vec2<i32>, dim: &Vec2<i32>) -> Vec<Vec2<i32>> {
    let (w, h) = (dim.x, dim.y);

    [
        Vec2::new(p.x - 1, p.y + 0),
        Vec2::new(p.x - 1, p.y + 1),
        Vec2::new(p.x - 1, p.y - 1),
        Vec2::new(p.x + 1, p.y + 0),
        Vec2::new(p.x + 1, p.y + 1),
        Vec2::new(p.x + 1, p.y - 1),
        Vec2::new(p.x + 0, p.y - 1),
        Vec2::new(p.x + 0, p.y + 1),
    ]
    .into_iter()
    .filter(|s| s.x >= 0 && s.x < w && s.y >= 0 && s.y < h)
    .collect::<Vec<_>>()
}

mod test {
    use crate::day_04::p1;
    use std::fs;

    #[test]
    fn test_p1() {
        let input = fs::read_to_string("inputs/04.custom").unwrap();
        assert_eq!(p1(&input), 2);
        let input = fs::read_to_string("inputs/04.example").unwrap();
        assert_eq!(p1(&input), 13);
    }
}
