use std::fmt::Display;

use crate::direction::Direction;
use crate::error::MyError;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BombKind {
    Normal,
    Pierce,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Enemy(u8),
    Bomb(u8, BombKind),
    Detour(Direction),
    Rock,
    Wall,
    Empty,
}

impl TryFrom<&[u8]> for Cell {
    type Error = MyError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let x = value[0];
        let y = value.get(1).copied();

        let cell = match (x, y) {
            (b'R', _) => Cell::Rock,
            (b'W', _) => Cell::Wall,
            (b'_', _) => Cell::Empty,
            (b'F', Some(y)) => Cell::Enemy(y - b'0'),
            (b'B', Some(y)) => Cell::Bomb(y - b'0', BombKind::Normal),
            (b'S', Some(y)) => Cell::Bomb(y - b'0', BombKind::Pierce),
            (b'D', Some(y)) => Cell::Detour(Direction::try_from(y)?),
            _ => Err(MyError::InvalidCell)?,
        };

        Ok(cell)
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Enemy(hp) => write!(f, "F{}", hp),
            Cell::Bomb(range, kind) => match kind {
                BombKind::Normal => write!(f, "B{}", range),
                BombKind::Pierce => write!(f, "S{}", range),
            },
            Cell::Detour(direction) => write!(f, "D{}", direction),
            Cell::Rock => write!(f, "R"),
            Cell::Wall => write!(f, "W"),
            Cell::Empty => write!(f, "_"),
        }
    }
}
