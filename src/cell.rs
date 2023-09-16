use crate::direction::Direction;
use crate::error::Error;

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

impl TryFrom<&[u8]> for Cell {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let x = value[0];
        let y = value.get(1).copied();

        let cell = match (x, y) {
            (b'R', _) => Cell::Rock,
            (b'W', _) => Cell::Wall,
            (b'_', _) => Cell::Empty,
            (b'F', Some(y)) => Cell::Enemy(y - b'0'),
            (b'B', Some(y)) => Cell::Bomb(y - b'0'),
            (b'S', Some(y)) => Cell::PierceBomb(y - b'0'),
            (b'D', Some(y)) => Cell::Detour(Direction::try_from(y)?),
            _ => Err(Error::InvalidCell)?,
        };

        Ok(cell)
    }
}

impl ToString for Cell {
    fn to_string(&self) -> String {
        match self {
            Cell::Enemy(hp) => format!("F{}", hp),
            Cell::Bomb(range) => format!("B{}", range),
            Cell::PierceBomb(range) => format!("S{}", range),
            Cell::Detour(direction) => format!("D{}", direction.to_string()),
            Cell::Rock => "R".to_string(),
            Cell::Wall => "W".to_string(),
            Cell::Empty => "_".to_string(),
        }
    }
}
