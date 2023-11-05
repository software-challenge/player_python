use pyo3::{pyclass, PyErr, pymethods};
use pyo3::exceptions::PyValueError;

#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
#[pyclass]
pub enum InvalidMoveException {
    NoActions,
    PushActionRequired,
    SandBankEnd,
    FirstActionAccelerate,
    MovementPointsLeft,
    MovementPointsMissing,
}


#[pymethods]
impl InvalidMoveException {
    pub fn message(&self) -> String {
        match self {
            InvalidMoveException::NoActions => "Der Zug enthält keine Aktionen".to_string(),
            InvalidMoveException::PushActionRequired => "Wenn du auf einem gegnerischen Schiff landest, muss darauf eine Abdrängaktion folgen.".to_string(),
            InvalidMoveException::SandBankEnd => "Zug auf eine Sandbank muss letzte Aktion sein.".to_string(),
            InvalidMoveException::FirstActionAccelerate => "Du kannst nur in der ersten Aktion beschleunigen.".to_string(),
            InvalidMoveException::MovementPointsLeft => "Es sind noch Bewegungspunkte übrig.".to_string(),
            InvalidMoveException::MovementPointsMissing => "Nicht genug Bewegungspunkte.".to_string(),
        }
    }
}

impl From<InvalidMoveException> for PyErr {
    fn from(err: InvalidMoveException) -> Self {
        PyValueError::new_err(err.message())
    }
}