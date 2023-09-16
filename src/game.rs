use crate::board::Board;
use crate::cell::Cell;
use crate::direction::Direction;
use crate::error::Error;
use crate::position::Position;

const BOMB_DAMAGE: u8 = 2;

pub struct Game {
    board: Board,
}

impl Game {
    pub fn explode(&mut self, position: Position) -> Result<(), Error> {
        let cell_to_explode = self.board.get_cell(position)?;

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
            self.board.set_cell(position, Cell::Empty);
        } else {
            self.board.set_cell(position, Cell::Enemy(hp));
        }
    }

    fn explode_bomb(&mut self, position: Position, range: u8, pierce: bool) {
        self.board.set_cell(position, Cell::Empty);

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

        let cell = self.board.get_cell(position)?;

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
}

impl TryFrom<&String> for Game {
    type Error = Error;

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        Game::try_from(s.as_str())
    }
}

impl TryFrom<&str> for Game {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let board = Board::try_from(s)?;

        Ok(Self { board })
    }
}

impl ToString for Game {
    fn to_string(&self) -> String {
        self.board.to_string()
    }
}
