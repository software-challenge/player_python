use pyo3::*;

use super::{action::Action, game_state::GameState};

#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
pub struct Move {
    #[pyo3(get, set)]
    pub action: Action,
}

#[pymethods]
impl Move {
    #[new]
    #[must_use]
    pub fn new(action: Action) -> Self {
        Self { action }
    }

    pub fn perform(&self, state: &mut GameState) -> Result<(), PyErr> {
        self.action.perform(state)
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Move(action={:?})", self.action))
    }
}
