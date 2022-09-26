use pyo3::prelude::*;
use crate::plugins::penguins::vector::Vector;

#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash, Copy)]
pub struct HexCoordinate {
    #[pyo3(get, set)]
    pub x: i32,
    #[pyo3(get, set)]
    pub y: i32,
}

#[pymethods]
impl HexCoordinate {
    #[new]
    pub fn new(x: i32, y: i32) -> Self {
        HexCoordinate { x, y }
    }

    pub fn to_cartesian(&self) -> CartesianCoordinate {
        CartesianCoordinate {
            x: self.x / 2 - (if self.y % 2 == 1 {1} else {0}),
            y: self.y,
        }
    }

    pub fn to_vector(&self) -> Vector {
        Vector {
            x: self.x,
            y: self.y,
        }
    }

    pub fn add_vector(&self, vector: &Vector) -> HexCoordinate {
        let vector: Vector = self.to_vector().add(vector);
        HexCoordinate {
            x: vector.x,
            y: vector.y,
        }
    }

    pub fn subtract_vector(&self, vector: &Vector) -> HexCoordinate {
        let vector: Vector = self.to_vector().sub(vector);
        HexCoordinate {
            x: vector.x,
            y: vector.y,
        }
    }

    pub fn get_neighbours(&self) -> Vec<HexCoordinate> {
        self.to_vector().neighbours().iter().map(|vector| HexCoordinate {
            x: vector.x,
            y: vector.y,
        }).collect()
    }

    pub fn distance(&self, other: &HexCoordinate) -> i32 {
        let vector: Vector = self.to_vector().sub(&other.to_vector());
        vector.magnitude() as i32
    }

    fn __repr__(&self) -> String {
        format!("HexCoordinate(x={}, y={})", self.x, self.y)
    }
}

impl std::fmt::Display for HexCoordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "HexCoordinate(x={}, y={})", self.x, self.y)
    }
}

#[pyclass]
pub struct CartesianCoordinate {
    #[pyo3(get, set)]
    pub x: i32,
    #[pyo3(get, set)]
    pub y: i32,
}

#[pymethods]
impl CartesianCoordinate {
    #[new]
    pub fn new(x: i32, y: i32) -> Self {
        CartesianCoordinate { x, y }
    }

    pub fn to_hex(&self) -> HexCoordinate {
        HexCoordinate {
            x: self.x * 2 + (if self.y % 2 == 1 {1} else {0}),
            y: self.y,
        }
    }

    pub fn to_index(&self) -> u64 {
        (self.y - 1 * 8 + self.x) as u64
    }

    #[staticmethod]
    pub fn from_index(index: u64) -> CartesianCoordinate {
        CartesianCoordinate {
            x: (index % 7 + 1) as i32,
            y: (index / 7 + 1) as i32,
        }
    }

    fn __repr__(&self) -> String {
        format!("CartesianCoordinate(x={}, y={})", self.x, self.y)
    }
}

impl std::fmt::Display for CartesianCoordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "CartesianCoordinate(x={}, y={})", self.x, self.y)
    }
}