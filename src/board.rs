use crate::cell::Cell;
use crate::error::Error;
use crate::position::Position;

pub struct Board {
    cells: Vec<Vec<Cell>>,
}

impl Board {
    pub fn get_cell(&self, position: Position) -> Result<Cell, Error> {
        self.cells
            .get(position.y)
            .map(|row| row.get(position.x).cloned())
            .flatten()
            .ok_or(Error::OutOfBounds)
    }

    pub fn set_cell(&mut self, position: Position, cell: Cell) {
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
