use pyo3::prelude::*;
use crate::plugin::vector::Vector;

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
            x: (self.x as f32 / 2.0 - (if self.y as f32 % 2.0 == 1.0 { 1.0 } else { 0.0 })).ceil() as i32,
            y: self.y,
        }
    }

    pub fn to_vector(&self) -> Vector {
        Vector {
            dx: self.x,
            dy: self.y,
        }
    }

    pub fn get_neighbours(&self) -> Vec<HexCoordinate> {
        Vector::neighbours().iter().map(|vector| HexCoordinate {
            x: self.x + vector.dx,
            y: self.y + vector.dy,
        }).collect()
    }

    pub fn add_vector(&self, vector: &Vector) -> HexCoordinate {
        let vector: Vector = self.to_vector().add(vector);
        HexCoordinate {
            x: vector.dx,
            y: vector.dy,
        }
    }

    pub fn subtract_vector(&self, vector: &Vector) -> HexCoordinate {
        let vector: Vector = self.to_vector().sub(vector);
        HexCoordinate {
            x: vector.dx,
            y: vector.dy,
        }
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

    pub fn to_vector(&self) -> Vector {
        Vector {
            dx: self.x,
            dy: self.y,
        }
    }

    pub fn add_vector(&self, vector: &Vector) -> CartesianCoordinate {
        let vector: Vector = self.to_vector().add(vector);
        CartesianCoordinate {
            x: vector.dx,
            y: vector.dy,
        }
    }

    pub fn subtract_vector(&self, vector: &Vector) -> CartesianCoordinate {
        let vector: Vector = self.to_vector().sub(vector);
        CartesianCoordinate {
            x: vector.dx,
            y: vector.dy,
        }
    }

    pub fn distance(&self, other: &CartesianCoordinate) -> i32 {
        let vector: Vector = self.to_vector().sub(&other.to_vector());
        vector.magnitude() as i32
    }

    pub fn to_hex(&self) -> HexCoordinate {
        HexCoordinate {
            x: self.x * 2 + (if self.y % 2 == 1 { 1 } else { 0 }),
            y: self.y,
        }
    }

    pub fn to_index(&self) -> Option<u64> {
        if self.x < 0 || self.y < 0 || self.x > 7 || self.y > 7 {
            return None;
        }
        Some((self.y * 8 + self.x) as u64)
    }

    #[staticmethod]
    pub fn from_index(index: u64) -> CartesianCoordinate {
        CartesianCoordinate {
            x: (index % 8) as i32,
            y: (index / 8) as i32,
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