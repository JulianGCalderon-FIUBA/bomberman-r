use std::collections::HashSet;

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
    pub fn with_board(board: Board) -> Self {
        Game { board }
    }

    pub fn trigger_bomb(&mut self, position: Position) -> Result<(), Error> {
        let bomb = self.board.get_cell(position)?;

        match bomb {
            Cell::Bomb(range) => self.explode_bomb(position, range, false),
            Cell::PierceBomb(range) => self.explode_bomb(position, range, true),
            _ => return Err(Error::NotABomb),
        }

        Ok(())
    }

    fn explode(&mut self, position: Position) -> Result<(), Error> {
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

        let new_cell = if hp == 0 {
            Cell::Empty
        } else {
            Cell::Enemy(hp)
        };

        self.board
            .set_cell(position, new_cell)
            .expect("Already Checked");
    }

    fn explode_bomb(&mut self, position: Position, range: u8, pierce: bool) {
        self.board
            .set_cell(position, Cell::Empty)
            .expect("Already Checked");

        let positions = self.propagate_explosion(position, range, pierce);

        self.explode_positions(positions);
    }

    fn propagate_explosion(
        &mut self,
        position: Position,
        range: u8,
        pierce: bool,
    ) -> HashSet<Position> {
        let mut affected_positions = HashSet::new();
        for direction in Direction::variants() {
            let new_affected_positions =
                self.propagate_explosion_in_direction(position, direction, range, pierce);

            affected_positions.extend(&new_affected_positions);
        }
        affected_positions
    }

    fn propagate_explosion_in_direction(
        &self,
        mut position: Position,
        mut direction: Direction,
        mut range: u8,
        pierce: bool,
    ) -> HashSet<Position> {
        let mut positions = HashSet::new();

        while range > 0 {
            match self.advance_position(position, direction) {
                Ok(new_position) => position = new_position,
                Err(_) => break,
            }

            positions.insert(position);

            let cell = self
                .board
                .get_cell(position)
                .expect("Should be valid position");

            if Cell::Wall == cell || (Cell::Rock == cell && !pierce) {
                break;
            }

            if let Cell::Detour(new_direction) = cell {
                direction = new_direction
            }

            range -= 1;
        }

        positions
    }

    fn explode_positions(&mut self, positions: HashSet<Position>) {
        for position in positions.into_iter() {
            self.explode(position).expect("Should be valid position");
        }
    }

    fn advance_position(
        &self,
        position: Position,
        direction: Direction,
    ) -> Result<Position, Error> {
        let position = position.advance(direction);

        if position.x >= self.board.width() || position.y >= self.board.height() {
            return Err(Error::OutOfBounds);
        }

        Ok(position)
    }

    pub fn board(&self) -> &Board {
        &self.board
    }
}
