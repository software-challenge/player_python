use pyo3::*;

use super::actions::Action;

#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
pub struct Move {
    #[pyo3(get, set)]
    pub actions: Vec<Action>,
}

#[pymethods]
impl Move {
    #[new]
    #[must_use]
    pub fn new(actions: Vec<Action>) -> Self {
        Self { actions }
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Move(actions={:?})", self.actions))
    }
}
