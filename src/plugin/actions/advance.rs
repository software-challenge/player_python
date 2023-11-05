use pyo3::exceptions::PyBaseException;
use pyo3::prelude::*;

use log::{ error, debug };

use crate::plugin::ship::{ TeamEnum, Ship };
use crate::plugin::{
    constants::PluginConstants,
    errors::advance_errors::AdvanceProblem,
    game_state::GameState,
};
use crate::plugin::field::FieldType;

#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash, Copy)]
pub struct Advance {
    /// The number of fields to advance. A negative value means moving backward.
    #[pyo3(get, set)]
    pub distance: i32,
}

#[pymethods]
impl Advance {
    #[new]
    pub fn new(distance: i32) -> Self {
        Advance { distance }
    }

    pub fn perform(&self, state: &GameState) -> Result<GameState, PyErr> {
        debug!("Performing advance action");
        if
            (self.distance < PluginConstants::MIN_SPEED &&
                state.board.get(&state.current_ship().position).unwrap().field_type !=
                    FieldType::Sandbank) ||
            self.distance > PluginConstants::MAX_SPEED
        {
            error!(
                "Invalid distance: {}. Due to low speed, high speed or sandbank.",
                self.distance
            );
            return Err(PyBaseException::new_err(AdvanceProblem::InvalidDistance.message()));
        }

        if self.distance > state.current_ship().movement {
            error!("Movement points missing: {}", self.distance);
            return Err(PyBaseException::new_err(AdvanceProblem::MovementPointsMissing.message()));
        }

        let result: Vec<Advance> = state.check_advance_limit(
            &state.current_ship().position,
            &(match self.distance > 0 {
                true => state.current_ship().direction,
                false => state.current_ship().direction.opposite(),
            }),
            state.current_ship().movement
        );

        if (result.len() as i32) < self.distance.abs() {
            error!(
                "Invalid distance: {}. Due to blocked field after {} fields.",
                self.distance,
                result.len()
            );
            return Err(PyBaseException::new_err(AdvanceProblem::InvalidDistance.message()));
        }
        let new_ship: &mut Ship = &mut state.current_ship().clone();
        new_ship.position += new_ship.direction.vector() * self.distance;
        new_ship.movement -= result[(self.distance.abs() as usize) - 1].distance;

        let new_state: &mut GameState = &mut state.clone();

        match new_ship.team {
            TeamEnum::One => {
                new_state.team_one = new_ship.clone();
            }
            TeamEnum::Two => {
                new_state.team_two = new_ship.clone();
            }
        }

        debug!("Advance action performed with new ship: {:?}", new_ship);

        Ok(new_state.clone())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Advance({})", self.distance))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin::board::Board;
    use crate::plugin::field::Field;
    use crate::plugin::game_state::GameState;
    use crate::plugin::coordinate::{ CubeCoordinates, CubeDirection };
    use crate::plugin::segment::Segment;
    use crate::plugin::ship::{ Ship, TeamEnum };

    #[test]
    fn test_new_advance() {
        let advance: Advance = Advance::new(5);
        assert_eq!(advance.distance, 5);
    }

    fn setup() -> GameState {
        let segment: Vec<Segment> = vec![Segment {
            direction: CubeDirection::Right,
            center: CubeCoordinates::new(0, 0),
            fields: vec![vec![Field::new(FieldType::Water, None); 4]; 5],
        }];
        let board: Board = Board::new(segment, CubeDirection::Right);
        let team_one: &mut Ship = &mut Ship::new(
            CubeCoordinates::new(0, -1),
            TeamEnum::One,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None
        );
        team_one.speed = 5;
        team_one.movement = 5;
        let team_two: &mut Ship = &mut Ship::new(
            CubeCoordinates::new(-1, 1),
            TeamEnum::Two,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None
        );
        team_two.speed = 5;
        team_two.movement = 5;
        let game_state: GameState = GameState::new(
            board,
            0,
            team_one.clone(),
            team_two.clone(),
            None
        );
        game_state
    }

    #[test]
    fn test_advance_perform_valid() {
        let advance: Advance = Advance::new(2);
        let state: GameState = setup();

        let result: Result<GameState, PyErr> = advance.perform(&state);

        assert!(result.is_ok());
        let new_state: GameState = result.unwrap();
        assert_eq!(new_state.current_ship().position, CubeCoordinates::new(2, -1));
        assert_eq!(new_state.current_ship().movement, 3);
    }

    #[test]
    fn test_advance_perform_invalid_distance() {
        let advance: Advance = Advance::new(4);
        let state: GameState = setup();

        let result: Result<GameState, PyErr> = advance.perform(&state);

        assert!(result.is_err());
        let error: PyErr = result.unwrap_err();

        Python::with_gil(|py| {
            assert_eq!(error.value(py).to_string(), AdvanceProblem::InvalidDistance.message());
        });
    }

    #[test]
    fn test_advance_perform_movement_points_missing() {
        let advance: Advance = Advance::new(6);
        let state: GameState = setup();

        let result: Result<GameState, PyErr> = advance.perform(&state);

        assert!(result.is_err());
        let error: PyErr = result.unwrap_err();

        Python::with_gil(|py| {
            assert_eq!(
                error.value(py).to_string(),
                AdvanceProblem::MovementPointsMissing.message()
            );
        });
    }
}
