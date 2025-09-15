use pyo3::*;

use crate::plugin2026::{
    utils::vector::Vector
};

#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft
}

#[pymethods]
impl Direction {
    fn __str__(&self) -> String {self.to_string()}
    fn __repr__(&self) -> String {format!("{:?}", self)}
    fn __eq__(&self, other: &Direction) -> bool {self == other}
    fn __ne__(&self, other: &Direction) -> bool {self != other}
    fn deepcopy(&self) -> Direction {*self}

    #[staticmethod]
    pub fn from_vector(vector: &Vector) -> Option<Direction> {
        match (vector.delta_x, vector.delta_y) {
            (0, 1) => Some(Direction::Up),
            (1, 1) => Some(Direction::UpRight),
            (1, 0) => Some(Direction::Right),
            (1, -1) => Some(Direction::DownRight),
            (0, -1) => Some(Direction::Down),
            (-1, -1) => Some(Direction::DownLeft),
            (-1, 0) => Some(Direction::Left),
            (-1, 1) => Some(Direction::UpLeft),
            _ => None,
        }
    }

    #[staticmethod]
    pub fn all_directions() -> Vec<Direction> {
        vec![
            Direction::Up,
            Direction::UpRight,
            Direction::Right,
            Direction::DownRight,
            Direction::Down,
            Direction::DownLeft,
            Direction::Left,
            Direction::UpLeft
        ]
    }

    pub fn to_vector(&self) -> Vector {
        match self {
            Direction::Up => Vector { delta_x: 0, delta_y: 1 },
            Direction::UpRight => Vector { delta_x: 1, delta_y: 1 },
            Direction::Right => Vector { delta_x: 1, delta_y: 0 },
            Direction::DownRight => Vector { delta_x: 1, delta_y: -1 },
            Direction::Down => Vector { delta_x: 0, delta_y: -1 },
            Direction::DownLeft => Vector { delta_x: -1, delta_y: -1 },
            Direction::Left => Vector { delta_x: -1, delta_y: 0 },
            Direction::UpLeft => Vector { delta_x: -1, delta_y: 1 },
        }
    }

    pub fn to_mirrored(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::UpRight => Direction::DownLeft,
            Direction::Right => Direction::Left,
            Direction::DownRight => Direction::UpLeft,
            Direction::Down => Direction::Up,
            Direction::DownLeft => Direction::UpRight,
            Direction::Left => Direction::Right,
            Direction::UpLeft => Direction::DownRight,
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "(Up ↑)"),
            Direction::UpRight => write!(f, "(UpRight ↗)"),
            Direction::Right => write!(f, "(Right →)"),
            Direction::DownRight => write!(f, "(DownRight ↘)"),
            Direction::Down => write!(f, "(Down ↓)"),
            Direction::DownLeft => write!(f, "(DownLeft ↙)"),
            Direction::Left => write!(f, "(Left ←)"),
            Direction::UpLeft => write!(f, "(UpLeft ↖)")
        }
    }
}
