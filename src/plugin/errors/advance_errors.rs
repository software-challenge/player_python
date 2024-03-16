use pyo3::exceptions::PyValueError;
use pyo3::{pyclass, pymethods, PyErr};

#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
#[pyclass]
pub enum AdvanceProblem {
    MovementPointsMissing,
    InsufficientPush,
    InvalidDistance,
    ShipAlreadyInTarget,
    FieldIsBlocked,
    MoveEndOnSandbank,
}

#[pymethods]
impl AdvanceProblem {
    #[must_use]
    pub fn message(&self) -> String {
        match self {
            Self::MovementPointsMissing => String::from("Nicht genug Bewegungspunkte."),
            Self::InsufficientPush => String::from(
                "Nicht genug Bewegungspunkte für notwendige nachfolgende Abdrängaktion.",
            ),
            Self::InvalidDistance => String::from("Zurückzulegende Distanz ist ungültig."),
            Self::ShipAlreadyInTarget => String::from("Kann nicht durch einen Gegner ziehen."),
            Self::FieldIsBlocked => String::from("Feld ist blockiert."),
            Self::MoveEndOnSandbank => {
                String::from("Zug sollte bereits enden, da auf Sandbank gefahren wurde.")
            }
        }
    }
}

impl From<AdvanceProblem> for PyErr {
    fn from(err: AdvanceProblem) -> Self {
        PyValueError::new_err(err.message())
    }
}
