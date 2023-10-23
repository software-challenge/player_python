use pyo3::{pyclass, pymethods};

use crate::plugin::{errors::acceleration_errors::AccelerationProblem, game_state::GameState};
use crate::plugin::errors::ActionProblem;
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
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
pub struct Accelerate {
    /// The magnitude of acceleration. A negative value means deceleration.
    /// Must not be 0.
    #[pyo3(get, set)]
    acc: i32,
}

#[pymethods]
impl Accelerate {
    #[new]
    pub fn new(acc: i32) -> Self {
        Self { acc }
    }

    pub fn perform(&self, state: &GameState) -> Result<GameState, ActionProblem> {
        let new_state: GameState = state.clone();
        let mut ship: Ship = new_state.current_ship();
        let mut speed = ship.speed;
        speed += self.acc;

        match () {
            _ if self.acc == 0 => Err(ActionProblem::AccelerationProblem(AccelerationProblem::ZeroAcc)),
            _ if speed > 6 => Err(ActionProblem::AccelerationProblem(AccelerationProblem::AboveMaxSpeed)),
            _ if speed < 1 => Err(ActionProblem::AccelerationProblem(AccelerationProblem::BelowMinSpeed)),
            _ if new_state.board.get(&ship.position).unwrap().field_type == FieldType::Sandbank => Err(ActionProblem::AccelerationProblem(AccelerationProblem::OnSandbank)),
            _ => {
                self.accelerate(&mut ship);
                if ship.coal < 0 {
                    Err(ActionProblem::AccelerationProblem(AccelerationProblem::InsufficientCoal))
                } else {
                    Ok(new_state)
                }
            }
        }
    }

    fn accelerate(&self, ship: &mut Ship) {
        let used_coal = self.acc.abs() - ship.free_acc;
        ship.coal -= used_coal.max(0);
        ship.free_acc = (-used_coal).max(0);
        ship.accelerate_by(self.acc);
    }
}

impl std::fmt::Display for Accelerate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Beschleunige um {}", self.acc)
    }
}
