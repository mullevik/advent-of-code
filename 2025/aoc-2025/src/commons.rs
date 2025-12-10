#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x: x, y: y }
    }
}

#[macro_export]
macro_rules! vec2 {
    ($a:expr, $b:expr) => {
        Vec2::new($a, $b)
    };
}
