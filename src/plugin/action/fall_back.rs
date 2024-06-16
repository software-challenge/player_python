use pyo3::*;

use crate::plugin::game_state::GameState;

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct FallBack {}

#[pymethods]
impl FallBack {
    pub fn perform(&self, state: &mut GameState) -> Result<(), PyErr> {
        let mut current = state.clone_current_player();
        current.fall_back(state)?;
        Ok(())
    }
}
