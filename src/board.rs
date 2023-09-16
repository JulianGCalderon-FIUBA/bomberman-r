use crate::cell::Cell;
use crate::direction::Direction;
use crate::error::Error;
use crate::position::Position;

const BOMB_DAMAGE: u8 = 2;

pub struct Board {
    pub cells: Vec<Vec<Cell>>,
}

impl Board {
    pub fn explode(&mut self, position: Position) -> Result<(), Error> {
        let cell_to_explode = self.get_cell(position)?;

        match cell_to_explode {
            Cell::Enemy(hp) => self.explode_enemy(position, hp),
            Cell::Bomb(range) => self.explode_bomb(position, range, false),
            Cell::PierceBomb(range) => self.explode_bomb(position, range, true),
            _ => (),
        }

        Ok(())
    }

    fn explode_enemy(&mut self, position: Position, mut hp: u8) {
        hp = hp.saturating_sub(BOMB_DAMAGE);

        if hp == 0 {
            self.set_cell(position, Cell::Empty);
        } else {
            self.set_cell(position, Cell::Enemy(hp));
        }
    }

    fn explode_bomb(&mut self, position: Position, range: u8, pierce: bool) {
        self.set_cell(position, Cell::Empty);

        for direction in Direction::variants() {
            let _ = self.propagate_explosion(position, direction, range, pierce);
        }
    }

    fn propagate_explosion(
        &mut self,
        position: Position,
        direction: Direction,
        range: u8,
        pierce: bool,
    ) -> Result<(), Error> {
        if range == 0 {
            return Ok(());
        }

        let position = position.advance(direction)?;

        self.explode(position)?;

        let cell = self.get_cell(position)?;

        if Cell::Wall == cell || (Cell::Rock == cell && !pierce) {
            return Ok(());
        }

        if let Cell::Detour(new_direction) = cell {
            let _ = self.propagate_explosion(position, new_direction, range - 1, pierce);
        } else {
            let _ = self.propagate_explosion(position, direction, range - 1, pierce);
        }

        Ok(())
    }

    fn get_cell(&mut self, position: Position) -> Result<Cell, Error> {
        self.cells
            .get(position.y)
            .map(|row| row.get(position.x).cloned())
            .flatten()
            .ok_or(Error::OutOfBounds)
    }

    fn set_cell(&mut self, position: Position, cell: Cell) {
        self.cells[position.y][position.x] = cell;
    }
}

impl TryFrom<&String> for Board {
    type Error = Error;

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        Board::try_from(s.as_str())
    }
}

impl TryFrom<&str> for Board {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut cells = Vec::new();

        for line in s.as_bytes().split(|&b| b == b'\n') {
            if line.is_empty() {
                continue;
            }

            let mut row = Vec::new();

            for ascii_cell in line.split(|&b| b == b' ') {
                let cell = Cell::try_from(ascii_cell)?;
                row.push(cell);
            }

            cells.push(row);
        }

        Ok(Board { cells })
    }
}

impl ToString for Board {
    fn to_string(&self) -> String {
        let mut s = String::new();

        for row in &self.cells {
            for cell in row {
                s.push_str(&cell.to_string());
                s.push(' ');
            }
            s.pop();
            s.push('\n');
        }

        s
    }
}
