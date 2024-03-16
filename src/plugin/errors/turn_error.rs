use pyo3::exceptions::PyValueError;
use pyo3::{pyclass, pymethods, PyErr};

#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
#[pyclass]
pub enum TurnProblem {
    RotationOnSandbankNotAllowed,
    NotEnoughCoalForRotation,
    RotationOnNonExistingField,
}

#[pymethods]
impl TurnProblem {
    #[must_use]
    pub fn message(&self) -> String {
        match self {
            Self::RotationOnSandbankNotAllowed => "Drehung auf Sandbank nicht erlaubt.".to_string(),
            Self::NotEnoughCoalForRotation => "Nicht genug Kohle für Drehung.".to_string(),
            Self::RotationOnNonExistingField => {
                "Auf einem inexistenten Feld ist keine Drehung möglich.".to_string()
            }
        }
    }
}

impl From<TurnProblem> for PyErr {
    fn from(err: TurnProblem) -> Self {
        PyValueError::new_err(err.message())
    }
}
