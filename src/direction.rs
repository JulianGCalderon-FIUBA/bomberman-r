#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn advance(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        match self {
            Direction::Up => Some((x, y.checked_sub(1)?)),
            Direction::Down => Some((x, y + 1)),
            Direction::Left => Some((x.checked_sub(1)?, y)),
            Direction::Right => Some((x + 1, y)),
        }
    }
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            b'U' => Direction::Up,
            b'D' => Direction::Down,
            b'L' => Direction::Left,
            b'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::Up => "U".to_string(),
            Direction::Down => "D".to_string(),
            Direction::Left => "L".to_string(),
            Direction::Right => "R".to_string(),
        }
    }
}
