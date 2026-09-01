
use pyo3::*;

#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rotation {
    NONE,
    RIGHT,
    MIRROR, // 180 rotation actually
    LEFT
}

#[pymethods]
impl Rotation {
    fn __str__(&self) -> String {self.to_string()}
    fn __repr__(&self) -> String {format!("{:?}", self)}
    fn __eq__(&self, other: &Rotation) -> bool {self == other}
    fn __ne__(&self, other: &Rotation) -> bool {self != other}

    pub fn value(&self) -> usize {
        match self {
            Rotation::NONE => 0,
            Rotation::RIGHT => 1,
            Rotation::MIRROR => 2,
            Rotation::LEFT => 3
        }
    }

    pub fn rotate(&self, other: &Rotation) -> Rotation {
        let variants = [Rotation::NONE, Rotation::RIGHT, Rotation::MIRROR, Rotation::LEFT];
        let sum = self.value() + other.value();
        variants[sum % variants.len()]
    }

    #[staticmethod]
    pub fn all() -> Vec<Rotation> {
        vec![Rotation::NONE, Rotation::RIGHT, Rotation::MIRROR, Rotation::LEFT]
    }

    pub fn name(&self) -> String {
        format!("{:?}", self)
    }
}

impl std::fmt::Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rotated by {}°", {self.value() * 90})
    }
}