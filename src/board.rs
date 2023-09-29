use std::fmt::Display;
use std::path::Path;
use std::str::FromStr;

use crate::cell::Cell;
use crate::error::{MyError, MyResult};
use crate::io::{read_from_file, write_to_file};
use crate::position::Position;

pub struct Board {
    cells: Vec<Vec<Cell>>,
}

impl Board {
    pub fn from_file(path: &Path) -> MyResult<Self> {
        let content = read_from_file(path)?;

        content.parse()
    }

    pub fn get_cell(&self, position: Position) -> Cell {
        self.cells[position.y][position.x]
    }

    pub fn set_cell(&mut self, position: Position, cell: Cell) {
        self.cells[position.y][position.x] = cell;
    }

    pub fn contains(&self, position: Position) -> bool {
        return position.x < self.width() && position.y < self.height();
    }

    pub fn width(&self) -> usize {
        self.cells.get(0).map_or(0, |row| row.len())
    }

    pub fn height(&self) -> usize {
        self.cells.len()
    }

    pub fn to_file(&self, output_path: &Path) -> Result<(), MyError> {
        let output = self.to_string();
        write_to_file(output_path, &output)?;

        Ok(())
    }
}

impl FromStr for Board {
    type Err = MyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.cells {
            let mut row_iter = row.iter();

            if let Some(first) = row_iter.next() {
                write!(f, "{}", first)?;
            } else {
                continue;
            }

            for element in row_iter {
                write!(f, " {}", element)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}
