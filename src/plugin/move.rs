use pyo3::pyclass;

use crate::plugin::actions::Action;

#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
pub struct Move {
    #[pyo3(get, set)]
    pub actions: Vec<Action>,
}

impl Move {
    pub fn new(actions: Vec<Action>) -> Self {
        Move { actions }
    }
}
