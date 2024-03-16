use pyo3::exceptions::PyValueError;
use pyo3::{pyclass, pymethods, PyErr};

#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
#[pyclass]
pub enum MoveMistake {
    NoActions,
    PushActionRequired,
    SandBankEnd,
    FirstActionAccelerate,
    MovementPointsLeft,
    MovementPointsMissing,
}

#[pymethods]
impl MoveMistake {
    #[must_use]
    pub fn message(&self) -> String {
        match self {
            Self::NoActions => "Der Zug enthält keine Aktionen".to_string(),
            Self::PushActionRequired =>
                "Wenn du auf einem gegnerischen Schiff landest, muss darauf eine Abdrängaktion folgen.".to_string(),
            Self::SandBankEnd =>
                "Zug auf eine Sandbank muss letzte Aktion sein.".to_string(),
            Self::FirstActionAccelerate =>
                "Du kannst nur in der ersten Aktion beschleunigen.".to_string(),
            Self::MovementPointsLeft => "Es sind noch Bewegungspunkte übrig.".to_string(),
            Self::MovementPointsMissing => "Nicht genug Bewegungspunkte.".to_string(),
        }
    }
}

impl From<MoveMistake> for PyErr {
    fn from(err: MoveMistake) -> Self {
        PyValueError::new_err(err.message())
    }
}
