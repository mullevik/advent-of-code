use core::fmt;
use std::cmp::max;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GridError {
    DimensionError,
    AccessError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

#[macro_export]
macro_rules! p {
    ($x:expr, $y:expr) => {
        Point::new($x, $y)
    };
}

pub struct Grid<T: std::clone::Clone> {
    positions: Vec<Vec<T>>,
}
impl<T: std::clone::Clone> Grid<T> {
    pub fn from_rows(rows: impl Iterator<Item = Vec<T>>) -> Result<Self, GridError> {
        let collected_rows = rows.collect::<Vec<_>>();

        if !are_same_length(&collected_rows) {
            Err(GridError::DimensionError)
        } else {
            Ok(Self {
                positions: collected_rows,
            })
        }
    }

    pub fn full(width: usize, height: usize, fill_value: T) -> Self {
        Grid::from_rows(vec![vec![fill_value; width]; height].into_iter()).unwrap()
    }

    pub fn height(&self) -> usize {
        self.positions.len()
    }

    pub fn width(&self) -> usize {
        if self.positions.is_empty() {
            0
        } else {
            self.positions.first().unwrap().len()
        }
    }

    pub fn len(&self) -> usize {
        self.height() * self.width()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn contains(&self, p: &Point) -> bool {
        (p.x >= 0 && (max(p.x, 0) as usize) < self.width())
            && (p.y >= 0 && (max(p.y, 0) as usize) < self.height())
    }

    pub fn at(&self, p: &Point) -> Result<&T, GridError> {
        if self.contains(p) {
            Ok(&self.positions[p.y as usize][p.x as usize])
        } else {
            Err(GridError::AccessError)
        }
    }

    pub fn at_mut(&mut self, p: &Point) -> Result<&mut T, GridError> {
        if self.contains(p) {
            Ok(&mut self.positions[p.y as usize][p.x as usize])
        } else {
            Err(GridError::AccessError)
        }
    }

    pub fn at_xy(&self, x: usize, y: usize) -> Result<&T, GridError> {
        self.at(&Point::new(x as i32, y as i32))
    }

    pub fn four_neighborhood_at(&self, p: &Point) -> Vec<(Point, &T)> {
        [
            Point::new(p.x - 1, p.y),
            Point::new(p.x + 1, p.y),
            Point::new(p.x, p.y - 1),
            Point::new(p.x, p.y + 1),
        ]
        .iter()
        .map(|_p| (_p, self.at(_p)))
        .filter(|(_p, _v)| _v.is_ok())
        .map(|(_p, _v)| (*_p, _v.unwrap()))
        .collect::<Vec<_>>()
    }
    pub fn four_neighborhood_at_xy(&self, x: usize, y: usize) -> Vec<(Point, &T)> {
        self.four_neighborhood_at(&Point::new(x as i32, y as i32))
    }

    pub fn eight_neighborhood_at(&self, p: &Point) -> Vec<(Point, &T)> {
        [
            Point::new(p.x - 1, p.y),
            Point::new(p.x + 1, p.y),
            Point::new(p.x, p.y - 1),
            Point::new(p.x - 1, p.y - 1),
            Point::new(p.x + 1, p.y - 1),
            Point::new(p.x, p.y + 1),
            Point::new(p.x - 1, p.y + 1),
            Point::new(p.x + 1, p.y + 1),
        ]
        .iter()
        .map(|_p| (_p, self.at(_p)))
        .filter(|(_p, _v)| _v.is_ok())
        .map(|(_p, _v)| (*_p, _v.unwrap()))
        .collect::<Vec<_>>()
    }

    pub fn eight_neighborhood_xy(&self, x: usize, y: usize) -> Vec<(Point, &T)> {
        self.eight_neighborhood_at(&Point::new(x as i32, y as i32))
    }

    pub fn iter_points(&self) -> GridPointIterator<T> {
        GridPointIterator {
            index: 0,
            grid: self,
        }
    }
    pub fn iter_values(&self) -> impl Iterator<Item = &T> {
        self.iter_points().map(|p| self.at(&p).unwrap())
    }
    pub fn iter(&self) -> impl Iterator<Item = (Point, &T)> {
        self.iter_points().map(|p| (p, self.at(&p).unwrap()))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Point, &mut T)> {
        self.positions
            .iter_mut()
            .enumerate()
            .map(|(y, row)| {
                row.iter_mut()
                    .enumerate()
                    .map(move |(x, val)| (Point::new(x as i32, y as i32), val))
            })
            .flatten()
    }
}

impl<T: std::clone::Clone> fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Grid{{ w: {}, h: {} }}", self.width(), self.height())
    }
}

impl<T: std::clone::Clone> FromIterator<Vec<T>> for Grid<T> {
    fn from_iter<M: IntoIterator<Item = Vec<T>>>(iter: M) -> Self {
        Self::from_rows(iter.into_iter()).unwrap()
    }
}

pub struct GridPointIterator<'a, T: std::clone::Clone> {
    index: usize,
    grid: &'a Grid<T>,
}

impl<'a, T: std::clone::Clone> Iterator for GridPointIterator<'a, T> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.grid.len() {
            None
        } else {
            let current_position = Point::new(
                (self.index % self.grid.width()) as i32,
                (self.index / self.grid.width()) as i32,
            );
            self.index += 1;
            Some(current_position)
        }
    }
}

fn are_same_length<T>(vectors: &[Vec<T>]) -> bool {
    if vectors.is_empty() {
        true
    } else {
        let first_length = vectors.first().unwrap().len();
        vectors.iter().all(|v| v.len() == first_length)
    }
}

mod tests_grid {
    use super::*;

    #[test]
    fn test_create() {
        let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];

        let g = Grid::from_rows(rows.iter().cloned()).unwrap();
        assert_eq!(g.width(), 3);
        assert_eq!(g.height(), 2);
        assert_eq!(g.len(), 6);
    }

    #[test]
    fn test_empty_grid() {
        let empty_rows: Vec<Vec<i32>> = vec![];
        let empty_g = Grid::from_rows(empty_rows.iter().cloned()).unwrap();
        assert!(empty_g.is_empty());
        assert_eq!(empty_g.len(), 0);
    }

    #[test]
    fn test_access() {
        let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];

        let g = rows.iter().cloned().collect::<Grid<_>>();
        assert_eq!(g.at_xy(0, 0).unwrap(), &1);
        assert_eq!(g.at_xy(100, 100).err().unwrap(), GridError::AccessError);
        assert_eq!(
            g.four_neighborhood_at_xy(1, 1)
                .iter()
                .cloned()
                .map(|(p, v)| v)
                .collect::<Vec<_>>(),
            [4, 6, 2].iter().collect::<Vec<_>>()
        );
        assert_eq!(
            g.iter_values().cloned().collect::<Vec<_>>(),
            vec![1, 2, 3, 4, 5, 6]
        );
    }
}
