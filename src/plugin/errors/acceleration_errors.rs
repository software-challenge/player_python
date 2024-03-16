use pyo3::exceptions::PyValueError;
use pyo3::{pyclass, pymethods, PyErr};

use crate::plugin::constants::PluginConstants;

#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
#[pyclass]
pub enum AccelerationProblem {
    ZeroAcc,
    AboveMaxSpeed,
    BelowMinSpeed,
    InsufficientCoal,
    OnSandbank,
}

#[pymethods]
impl AccelerationProblem {
    #[must_use]
    pub fn message(&self) -> String {
        match self {
            Self::ZeroAcc => String::from("Es kann nicht um den Wert 0 beschleunigt werden."),
            Self::AboveMaxSpeed => {
                format!(
                    "Die maximale Geschwindigkeit von {} darf nicht überschritten werden.",
                    PluginConstants::MAX_SPEED
                )
            }
            Self::BelowMinSpeed => {
                format!(
                    "Die minimale Geschwindigkeit von {} darf nicht unterschritten werden.",
                    PluginConstants::MIN_SPEED
                )
            }
            Self::InsufficientCoal => String::from("Nicht genug Kohle für die Aktion vorhanden."),
            Self::OnSandbank => String::from("Auf einer Sandbank kann nicht beschleunigt werden."),
        }
    }
}

impl From<AccelerationProblem> for PyErr {
    fn from(err: AccelerationProblem) -> Self {
        PyValueError::new_err(err.message())
    }
}
