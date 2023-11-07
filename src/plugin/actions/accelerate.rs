use pyo3::exceptions::PyBaseException;
use pyo3::prelude::*;

use crate::plugin::{ errors::acceleration_errors::AccelerationProblem, game_state::GameState };
use crate::plugin::field::FieldType;
use crate::plugin::ship::Ship;

/// `Accelerate` is representing a ship's ability to change its speed and acceleration.
/// It contains methods for initiating and managing the acceleration process.
///
/// The struct contains one field:
/// * `acc`: stores the magnitude of acceleration. A negative value indicates deceleration. This value cannot be 0.
///
/// # Methods
///
/// * `new()`: creates a new instance of the `Accelerate` object.
/// * `accelerate()`: performs the actual speed change.
/// * `perform()`: checks and manages different speed, acceleration conditions, and errors that might occur in the process.
///   It throws an error when acceleration(`acc`) is zero, or speed is above maximum or below minimum, if the ship is on Sandbank, or there is insufficient coal to maintain the acceleration.
///
/// Accelerate also implements the Display trait with `fmt()` function, enabling it to be represented as a string.
#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash, Copy)]
pub struct Accelerate {
    /// The magnitude of acceleration. A negative value means deceleration.
    /// Must not be 0.
    #[pyo3(get, set)]
    pub acc: i32,
}

#[pymethods]
impl Accelerate {
    #[new]
    pub fn new(acc: i32) -> Self {
        Self { acc }
    }

    pub fn perform(&self, state: &GameState) -> Result<Ship, PyErr> {
        let mut ship: Ship = state.current_ship.clone();
        let mut speed = ship.speed;
        speed += self.acc;

        match () {
            _ if self.acc == 0 =>
                Err(PyBaseException::new_err(AccelerationProblem::ZeroAcc.message())),
            _ if speed > 6 =>
                Err(PyBaseException::new_err(AccelerationProblem::AboveMaxSpeed.message())),
            _ if speed < 1 =>
                Err(PyBaseException::new_err(AccelerationProblem::BelowMinSpeed.message())),
            _ if state.board.get(&ship.position).unwrap().field_type == FieldType::Sandbank =>
                Err(PyBaseException::new_err(AccelerationProblem::OnSandbank.message())),
            _ => {
                let new_ship: Ship = self.accelerate(&mut ship);
                if new_ship.coal < 0 {
                    Err(PyBaseException::new_err(AccelerationProblem::InsufficientCoal.message()))
                } else {
                    Ok(ship)
                }
            }
        }
    }

    fn accelerate(&self,  ship: &mut Ship) -> Ship {
        let used_coal: i32 = self.acc.abs() - ship.free_acc;
        ship.coal -= used_coal.max(0);
        ship.free_acc = (-used_coal).max(0);
        ship.accelerate_by(self.acc);

        return ship.clone();
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Accelerate({})", self.acc))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin::board::Board;
    use crate::plugin::constants::PluginConstants;
    use crate::plugin::field::Field;
    use crate::plugin::game_state::GameState;
    use crate::plugin::coordinate::{ CubeCoordinates, CubeDirection };
    use crate::plugin::segment::Segment;
    use crate::plugin::ship::{ Ship, TeamEnum };

    #[test]
    fn test_new() {
        let acc = 2;
        let accelerate = Accelerate::new(acc);
        assert_eq!(accelerate.acc, acc);
    }

    fn setup(acc: i32) -> (Accelerate, GameState) {
        let accelerate: Accelerate = Accelerate::new(acc);
        let segment: Vec<Segment> = vec![Segment {
            direction: CubeDirection::Right,
            center: CubeCoordinates::new(0, 0),
            fields: vec![vec![Field::new(FieldType::Water, None); 4]; 5],
        }];
        let board: Board = Board::new(segment, CubeDirection::Right);
        let team_one: Ship = Ship::new(
            CubeCoordinates::new(0, -1),
            TeamEnum::One,
            None,
            None,
            None,
            None,
            None,
            None,
            None);
        let team_two: Ship = Ship::new(
            CubeCoordinates::new(-1, 1),
            TeamEnum::Two,
            None,
            None,
            None,
            None,
            None,
            None,
            None);
        let game_state: GameState = GameState::new(board, 0, team_one.clone(), team_two, None);
        (accelerate, game_state)
    }

    #[test]
    fn test_perform_zero_acc() {
        let (accelerate, game_state) = setup(0);

        let result_error: PyErr = accelerate.perform(&game_state).unwrap_err();

        Python::with_gil(|py| {
            assert_eq!(result_error.value(py).to_string(), AccelerationProblem::ZeroAcc.message());
        });
    }

    #[test]
    fn test_perform_above_max_speed() {
        let (accelerate, game_state) = setup(PluginConstants::MAX_SPEED + 1);

        let result_error = accelerate.perform(&game_state).unwrap_err();

        Python::with_gil(|py| {
            assert_eq!(
                result_error.value(py).to_string(),
                AccelerationProblem::AboveMaxSpeed.message()
            );
        });
    }

    #[test]
    fn test_perform_below_min_speed() {
        let (accelerate, game_state) = setup(PluginConstants::MIN_SPEED - 2);

        let result = accelerate.perform(&game_state).unwrap_err();

        Python::with_gil(|py| {
            assert_eq!(result.value(py).to_string(), AccelerationProblem::BelowMinSpeed.message());
        });
    }

    #[test]
    fn test_perform_insufficient_coal() {
        let (accelerate, game_state) = setup(2);

        let mute_state: &mut GameState = &mut game_state.clone();
        mute_state.current_ship.coal = 0;
        mute_state.other_ship.coal = 0;

        let result = accelerate.perform(&mute_state).unwrap_err();

        Python::with_gil(|py| {
            assert_eq!(
                result.value(py).to_string(),
                AccelerationProblem::InsufficientCoal.message()
            );
        });
    }

    #[test]
    fn test_perform_success() {
        let (accelerate, game_state) = setup(2);

        let result: Ship = accelerate.perform(&game_state).unwrap();
        assert_eq!(result.speed, 3);
    }

    #[test]
    fn test_accelerate() {
        let acc: i32 = 2;

        let ship: Ship = Ship::new(
            CubeCoordinates::new(0, 0),
            TeamEnum::One,
            None,
            None,
            None,
            None,
            None,
            None,
            None);

        assert_eq!(ship.speed, PluginConstants::MIN_SPEED);
        assert_eq!(ship.coal, PluginConstants::START_COAL);
        assert_eq!(ship.free_acc, PluginConstants::FREE_ACC);

        let accelerate: Accelerate = Accelerate::new(acc);
        let new_ship: Ship = accelerate.accelerate(&mut ship.clone());

        assert_eq!(new_ship.speed, PluginConstants::MIN_SPEED + 2);
        assert_eq!(new_ship.coal, PluginConstants::START_COAL - 1);
        assert_eq!(new_ship.free_acc, PluginConstants::FREE_ACC - 1);
    }
}
