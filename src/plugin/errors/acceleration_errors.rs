use pyo3::{pyclass, PyErr, pymethods};
use pyo3::exceptions::PyValueError;

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
impl  AccelerationProblem {
    pub fn message(&self) -> String {
        match self {
            AccelerationProblem::ZeroAcc => String::from("Es kann nicht um den Wert 0 beschleunigt werden."),
            AccelerationProblem::AboveMaxSpeed => {
                format!("Die maximale Geschwindigkeit von {} darf nicht überschritten werden.", PluginConstants::MAX_SPEED)
            }
            AccelerationProblem::BelowMinSpeed => {
                format!("Die minimale Geschwindigkeit von {} darf nicht unterschritten werden.", PluginConstants::MIN_SPEED)
            }
            AccelerationProblem::InsufficientCoal => String::from("Nicht genug Kohle für die Aktion vorhanden."),
            AccelerationProblem::OnSandbank => String::from("Auf einer Sandbank kann nicht beschleunigt werden."),
        }
    }
}

impl From<AccelerationProblem> for PyErr {
    fn from(err: AccelerationProblem) -> Self {
        PyValueError::new_err(err.message())
    }
}
