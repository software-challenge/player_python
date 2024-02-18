use log::debug;
use pyo3::exceptions::PyBaseException;
use pyo3::prelude::*;

use crate::plugin::{
    constants::PluginConstants,
    errors::advance_errors::AdvanceProblem,
    game_state::GameState,
};

use crate::plugin::ship::Ship;

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
        debug!("New Advance with distance: {}", distance);
        Advance { distance }
    }

    pub fn perform(&self, state: &GameState) -> Result<Ship, PyErr> {
        debug!("Performing advance with distance: {}", self.distance);
        let mut ship = state.current_ship.clone();
        let valid_distance = self.validate_distance(&state, &ship)?;
        let advance_info = state.calculate_advance_info(
            &ship.position,
            &ship.resolve_direction(!valid_distance),
            ship.movement
        );
        let advance_possible = (advance_info.distance() as i32) >= self.distance.abs();

        if !advance_possible {
            debug!("Advance problem: {}", advance_info.problem.message());
            return Err(PyBaseException::new_err(advance_info.problem.message()));
        }

        ship.update_position(self.distance, advance_info);
        debug!("Advance completed: {:?}", ship);
        Ok(ship)
    }

    fn validate_distance(&self, state: &GameState, ship: &Ship) -> Result<bool, PyErr> {
        if
            (self.distance < PluginConstants::MIN_SPEED &&
                !state.board.is_sandbank(&ship.position)) ||
            self.distance > PluginConstants::MAX_SPEED
        {
            debug!("Invalid distance: {}", self.distance);
            return Err(PyBaseException::new_err(AdvanceProblem::InvalidDistance.message()));
        }
        if self.distance > ship.movement {
            debug!(
                "Movement points missing: {} needed, {} available",
                self.distance,
                ship.movement
            );
            return Err(PyBaseException::new_err(AdvanceProblem::MovementPointsMissing.message()));
        }
        Ok(true)
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Advance({})", self.distance))
    }
}

#[cfg(test)]
mod tests {
    use pyo3::prepare_freethreaded_python;

    use crate::plugin::board::Board;
    use crate::plugin::coordinate::{ CubeCoordinates, CubeDirection };
    use crate::plugin::field::{ Field, FieldType };
    use crate::plugin::game_state::GameState;
    use crate::plugin::segment::Segment;
    use crate::plugin::ship::{ Ship, TeamEnum };

    use super::*;

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

        let result: Result<Ship, PyErr> = advance.perform(&state);

        assert!(result.is_ok());
        let new_ship: Ship = result.unwrap();
        assert_eq!(new_ship.position, CubeCoordinates::new(2, -1));
        assert_eq!(new_ship.movement, 3);
    }

    #[test]
    fn test_advance_perform_invalid_distance() {
        let advance: Advance = Advance::new(-2);
        let state: GameState = setup();

        let result: Result<Ship, PyErr> = advance.perform(&state);

        assert!(result.is_err());
        let error: PyErr = result.unwrap_err();

        prepare_freethreaded_python();
        Python::with_gil(|py| {
            assert_eq!(error.value(py).to_string(), AdvanceProblem::InvalidDistance.message());
        });
    }

    #[test]
    fn test_advance_perform_movement_points_missing() {
        let advance: Advance = Advance::new(6);
        let state: GameState = setup();

        let result: Result<Ship, PyErr> = advance.perform(&state);

        assert!(result.is_err());
        let error: PyErr = result.unwrap_err();

        prepare_freethreaded_python();
        Python::with_gil(|py| {
            assert_eq!(
                error.value(py).to_string(),
                AdvanceProblem::MovementPointsMissing.message()
            );
        });
    }
}
