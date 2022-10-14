use libm::atan2;
use pyo3::prelude::*;

#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
pub struct Vector {
    #[pyo3(get, set)]
    pub dx: i32,
    #[pyo3(get, set)]
    pub dy: i32,
}

#[pymethods]
impl Vector {
    #[new]
    fn new(x: i32, y: i32) -> Self {
        Vector { dx: x, dy: y }
    }

    #[staticmethod]
    pub fn neighbours() -> Vec<Vector> {
        vec![
            Vector { dx: 1, dy: -1 },
            Vector { dx: -2, dy: 0 },
            Vector { dx: 1, dy: 1 },
            Vector { dx: -1, dy: 1 },
            Vector { dx: 2, dy: 0 },
            Vector { dx: -1, dy: -1 },
        ]
    }

    pub fn magnitude(&self) -> f32 {
        ((self.dx.pow(2) + self.dy.pow(2)) as f32).sqrt()
    }

    fn normalize(&self) -> Vector {
        let magnitude = self.magnitude();
        Vector {
            dx: (self.dx as f32 / magnitude).round() as i32,
            dy: (self.dy as f32 / magnitude).round() as i32,
        }
    }

    fn dot(&self, other: &Vector) -> i32 {
        self.dx * other.dx + self.dy * other.dy
    }

    fn cross(&self, other: &Vector) -> i32 {
        self.dx * other.dy - self.dy * other.dx
    }

    fn angle(&self) -> f64 {
        atan2(self.dy as f64, self.dx as f64) * 180.0 / std::f64::consts::PI
    }

    fn scalar(&self, scalar: i32) -> Vector {
        Vector {
            dx: self.dx * scalar,
            dy: self.dy * scalar,
        }
    }

    pub fn add(&self, other: &Vector) -> Vector {
        Vector {
            dx: self.dx + other.dx,
            dy: self.dy + other.dy,
        }
    }

    pub fn sub(&self, other: &Vector) -> Vector {
        Vector {
            dx: self.dx - other.dx,
            dy: self.dy - other.dy,
        }
    }

    fn eq(&self, other: &Vector) -> bool {
        self.magnitude() == other.magnitude() && self.angle() == other.angle()
    }
}

