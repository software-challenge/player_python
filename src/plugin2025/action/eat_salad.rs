use pyo3::*;

use crate::plugin2025::{game_state::GameState, rules_engine::RulesEngine};

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Default)]
pub struct EatSalad {}

#[pymethods]
impl EatSalad {
    #[new]
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }

    pub fn perform(&self, state: &mut GameState) -> Result<(), PyErr> {
        let mut current = state.clone_current_player();
        RulesEngine::can_eat_salad(&state.board, &current)?;
        current.eat_salad(state)?;
        Ok(())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}

impl std::fmt::Display for EatSalad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EatSalad")
    }
}
