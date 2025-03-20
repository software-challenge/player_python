use pyo3::*;

use super::{
    action::{card::Card, eat_salad::EatSalad, Action},
    board::Board,
    errors::HUIError,
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
                    return Err(HUIError::new_err("You can only exchange 10 carrots"));
                }
                if count == -10 && player.carrots < 10 {
                    return Err(HUIError::new_err("Not enough carrots"));
                }
                Ok(())
            }
            Some(_) => Err(HUIError::new_err("Field is not a carrot field")),
            None => Err(HUIError::new_err("Field not found")),
        }
    }

    #[staticmethod]
    pub fn can_eat_salad(board: &Board, player: &Hare) -> Result<(), PyErr> {
        if player.salads < 1 {
            return Err(HUIError::new_err("No salad to eat"));
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
            Some(Field::Salad) => Err(HUIError::new_err("Cannot eat salad twice in a row")),
            Some(_) => Err(HUIError::new_err("Field is not a salad")),
            None => Err(HUIError::new_err("Field not found")),
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
                    Err(HUIError::new_err("Cannot advance without eating salad"))
                } else {
                    Ok(())
                }
            }
            Some(_) => Ok(()),
            None => Ok(()),
        }
    }

    #[staticmethod]
    pub fn can_move_to(
        board: &Board,
        distance: isize,
        player: &Hare,
        other_player: &Hare,
        cards: Vec<Card>,
    ) -> Result<(), PyErr> {
        if distance == 0 {
            return Err(HUIError::new_err("Advance distance cannot be 0"));
        }

        let new_position = (player.position as isize + distance) as usize;

        if new_position == 0 {
            return Err(HUIError::new_err("Cannot jump to position 0"));
        }

        Self::has_to_eat_salad(board, player)?;

        let field = board
            .get_field(new_position)
            .ok_or_else(|| HUIError::new_err("Field not found"))?;

        if field != Field::Goal && new_position == other_player.position {
            return Err(HUIError::new_err("Field is occupied by opponent"));
        }

        let needed_carrots = RulesEngine::calculates_carrots(distance.try_into().unwrap());

        match field {
            Field::Hedgehog => Err(HUIError::new_err("Cannot advance on Hedgehog field")),
            Field::Salad if player.salads > 0 => Ok(()),
            Field::Salad => Err(HUIError::new_err("No salad to eat")),
            Field::Hare if !cards.is_empty() => Ok(()),
            Field::Hare => Err(HUIError::new_err("No card to play")),
            Field::Market if player.carrots >= 10 && !cards.is_empty() => Ok(()),
            Field::Market => Err(HUIError::new_err("Not enough carrots or no card to play")),
            Field::Goal if player.carrots - needed_carrots <= 10 && player.salads == 0 => Ok(()),
            Field::Goal => Err(HUIError::new_err("Too many carrots or salads")),
            _ => Ok(()),
        }
    }
}
