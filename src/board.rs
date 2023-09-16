use crate::cell::Cell;
use crate::direction::Direction;

pub struct Board {
    pub cells: Vec<Vec<Cell>>,
}

impl Board {
    pub fn explode(&mut self, x: usize, y: usize) {
        let cell_to_explode = self.cells[y][x];

        match cell_to_explode {
            Cell::Bomb(range) => self.explode_bomb(x, y, range, false),
            Cell::PierceBomb(range) => self.explode_bomb(x, y, range, true),
            Cell::Enemy(hp) => self.explode_enemy(x, y, hp),
            _ => (),
        }
    }

    fn explode_bomb(&mut self, x: usize, y: usize, range: u8, pierce: bool) {
        self.cells[y][x] = Cell::Empty;

        self.propagate_explosion(x, y, range, pierce, Direction::Up);
        self.propagate_explosion(x, y, range, pierce, Direction::Down);
        self.propagate_explosion(x, y, range, pierce, Direction::Left);
        self.propagate_explosion(x, y, range, pierce, Direction::Right);
    }

    fn propagate_explosion(
        &mut self,
        mut x: usize,
        mut y: usize,
        range: u8,
        pierce: bool,
        mut direction: Direction,
    ) {
        if range == 0 {
            return;
        }

        (x, y) = if let Some((x, y)) = direction.advance(x, y) {
            (x, y)
        } else {
            return;
        };

        let cell = if let Some(cell) = self.get_cell(y, x) {
            cell
        } else {
            return;
        };

        self.explode(x, y);

        if Cell::Wall == cell || (Cell::Rock == cell && !pierce) {
            return;
        }

        if let Cell::Detour(detour_direction) = cell {
            direction = detour_direction
        }

        self.propagate_explosion(x, y, range - 1, pierce, direction);
    }

    fn get_cell(&mut self, y: usize, x: usize) -> Option<Cell> {
        self.cells.get(y).map(|row| row.get(x).cloned()).flatten()
    }

    fn explode_enemy(&mut self, x: usize, y: usize, hp: u8) {
        self.cells[y][x] = if hp == 1 || hp == 2 {
            Cell::Empty
        } else {
            Cell::Enemy(hp - 2)
        }
    }
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
