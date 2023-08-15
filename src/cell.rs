#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cell {
    pub x: i64,
    pub y: i64,
    pub is_alive: bool,
}

impl Cell {
    pub fn new(x: i64, y: i64, is_alive: bool) -> Cell {
        Cell { x, y, is_alive }
    }
}
