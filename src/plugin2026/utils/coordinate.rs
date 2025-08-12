use pyo3::*;

use crate::plugin2026::{utils::vector::Vector};

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct Coordinate {
    #[pyo3(get, set)]
    pub x: isize,
    #[pyo3(get, set)]
    pub y: isize,
}

#[pymethods]
impl Coordinate {
    #[new]
    pub fn new(x: isize, y: isize) -> Self {
        Self {
            x, y
        }
    }

    pub fn __str__(&self) -> String {self.to_string()}
    pub fn __repr__(&self) -> String {format!("{:?}", self)}

    pub fn add_vector(&self, vector: &Vector) -> Coordinate {
        Coordinate {
            x: self.x + vector.delta_x,
            y: self.y + vector.delta_y
        }
    }

    pub fn add_vector_mut(&mut self, vector: &Vector) {
        self.x += vector.delta_x;
        self.y += vector.delta_y;
    }

    pub fn get_difference(&self, other: &Coordinate) -> Vector {
        Vector {
            delta_x: other.x - self.x,
            delta_y: other.y - self.y
        }
    }
}

impl std::fmt::Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}