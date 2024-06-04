use std::mem::swap;

use pyo3::*;

use crate::plugin::{
    errors::{ CannotEnterFieldError, CannotPlayCardError, CardNotOwnedError },
    field::Field,
    game_state::GameState,
};

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

    fn play(&self, state: &mut GameState) -> Result<(), PyErr> {
        let mut current = state.get_current_player();
        let mut other = state.get_other_player(&current);
        match self {
            Card::FallBack => state.move_to_field(&mut current, other.position - 1)?,
            Card::HurryAhead => state.move_to_field(&mut current, other.position + 1)?,
            Card::EatSalad => state.get_current_player().eat_salad()?,
            Card::SwapCarrots => swap(&mut current.carrots, &mut other.carrots),
        }
        state.set_current_player(current);
        state.set_other_player(other);
        Ok(())
    }

    pub fn perform(&self, state: &mut GameState) -> Result<(), PyErr> {
        let mut current = state.get_current_player();

        let field = state.board
            .get_field(current.position)
            .ok_or_else(|| CannotEnterFieldError::new_err("Field not found"))?;

        if field != Field::Hare {
            return Err(CannotPlayCardError::new_err(""));
        }

        let index = current.cards
            .iter()
            .position(|card| card == self)
            .ok_or_else(|| CardNotOwnedError::new_err(""))?;

        current.cards.remove(index);
        state.set_current_player(current);

        self.play(state)?;

        Ok(())
    }
}
