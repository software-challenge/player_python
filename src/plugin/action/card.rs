use std::mem::swap;

use pyo3::*;

use crate::plugin::{
    errors::{CannotEnterFieldError, CannotPlayCardError, CardNotOwnedError},
    field::Field,
    game_state::GameState,
    hare::Hare,
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

    fn play(
        &self,
        state: &mut GameState,
        current: &mut Hare,
        other: &mut Hare,
    ) -> Result<(), PyErr> {
        match self {
            Card::FallBack => current.move_to_field(state, other.position - 1)?,
            Card::HurryAhead => current.move_to_field(state, other.position + 1)?,
            Card::EatSalad => current.eat_salad(state)?,
            Card::SwapCarrots => swap(&mut current.carrots, &mut other.carrots),
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
