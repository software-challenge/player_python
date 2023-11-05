use pyo3::{pyclass, PyErr, pymethods};
use pyo3::exceptions::PyValueError;

#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
#[pyclass]
pub enum TurnProblem {
    RotationOnSandbankNotAllowed,
    NotEnoughCoalForRotation,
    RotationOnNonExistingField,
}

#[pymethods]
impl TurnProblem {
    pub fn message(&self) -> String {
        match self {
            TurnProblem::RotationOnSandbankNotAllowed => "Drehung auf Sandbank nicht erlaubt.".to_string(),
            TurnProblem::NotEnoughCoalForRotation => "Nicht genug Kohle für Drehung.".to_string(),
            TurnProblem::RotationOnNonExistingField => "Auf einem inexistenten Feld ist keine Drehung möglich.".to_string(),
        }
    }
}

impl From<TurnProblem> for PyErr {
    fn from(err: TurnProblem) -> Self {
        PyValueError::new_err(err.message())
    }
}