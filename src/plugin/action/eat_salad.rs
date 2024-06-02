use pyo3::*;

use crate::plugin::{ errors::CannotEatSaladError, game_state::GameState };

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct EatSalad {}

#[pymethods]
impl EatSalad {
    pub fn perform(&self, state: &GameState) -> Result<(), PyErr> {
        if state.must_eat_salad(&state.get_current())? {
            state.get_current().salad_eaten = true;
            state.eat_salad(&mut state.get_current())?;
            return Ok(());
        }

        Err(CannotEatSaladError::new_err("Cannot eat salad"))
    }
}
