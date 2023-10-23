use pyo3::prelude::*;

use crate::plugin::coordinate::CubeDirection;
use crate::plugin::errors::ActionProblem;
use crate::plugin::errors::turn_error::TurnProblem;
use crate::plugin::field::FieldType;
use crate::plugin::game_state::GameState;
use crate::plugin::ship::Ship;

#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
pub struct Turn {
    direction: CubeDirection,
}

#[pymethods]
impl Turn {
    #[new]
    pub fn new(direction: CubeDirection) -> Self {
        Turn { direction }
    }

    pub fn perform(&self, state: &GameState) -> Result<GameState, ActionProblem> {
        let new_state: GameState = state.clone();
        let turn_count = new_state.current_ship().direction.turn_count_to(self.direction.clone());

        let abs_turn_count = turn_count.abs();
        let used_coal = abs_turn_count - new_state.current_ship().free_turns;

        new_state.current_ship().free_turns = std::cmp::max(new_state.current_ship().free_turns - abs_turn_count, 0);

        if new_state.board.get(&new_state.current_ship().position).unwrap().field_type == FieldType::Sandbank {
            return Err(ActionProblem::TurnProblem(TurnProblem::RotationOnSandbankNotAllowed));
        }
        if new_state.current_ship().coal < used_coal {
            return Err(ActionProblem::TurnProblem(TurnProblem::NotEnoughCoalForRotation));
        }
        if used_coal > 0 {
            new_state.current_ship().coal -= used_coal
        }

        new_state.current_ship().direction = self.direction.clone();
        Ok(new_state)
    }

    pub fn coal_cost(&self, ship: &Ship) -> i32 {
        self.direction.turn_count_to(self.direction.clone()).abs().saturating_sub(ship.free_turns)
    }
}

impl ToString for Turn {
    fn to_string(&self) -> String {
        format!("Drehe nach {:?}", self.direction)
    }
}