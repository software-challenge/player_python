use std::fmt::Debug;

use pyo3::*;

use crate::plugin2026::{
    utils::{
        coordinate::Coordinate,
        direction::Direction
    }
};

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Move {
    #[pyo3(get, set)]
    pub start: Coordinate,
    #[pyo3(get, set)]
    pub direction: Direction
}

#[pymethods]
impl Move {
    #[new]
    pub fn new(start: Coordinate, direction: Direction) -> Self {
        Self {
            start, direction
        }
    }

    pub fn __str__(&self) -> String {self.to_string()}
    pub fn __repr__(&self) -> String {format!("{:?}", self)}
    pub fn __eq__(&self, other: &Move) -> bool {self == other}
    pub fn __ne_(&self, other: &Move) -> bool {self != other}
    
    pub fn deepcopy(&self) -> Move {self.clone()}
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Move von {} in Richtung {}", self.start, self.direction)
    }
}
