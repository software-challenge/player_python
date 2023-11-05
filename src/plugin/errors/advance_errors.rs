use pyo3::{pyclass, PyErr, pymethods};
use pyo3::exceptions::PyValueError;

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
    pub fn message(&self) -> String {
        match self {
            AdvanceProblem::MovementPointsMissing => String::from("Nicht genug Bewegungspunkte."),
            AdvanceProblem::InsufficientPush => String::from("Nicht genug Bewegungspunkte für notwendige nachfolgende Abdrängaktion."),
            AdvanceProblem::InvalidDistance => String::from("Zurückzulegende Distanz ist ungültig."),
            AdvanceProblem::ShipAlreadyInTarget => String::from("Kann nicht durch einen Gegner ziehen."),
            AdvanceProblem::FieldIsBlocked => String::from("Feld ist blockiert."),
            AdvanceProblem::MoveEndOnSandbank => String::from("Zug sollte bereits enden, da auf Sandbank gefahren wurde."),
        }
    }
}

impl From<AdvanceProblem> for PyErr {
    fn from(err: AdvanceProblem) -> Self {
        PyValueError::new_err(err.message())
    }
}
