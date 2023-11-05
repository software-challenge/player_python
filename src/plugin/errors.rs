use pyo3::{ IntoPy, PyAny, PyErr, PyObject };
use pyo3::FromPyObject;
use pyo3::Python;
use crate::plugin::errors::invalid_move_exception::InvalidMoveException;

use crate::plugin::errors::movement_error::MoveMistake;
use crate::plugin::errors::push_error::PushProblem;
use crate::plugin::errors::turn_error::TurnProblem;

use self::{ acceleration_errors::AccelerationProblem, advance_errors::AdvanceProblem };

pub mod acceleration_errors;
pub mod advance_errors;
pub mod invalid_move_exception;
pub mod movement_error;
pub mod push_error;
pub mod turn_error;

#[derive(FromPyObject, PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
pub enum ActionProblem {
    MoveMistake(MoveMistake),
    InvalidMoveException(InvalidMoveException),
    AccelerationProblem(AccelerationProblem),
    AdvanceProblem(AdvanceProblem),
    PushProblem(PushProblem),
    TurnProblem(TurnProblem),
}

impl IntoPy<PyObject> for ActionProblem {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            ActionProblem::MoveMistake(accelerate) => accelerate.into_py(py),
            ActionProblem::InvalidMoveException(invalid_move) => invalid_move.into_py(py),
            ActionProblem::AccelerationProblem(acc) => acc.into_py(py),
            ActionProblem::AdvanceProblem(adv) => adv.into_py(py),
            ActionProblem::PushProblem(push) => push.into_py(py),
            ActionProblem::TurnProblem(turn) => turn.into_py(py),
        }
    }
}

impl From<ActionProblem> for PyErr {
    fn from(error: ActionProblem) -> Self {
        PyErr::new::<PyAny, _>(error)
    }
}

impl std::fmt::Display for ActionProblem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ActionProblem::MoveMistake(accelerate) => write!(f, "{}", accelerate.message()),
            ActionProblem::InvalidMoveException(invalid_move) =>
                write!(f, "{}", invalid_move.message()),
            ActionProblem::AccelerationProblem(acc) => write!(f, "{}", acc.message()),
            ActionProblem::AdvanceProblem(adv) => write!(f, "{}", adv.message()),
            ActionProblem::PushProblem(push) => write!(f, "{}", push.message()),
            ActionProblem::TurnProblem(turn) => write!(f, "{}", turn.message()),
        }
    }
}
