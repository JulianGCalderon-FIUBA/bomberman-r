use std::collections::HashSet;

use crate::board::Board;
use crate::cell::{BombKind, Cell};
use crate::direction::Direction;
use crate::error::MyError;
use crate::position::Position;

const BOMB_DAMAGE: u8 = 1;

pub struct Game {
    board: Board,
}

impl Game {
    pub fn with_board(board: Board) -> Self {
        Game { board }
    }

    pub fn trigger_bomb(&mut self, position: Position) -> Result<(), MyError> {
        if !self.board.contains(position) {
            return Err(MyError::OutOfBounds);
        }

        let cell = self.board.get_cell(position);

        match cell {
            Cell::Bomb(range, kind) => self.explode_bomb(position, range, kind),
            _ => return Err(MyError::NotABomb),
        }

        Ok(())
    }

    fn explode_bomb(&mut self, position: Position, range: u8, kind: BombKind) {
        self.board.set_cell(position, Cell::Empty);

        let positions = self.propagate_explosion(position, range, kind);

        self.explode_positions(positions);
    }

    fn propagate_explosion(
        &mut self,
        position: Position,
        range: u8,
        kind: BombKind,
    ) -> HashSet<Position> {
        let mut affected_positions = HashSet::new();
        for direction in Direction::variants() {
            let new_affected_positions =
                self.propagate_explosion_in_direction(position, direction, range, kind);

            affected_positions.extend(&new_affected_positions);
        }
        affected_positions
    }

    fn propagate_explosion_in_direction(
        &self,
        mut position: Position,
        mut direction: Direction,
        mut range: u8,
        kind: BombKind,
    ) -> HashSet<Position> {
        let mut positions = HashSet::new();

        while range > 0 {
            match self.advance_position(position, direction) {
                Ok(new_position) => position = new_position,
                Err(_) => break,
            }

            positions.insert(position);

            let cell = self.board.get_cell(position);

            if Cell::Wall == cell || (Cell::Rock == cell && kind == BombKind::Normal) {
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
            self.explode(position);
        }
    }

    fn explode(&mut self, position: Position) {
        let cell_to_explode = self.board.get_cell(position);

        match cell_to_explode {
            Cell::Enemy(hp) => self.explode_enemy(position, hp),
            Cell::Bomb(range, kind) => self.explode_bomb(position, range, kind),
            _ => (),
        }
    }

    fn explode_enemy(&mut self, position: Position, mut hp: u8) {
        hp = hp.saturating_sub(BOMB_DAMAGE);

        let new_cell = if hp == 0 {
            Cell::Empty
        } else {
            Cell::Enemy(hp)
        };

        self.board.set_cell(position, new_cell);
    }

    fn advance_position(
        &self,
        position: Position,
        direction: Direction,
    ) -> Result<Position, MyError> {
        let position = position.advance(direction);

        if !self.board.contains(position) {
            return Err(MyError::OutOfBounds);
        }

        Ok(position)
    }

    pub fn board(&self) -> &Board {
        &self.board
    }
}
