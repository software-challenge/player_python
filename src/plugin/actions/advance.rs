use pyo3::prelude::*;

use crate::plugin::{constants::PluginConstants, errors::advance_errors::AdvanceProblem, game_state::GameState};
use crate::plugin::errors::ActionProblem;
use crate::plugin::field::FieldType;

#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
pub struct Advance {
    /// The number of fields to advance. A negative value means moving backward.
    pub distance: i32,
}


#[pymethods]
impl Advance {
    #[new]
    pub fn new(distance: i32) -> Self {
        Advance { distance }
    }

    pub fn perform(&self, state: &GameState) -> Result<GameState, ActionProblem> {
        let new_state: GameState = state.clone();

        if self.distance < PluginConstants::MIN_SPEED
            && new_state.board.get(&new_state.current_ship().position).unwrap().field_type != FieldType::Sandbank
            || self.distance > PluginConstants::MAX_SPEED
        {
            return Err(ActionProblem::AdvanceProblem(AdvanceProblem::InvalidDistance));
        }

        if self.distance > new_state.current_ship().movement {
            return Err(ActionProblem::AdvanceProblem(AdvanceProblem::MovementPointsMissing));
        }

        let result = new_state.check_advance_limit(
            &new_state.current_ship().position,
            &match self.distance > 0 {
                true => new_state.current_ship().direction,
                false => new_state.current_ship().direction.opposite(),
            },
            new_state.current_ship().movement,
        );

        if (result.len() as i32) < self.distance.abs() {
            return Err(ActionProblem::AdvanceProblem(AdvanceProblem::InvalidDistance));
        }


        new_state.current_ship().position += new_state.current_ship().direction.vector() * self.distance;
        new_state.current_ship().movement -= self.distance;
        Ok(new_state)
    }
}

impl std::fmt::Display for Advance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Gehe {} Felder {}", self.distance, if self.distance >= 0 { "vor" } else { "zurück" })
    }
}
