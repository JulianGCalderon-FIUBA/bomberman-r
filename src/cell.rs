use crate::direction::Direction;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Enemy(u8),
    Bomb(u8),
    PierceBomb(u8),
    Detour(Direction),
    Rock,
    Wall,
    Empty,
}

impl From<&[u8]> for Cell {
    fn from(value: &[u8]) -> Self {
        let x = value[0];
        let y = value.get(1).copied();

        match (x, y) {
            (b'R', _) => Cell::Rock,
            (b'W', _) => Cell::Wall,
            (b'_', _) => Cell::Empty,
            (b'F', Some(y)) => Cell::Enemy(y - b'0'),
            (b'B', Some(y)) => Cell::Bomb(y - b'0'),
            (b'S', Some(y)) => Cell::PierceBomb(y - b'0'),
            (b'D', Some(y)) => Cell::Detour(Direction::from(y)),
            _ => panic!("Invalid cell"),
        }
    }
}

impl ToString for Cell {
    fn to_string(&self) -> String {
        match self {
            Cell::Enemy(y) => format!("F{}", y),
            Cell::Bomb(y) => format!("B{}", y),
            Cell::PierceBomb(y) => format!("S{}", y),
            Cell::Detour(direction) => format!("D{}", direction.to_string()),
            Cell::Rock => "R".to_string(),
            Cell::Wall => "W".to_string(),
            Cell::Empty => "_".to_string(),
        }
    }
}
