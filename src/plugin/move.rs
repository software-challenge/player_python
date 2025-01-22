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
        let result = self.action.perform(state);
        if result.is_ok() {
            let mut player = state.clone_current_player();
            player.last_move = Some(self.clone());
            state.last_move = Some(self.clone());
            state.update_player(player);
        }
        result
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Move(action={:?})", self.action))
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Move(action={})", self.action)
    }
}
