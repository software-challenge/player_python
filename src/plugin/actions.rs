use pyo3::{IntoPy, PyObject};
use pyo3::FromPyObject;
use pyo3::Python;

use crate::plugin::actions::push::Push;
use crate::plugin::actions::turn::Turn;

use self::{accelerate::Accelerate, advance::Advance};

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

