use pyo3::prelude::*;

use crate::plugin::coordinate::CubeDirection;
use crate::plugin::errors::ActionProblem;
use crate::plugin::errors::push_error::PushProblem;
use crate::plugin::field::FieldType;
use crate::plugin::game_state::GameState;

#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
pub struct Push {
    direction: CubeDirection,
}

#[pymethods]
impl Push {
    #[new]
    pub fn new(direction: CubeDirection) -> Self {
        Push { direction }
    }

    pub fn perform(&self, state: &GameState) -> Result<GameState, ActionProblem> {
        let new_state: GameState = state.clone();

        if new_state.current_ship().movement == 0 {
            return Err(ActionProblem::PushProblem(PushProblem::MovementPointsMissing));
        }

        let push_from = new_state.current_ship().position;
        let push_from_copy = push_from.clone();
        let push_to = push_from + self.direction.vector();

        let shift_to_field = match new_state.board.get(&push_to) {
            Some(value) => value,
            None => return Err(ActionProblem::PushProblem(PushProblem::InvalidFieldPush)),
        };

        if !shift_to_field.is_empty() {
            return Err(ActionProblem::PushProblem(PushProblem::BlockedFieldPush));
        }

        let mut other_ship = new_state.other_ship().clone(); // clone the other ship
        if push_from_copy != other_ship.position {
            return Err(ActionProblem::PushProblem(PushProblem::SameFieldPush));
        }

        if new_state.board.get(&push_from_copy).unwrap().field_type == FieldType::Sandbank {
            return Err(ActionProblem::PushProblem(PushProblem::SandbankPush));
        }

        if self.direction == state.current_ship().direction.opposite() {
            return Err(ActionProblem::PushProblem(PushProblem::BackwardPushingRestricted));
        }

        if shift_to_field.field_type == FieldType::Sandbank {
            other_ship.speed = 1;
            other_ship.movement = 1;
        }

        other_ship.position = push_to;
        other_ship.free_turns += 1;

        if let Some(ship_mutex) = new_state.current_ship().opponent {
            *ship_mutex.lock().unwrap() = other_ship;
        } else {
            panic!("No other ship found");
        }

        Ok(new_state)
    }
}

impl std::fmt::Display for Push {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dränge nach {} ab", self.direction)
    }
}
