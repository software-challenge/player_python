use pyo3::{pyclass, PyErr, pymethods};
use pyo3::exceptions::PyValueError;

#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
#[pyclass]
pub enum PushProblem {
    MovementPointsMissing,
    SameFieldPush,
    InvalidFieldPush,
    BlockedFieldPush,
    SandbankPush,
    BackwardPushingRestricted,
}

#[pymethods]
impl PushProblem {
    pub fn message(&self) -> String {
        match self {
            PushProblem::MovementPointsMissing => "Nicht genug Bewegungspunkte.".to_string(),
            PushProblem::SameFieldPush => "Um einen Spieler abzudrängen muss man sich auf demselben Feld wie der Spieler befinden.".to_string(),
            PushProblem::InvalidFieldPush => "Ein Spieler darf nicht auf ein nicht vorhandenes (oder nicht sichtbares) Feld abgedrängt werden.".to_string(),
            PushProblem::BlockedFieldPush => "Ein Spieler darf nicht auf ein blockiertes Feld abgedrängt werden.".to_string(),
            PushProblem::SandbankPush => "Von einer Sandbank ist abdrängen nicht möglich.".to_string(),
            PushProblem::BackwardPushingRestricted => "Ein Spieler darf nicht auf das Feld abdrängen, von dem er kommt.".to_string(),
        }
    }
}

impl From<PushProblem> for PyErr {
    fn from(err: PushProblem) -> Self {
        PyValueError::new_err(err.message())
    }
}
