use pyo3::*;

use crate::plugin::errors::{
    CardNotOwnedError, FieldOccupiedError, GoalConditionsError, HedgehogOnlyBackwardsError,
    MissingCarrotsError,
};

use super::{
    action::Action, board::Board, errors::CannotEnterFieldError, field::Field, hare::Hare,
    r#move::Move,
};

#[pyclass]
pub struct RulesEngine;

#[pymethods]
impl RulesEngine {
    #[staticmethod]
    pub fn calculates_carrots(distance: usize) -> i32 {
        let distancce_i32: i32 = distance.try_into().unwrap();
        (distancce_i32 * (distancce_i32 + 1)) / 2
    }

    #[staticmethod]
    pub fn can_exchange_carrots(board: &Board, player: &Hare, count: i32) -> Result<bool, PyErr> {
        match board.get_field(player.position) {
            Some(f) => {
                Ok(f == Field::Carrots && (count == 10 || (count == -10 && player.carrots >= 10)))
            }
            None => Err(CannotEnterFieldError::new_err("Field not found")),
        }
    }

    #[staticmethod]
    pub fn can_eat_salad(board: &Board, player: &Hare) -> Result<bool, PyErr> {
        match board.get_field(player.position) {
            Some(Field::Salad) => Ok(!matches!(
                player.last_move,
                Some(Move {
                    action: Action::EatSalad(_)
                })
            )),
            Some(_) => Ok(false),
            None => Err(CannotEnterFieldError::new_err("Field not found")),
        }
    }

    #[staticmethod]
    pub fn can_advance_to(
        board: &Board,
        new_position: usize,
        player: &Hare,
        other_player: &Hare,
    ) -> Result<(), PyErr> {
        if new_position == 0 {
            return Err(CannotEnterFieldError::new_err("Cannot jump to position 0"));
        }

        let field = match board.get_field(new_position) {
            Some(f) => f,
            None => {
                return Err(CannotEnterFieldError::new_err("Field not found"));
            }
        };

        if field != Field::Goal && new_position == other_player.position {
            return Err(FieldOccupiedError::new_err("Field is occupied by opponent"));
        }

        match field {
            Field::Hedgehog => Err(HedgehogOnlyBackwardsError::new_err(
                "You cannot go on Hedgehog field forwards",
            )),
            Field::Salad => {
                if player.salads > 0 {
                    Ok(())
                } else {
                    Err(FieldOccupiedError::new_err("Field is occupied by opponent"))
                }
            }
            Field::Hare => {
                if !player.cards.is_empty() {
                    Ok(())
                } else {
                    Err(CardNotOwnedError::new_err("No card to play"))
                }
            }
            Field::Market => {
                if player.carrots >= 10 {
                    Ok(())
                } else {
                    Err(MissingCarrotsError::new_err("Not enough carrots"))
                }
            }
            Field::Goal => {
                if player.carrots <= 10 && player.salads == 0 {
                    Ok(())
                } else {
                    Err(GoalConditionsError::new_err(
                        "Too much carrots or/and salads",
                    ))
                }
            }
            _ => Ok(()),
        }
    }
}
