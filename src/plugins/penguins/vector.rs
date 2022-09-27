use libm::atan2;
use pyo3::prelude::*;

#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
pub struct Vector {
    #[pyo3(get, set)]
    pub x: i32,
    #[pyo3(get, set)]
    pub y: i32,
}

#[pymethods]
impl Vector {
    #[new]
    fn new(x: i32, y: i32) -> Self {
        Vector { x, y }
    }

    pub fn neighbours(&self) -> Vec<Vector> {
        let mut neighbours = Vec::new();
        if self.x > 0 {
            neighbours.push(Vector::new(self.x + 1, self.y - 1));
        }
        if self.y > 1 {
            neighbours.push(Vector::new(self.x - 2, self.y));
        }
        if self.x < 7 {
            neighbours.push(Vector::new(self.x + 1, self.y + 1));
        }
        if self.y < 7 {
            neighbours.push(Vector::new(self.x - 1, self.y + 1));
        }
        if self.x > 1 && self.y < 7 {
            neighbours.push(Vector::new(self.x - 2, self.y));
        }
        if self.x < 7 && self.y > 0 {
            neighbours.push(Vector::new(self.x - 1, self.y - 1));
        }
        neighbours
    }

    pub fn magnitude(&self) -> f32 {
        ((self.x.pow(2) + self.y.pow(2)) as f32).sqrt()
    }

    fn normalize(&self) -> Vector {
        let magnitude = self.magnitude();
        Vector {
            x: (self.x as f32 / magnitude).round() as i32,
            y: (self.y as f32 / magnitude).round() as i32,
        }
    }

    fn dot(&self, other: &Vector) -> i32 {
        self.x * other.x + self.y * other.y
    }

    fn cross(&self, other: &Vector) -> i32 {
        self.x * other.y - self.y * other.x
    }

    fn angle(&self) -> f64 {
        atan2(self.y as f64, self.x as f64) * 180.0 / std::f64::consts::PI
    }

    fn scalar(&self, scalar: i32) -> Vector {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    pub fn add(&self, other: &Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn sub(&self, other: &Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn eq(&self, other: &Vector) -> bool {
        self.magnitude() == other.magnitude() && self.angle() == other.angle()
    }
}

