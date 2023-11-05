use pyo3::prelude::*;

use crate::plugin::actions::Action;

#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
pub struct Move {
    #[pyo3(get, set)]
    pub actions: Vec<Action>,
}

#[pymethods]
impl Move {
    #[new]
    pub fn new(actions: Vec<Action>) -> Self {
        Move { actions }
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Move(actions={:?})", self.actions))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin::actions::{ Action, accelerate::Accelerate, advance::Advance, push::Push };

    #[test]
    fn test_new_move() {
        let actions = vec![
            Action::Accelerate(Accelerate::new(1)),
            Action::Advance(Advance::new(1)),
            Action::Push(Push::new(crate::plugin::coordinate::CubeDirection::DownLeft))
        ];
        let m = Move::new(actions.clone());
        assert_eq!(m.actions, actions);
    }
}
