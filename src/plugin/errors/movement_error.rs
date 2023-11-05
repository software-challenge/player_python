use pyo3::{ pyclass, PyErr, pymethods };
use pyo3::exceptions::PyValueError;

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
    pub fn message(&self) -> String {
        match self {
            MoveMistake::NoActions => "Der Zug enthält keine Aktionen".to_string(),
            MoveMistake::PushActionRequired =>
                "Wenn du auf einem gegnerischen Schiff landest, muss darauf eine Abdrängaktion folgen.".to_string(),
            MoveMistake::SandBankEnd =>
                "Zug auf eine Sandbank muss letzte Aktion sein.".to_string(),
            MoveMistake::FirstActionAccelerate =>
                "Du kannst nur in der ersten Aktion beschleunigen.".to_string(),
            MoveMistake::MovementPointsLeft => "Es sind noch Bewegungspunkte übrig.".to_string(),
            MoveMistake::MovementPointsMissing => "Nicht genug Bewegungspunkte.".to_string(),
        }
    }
}

impl From<MoveMistake> for PyErr {
    fn from(err: MoveMistake) -> Self {
        PyValueError::new_err(err.message())
    }
}
