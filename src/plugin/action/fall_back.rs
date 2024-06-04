use pyo3::*;

use crate::plugin::{ errors::CannotEnterFieldError, game_state::GameState };

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct FallBack {}

#[pymethods]
impl FallBack {
    pub fn perform(&self, state: &mut GameState) -> Result<(), PyErr> {
        let mut current = state.get_current_player();
        match state.get_fall_back(&current) {
            Some(i) => {
                current.carrots += 10 * ((current.position - i) as i32);
                current.position = i;
                state.set_current_player(current);
                Ok(())
            }
            None => Err(CannotEnterFieldError::new_err("Field not found")),
        }
    }
}
