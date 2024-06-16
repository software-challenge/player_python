use pyo3::*;

use crate::plugin::game_state::GameState;

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct EatSalad {}

#[pymethods]
impl EatSalad {
    pub fn perform(&self, state: &mut GameState) -> Result<(), PyErr> {
        let mut current = state.clone_current_player();
        current.eat_salad(state)?;
        Ok(())
    }
}
