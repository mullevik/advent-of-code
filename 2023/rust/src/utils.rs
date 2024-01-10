use std::ops;


pub fn get_w_h_dimensions<T>(grid: &Vec<Vec<T>>) -> (usize, usize) {
    (grid.iter().next().unwrap().len(), grid.len())
}

pub fn show<T: std::fmt::Debug>(grid: &Vec<Vec<T>>) {
    for (y, row) in grid.iter().enumerate() {
        for (x, item) in row.iter().enumerate() {
            print!("{item:?}");
        }
        println!("")
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vec2<T: std::ops::Add> {
    pub x: T,
    pub y: T
}
impl<T: std::ops::Add + std::ops::Add<Output = T>> ops::Add<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: Vec2<T>) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x, 
            y: self.y + rhs.y
        }
    }
}