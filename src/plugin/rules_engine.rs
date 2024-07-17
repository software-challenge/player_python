use pyo3::*;

use crate::plugin::errors::{
    CardNotOwnedError, FieldOccupiedError, GoalConditionsError, HedgehogOnlyBackwardsError,
    MissingCarrotsError,
};

use super::{
    action::{eat_salad::EatSalad, Action},
    board::Board,
    errors::{CannotEnterFieldError, NoSaladError},
    field::Field,
    hare::Hare,
    r#move::Move,
};

#[pyclass]
pub struct RulesEngine;

#[pymethods]
impl RulesEngine {
    #[staticmethod]
    pub fn calculates_carrots(distance: usize) -> i32 {
        let distance_i32: i32 = distance.try_into().unwrap();
        (distance_i32 * (distance_i32 + 1)) / 2
    }

    #[staticmethod]
    pub fn can_exchange_carrots(board: &Board, player: &Hare, count: i32) -> Result<(), PyErr> {
        match board.get_field(player.position) {
            Some(Field::Carrots) => {
                if count != 10 && count != -10 {
                    return Err(MissingCarrotsError::new_err(
                        "You can only exchange 10 carrots",
                    ));
                }
                if count == -10 && player.carrots < 10 {
                    return Err(MissingCarrotsError::new_err("Not enough carrots"));
                }
                Ok(())
            }
            Some(_) => Err(CannotEnterFieldError::new_err(
                "Field is not a carrot field",
            )),
            None => Err(CannotEnterFieldError::new_err("Field not found")),
        }
    }

    #[staticmethod]
    pub fn can_eat_salad(board: &Board, player: &Hare) -> Result<(), PyErr> {
        if player.salads < 1 {
            return Err(NoSaladError::new_err("No salad to eat"));
        }

        match board.get_field(player.position) {
            Some(Field::Salad)
                if !matches!(
                    player.last_move,
                    Some(Move {
                        action: Action::EatSalad(_)
                    })
                ) =>
            {
                Ok(())
            }
            Some(Field::Salad) => Err(CannotEnterFieldError::new_err(
                "Cannot eat salad twice in a row",
            )),
            Some(_) => Err(CannotEnterFieldError::new_err("Field is not a salad")),
            None => Err(CannotEnterFieldError::new_err("Field not found")),
        }
    }

    #[staticmethod]
    pub fn has_to_eat_salad(board: &Board, player: &Hare) -> Result<(), PyErr> {
        match board.get_field(player.position) {
            Some(Field::Salad) => {
                if player.last_move
                    != Some(Move {
                        action: Action::EatSalad(EatSalad::new()),
                    })
                {
                    Err(CannotEnterFieldError::new_err(
                        "Cannot advance without eating salad",
                    ))
                } else {
                    Ok(())
                }
            }
            Some(_) => Ok(()),
            None => Ok(()),
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

        Self::has_to_eat_salad(board, player)?;

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
