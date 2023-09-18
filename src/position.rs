use crate::direction::Direction;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn advance(mut self, direction: Direction) -> Self {
        match direction {
            Direction::Up => self.y = self.y.wrapping_sub(1),
            Direction::Down => self.y += 1,
            Direction::Left => self.x = self.x.wrapping_sub(1),
            Direction::Right => self.x += 1,
        }

        self
    }
}
