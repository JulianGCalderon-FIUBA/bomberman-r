use crate::cell::Cell;

pub struct Board {
    pub cells: Vec<Vec<Cell>>,
}

impl<S: AsRef<str>> From<S> for Board {
    fn from(s: S) -> Self {
        let mut cells = Vec::new();

        for line in s.as_ref().as_bytes().split(|&b| b == b'\n') {
            if line.is_empty() {
                continue;
            }

            let mut row = Vec::new();

            for ascii_cell in line.split(|&b| b == b' ') {
                let cell = Cell::from(ascii_cell);
                row.push(cell);
            }

            cells.push(row);
        }

        Board { cells }
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
