use pyo3::*;
use std::collections::BTreeMap;

use super::errors::{
    CannotEnterFieldError,
    CardNotOwnedError,
    FieldOccupiedError,
    GoalConditionsError,
    HedgehogOnlyBackwardsError,
    MissingCarrotsError,
};
use super::field::Field;
use super::hare::Hare;
use super::board::Board;
use super::r#move::Move;

#[pyclass]
pub struct GameState {
    pub board: Board,
    pub turn: usize,
    player_one: Hare,
    player_two: Hare,
    pub moves: BTreeMap<usize, Move>,
}

#[pymethods]
impl GameState {
    #[new]
    pub fn new(
        board: Board,
        turn: usize,
        player_one: Hare,
        player_two: Hare,
        moves: BTreeMap<usize, Move>
    ) -> Self {
        Self {
            board,
            turn,
            player_one,
            player_two,
            moves,
        }
    }

    pub fn get_current(&self) -> Hare {
        if self.turn % 2 == 0 { self.player_one.clone() } else { self.player_two.clone() }
    }

    pub fn set_current(&mut self, player: Hare) {
        if self.turn % 2 == 0 {
            self.player_one = player;
        } else {
            self.player_two = player;
        }
    }

    pub fn get_opponent(&self, player: &Hare) -> Hare {
        if player.team == self.player_one.team {
            return self.player_two.clone();
        }
        self.player_one.clone()
    }

    pub fn set_opponent(&mut self, player: Hare) {
        if player.team == self.player_one.team {
            self.player_two = player;
        } else {
            self.player_one = player;
        }
    }

    pub fn is_ahead(&self, player: &Hare) -> bool {
        player.position > self.get_opponent(player).position
    }

    pub fn can_exchange_carrots(&self, player: &Hare, count: i32) -> Result<bool, PyErr> {
        match self.board.get_field(player.position) {
            Some(f) =>
                Ok(f == Field::Carrots && (count == 10 || (count == -10 && player.carrots >= 10))),
            None => Err(CannotEnterFieldError::new_err("Field not found")),
        }
    }

    pub fn must_eat_salad(&self, player: &Hare) -> Result<bool, PyErr> {
        match self.board.get_field(player.position) {
            Some(f) => Ok(f == Field::Salad && !player.salad_eaten),
            None => Err(CannotEnterFieldError::new_err("Field not found")),
        }
    }

    pub fn eat_salad(&self, player: &mut Hare) -> Result<(), PyErr> {
        player.eat_salad()?;
        match self.is_ahead(player) {
            true => {
                player.carrots += 10;
            }
            false => {
                player.carrots += 30;
            }
        }
        Ok(())
    }

    pub fn get_fall_back(&self, player: &Hare) -> Option<usize> {
        match self.board.get_previous_field(Field::Hedgehog, player.position) {
            Some(i) if self.get_opponent(player).position != i => Some(i),
            Some(_) => None,
            None => None,
        }
    }

    pub fn move_to_field(&self, player: &mut Hare, new_position: usize) -> Result<(), PyErr> {
        self.can_advance_to(new_position, player)?;
        player.position = new_position;
        Ok(())
    }

    pub fn can_advance_to(&self, new_position: usize, player: &Hare) -> Result<(), PyErr> {
        assert!(new_position > player.position); // TODO address backwards too

        if new_position == 0 {
            return Err(CannotEnterFieldError::new_err("Cannot jump to position 0"));
        }

        let field = match self.board.get_field(new_position) {
            Some(f) => f,
            None => {
                return Err(CannotEnterFieldError::new_err("Field not found"));
            }
        };

        if field != Field::Goal && new_position == self.get_opponent(player).position {
            return Err(FieldOccupiedError::new_err("Field is occupied by opponent"));
        }

        match field {
            Field::Hedgehog => {
                Err(HedgehogOnlyBackwardsError::new_err("You cannot go on Hedgehog field forwards"))
            }
            Field::Salad => if player.salads > 0 {
                Ok(())
            } else {
                Err(FieldOccupiedError::new_err("Field is occupied by opponent"))
            }
            Field::Hare => if !player.cards.is_empty() {
                Ok(())
            } else {
                Err(CardNotOwnedError::new_err("No card to play"))
            }
            Field::Market => if player.carrots >= 10 {
                Ok(())
            } else {
                Err(MissingCarrotsError::new_err("Not enough carrots"))
            }
            Field::Goal => if player.carrots <= 10 && player.salads == 0 {
                Ok(())
            } else {
                Err(GoalConditionsError::new_err("Too much carrots or/and salads"))
            }
            _ => { Ok(()) }
        }
    }
}
