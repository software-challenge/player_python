use pyo3::{ IntoPy, PyErr, PyObject };
use pyo3::FromPyObject;
use pyo3::Python;

use crate::plugin::actions::push::Push;
use crate::plugin::actions::turn::Turn;

use self::{ accelerate::Accelerate, advance::Advance };

use super::game_state::GameState;
use super::ship::Ship;

pub mod accelerate;
pub mod advance;
pub mod push;
pub mod turn;

#[derive(PartialEq, PartialOrd, Eq, Hash, Clone, Debug, Copy, FromPyObject)]
pub enum Action {
    Accelerate(Accelerate),
    Advance(Advance),
    Push(Push),
    Turn(Turn),
}

impl IntoPy<PyObject> for Action {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            Action::Accelerate(accelerate) => accelerate.into_py(py),
            Action::Advance(advance) => advance.into_py(py),
            Action::Push(push) => push.into_py(py),
            Action::Turn(turn) => turn.into_py(py),
        }
    }
}

impl Action {
    pub fn perform(
        &self,
        game_state: &mut GameState
    ) -> Result<(Option<Ship>, Option<Ship>), PyErr> {
        match self {
            Action::Accelerate(accelerate) =>
                accelerate.perform(game_state).map(|ship| (Some(ship), None)),
            Action::Advance(advance) => advance.perform(game_state).map(|ship| (Some(ship), None)),
            Action::Push(push) =>
                push.perform(game_state).map(|(ship1, ship2)| (Some(ship1), Some(ship2))),
            Action::Turn(turn) => turn.perform(game_state).map(|ship| (Some(ship), None)),
        }
    }
}

impl From<Accelerate> for Action {
    fn from(acc: Accelerate) -> Self {
        Action::Accelerate(acc)
    }
}

impl From<Turn> for Action {
    fn from(turn: Turn) -> Self {
        Action::Turn(turn)
    }
}

impl From<Advance> for Action {
    fn from(advance: Advance) -> Self {
        Action::Advance(advance)
    }
}

impl From<Push> for Action {
    fn from(push: Push) -> Self {
        Action::Push(push)
    }
}
