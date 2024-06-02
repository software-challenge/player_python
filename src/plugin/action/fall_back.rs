use pyo3::*;

use crate::plugin::{ errors::CannotEnterFieldError, game_state::GameState };

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct FallBack {}

#[pymethods]
impl FallBack {
    pub fn perform(&self, state: &GameState) -> Result<(), PyErr> {
        let mut player = state.get_current();
        match state.get_fall_back(&player) {
            Some(i) => {
                player.carrots += 10 * ((player.position - i) as i32);
                player.position = i;
                Ok(())
            }
            None => Err(CannotEnterFieldError::new_err("Field not found")),
        }
    }
}
