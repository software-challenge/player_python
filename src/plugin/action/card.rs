use std::mem::swap;

use pyo3::*;

use crate::plugin::{
    errors::{CannotEnterFieldError, CannotPlayCardError, CardNotOwnedError},
    field::Field,
    game_state::GameState,
    hare::Hare,
};

use super::Action;

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Copy)]
pub enum Card {
    FallBack,
    HurryAhead,
    EatSalad,
    SwapCarrots,
}

#[pymethods]
impl Card {
    pub fn moves(&self) -> bool {
        match self {
            Card::FallBack | Card::HurryAhead => true,
            Card::EatSalad | Card::SwapCarrots => false,
        }
    }

    fn play(
        &self,
        state: &mut GameState,
        current: &mut Hare,
        other: &mut Hare,
    ) -> Result<(), PyErr> {
        match self {
            Card::FallBack => {
                if current.position < other.position {
                    return Err(CannotPlayCardError::new_err(
                        "You can only play this card if you are ahead of the other player",
                    ));
                }
                current.move_to_field(state, other.position - 1)?;
            }
            Card::HurryAhead => {
                if current.position > other.position {
                    return Err(CannotPlayCardError::new_err(
                        "You can only play this card if you are behind the other player",
                    ));
                }
                current.move_to_field(state, other.position + 1)?;
            }
            Card::EatSalad => current.eat_salad(state)?,
            Card::SwapCarrots => {
                let last_lettuce_position = state
                    .board
                    .get_previous_field(Field::Salad, state.board.track.len() - 1)
                    .ok_or_else(|| {
                        CannotPlayCardError::new_err(
                            "Unable to find the last lettuce field position",
                        )
                    })?;

                if current.position < last_lettuce_position {
                    return Err(CannotPlayCardError::new_err(
                    "You can only play this card if you are standing in front of the last lettuce field",
                ));
                }

                if let (Some(current_last_move), Some(other_last_move)) =
                    (&current.last_move, &other.last_move)
                {
                    if let (Action::Advance(current_advance), Action::Advance(other_advance)) =
                        (&current_last_move.action, &other_last_move.action)
                    {
                        if current_advance.cards.contains(&Card::SwapCarrots)
                            && other_advance.cards.contains(&Card::SwapCarrots)
                        {
                            return Err(CannotPlayCardError::new_err(
                                "You can only play this card if the last similar swap card was not used in one of the last two turns",
                            ));
                        }
                    }
                }
                swap(&mut current.carrots, &mut other.carrots);
            }
        }
        Ok(())
    }

    pub fn perform(&self, state: &mut GameState) -> Result<(), PyErr> {
        let mut current = state.clone_current_player();
        let mut other = state.clone_other_player();

        let field = state
            .board
            .get_field(current.position)
            .ok_or_else(|| CannotEnterFieldError::new_err("Field not found"))?;

        if field != Field::Hare {
            return Err(CannotPlayCardError::new_err(
                "You can only play cards on the hare field",
            ));
        }

        let index = current
            .cards
            .iter()
            .position(|card| card == self)
            .ok_or_else(|| CardNotOwnedError::new_err(""))?;

        self.play(state, &mut current, &mut other)?;

        current.cards.remove(index);

        state.update_player(current);
        state.update_player(other);

        Ok(())
    }
}
