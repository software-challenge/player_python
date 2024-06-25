use pyo3::*;

use crate::plugin::game_state::GameState;

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Default)]
pub struct FallBack {}

#[pymethods]
impl FallBack {
    #[new]
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }

    pub fn perform(&self, state: &mut GameState) -> Result<(), PyErr> {
        let mut current = state.clone_current_player();
        current.fall_back(state)?;
        Ok(())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}

impl std::fmt::Display for FallBack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FallBack")
    }
}
