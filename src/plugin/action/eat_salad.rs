use pyo3::*;

use crate::plugin::{ errors::CannotEatSaladError, game_state::GameState };

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct EatSalad {}

#[pymethods]
impl EatSalad {
    pub fn perform(&self, state: &mut GameState) -> Result<(), PyErr> {
        let mut current = state.get_current_player();
        if state.must_eat_salad(&state.get_current_player())? {
            current.salad_eaten = true;
            state.eat_salad(&mut state.get_current_player())?;
            state.set_current_player(current);
            return Ok(());
        }

        Err(CannotEatSaladError::new_err("Cannot eat salad"))
    }
}
