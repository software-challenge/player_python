use pyo3::exceptions::PyValueError;
use pyo3::{pyclass, pymethods, PyErr};

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
    #[must_use]
    pub fn message(&self) -> String {
        match self {
            Self::MovementPointsMissing => "Nicht genug Bewegungspunkte.".to_string(),
            Self::SameFieldPush => "Um einen Spieler abzudrängen muss man sich auf demselben Feld wie der Spieler befinden.".to_string(),
            Self::InvalidFieldPush => "Ein Spieler darf nicht auf ein nicht vorhandenes (oder nicht sichtbares) Feld abgedrängt werden.".to_string(),
            Self::BlockedFieldPush => "Ein Spieler darf nicht auf ein blockiertes Feld abgedrängt werden.".to_string(),
            Self::SandbankPush => "Von einer Sandbank ist abdrängen nicht möglich.".to_string(),
            Self::BackwardPushingRestricted => "Ein Spieler darf nicht auf das Feld abdrängen, von dem er kommt.".to_string(),
        }
    }
}

impl From<PushProblem> for PyErr {
    fn from(err: PushProblem) -> Self {
        PyValueError::new_err(err.message())
    }
}
