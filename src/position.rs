use crate::direction::Direction;
use crate::error::Error;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn advance(mut self, direction: Direction) -> Result<Self, Error> {
        match direction {
            Direction::Up => self.y = self.y.checked_sub(1).ok_or(Error::OutOfBounds)?,
            Direction::Down => self.y += 1,
            Direction::Left => self.x = self.x.checked_sub(1).ok_or(Error::OutOfBounds)?,
            Direction::Right => self.x += 1,
        }

        Ok(self)
    }
}
