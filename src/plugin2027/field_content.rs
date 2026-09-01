
use pyo3::*;

use crate::plugin2027::{
    color::Color,
};

#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FieldContent {
    BLUE,
    YELLOW,
    RED,
    GREEN,
    EMPTY
}

#[pymethods]
impl FieldContent {
    fn __str__(&self) -> String {self.to_string()}
    fn __repr__(&self) -> String {format!("{:?}", self)}
    fn __eq__(&self, other: &FieldContent) -> bool {self == other}
    fn __ne__(&self, other: &FieldContent) -> bool {self != other}

    pub fn to_team_color(&self) -> Option<Color> {
        match self {
            FieldContent::BLUE => Some(Color::BLUE),
            FieldContent::YELLOW => Some(Color::YELLOW),
            FieldContent::RED => Some(Color::RED),
            FieldContent::GREEN => Some(Color::GREEN),
            FieldContent::EMPTY => None
        }
    }

    pub fn is_empty(&self) -> bool {
        *self == FieldContent::EMPTY
    }
}

impl std::fmt::Display for FieldContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldContent::BLUE => write!(f, "B"),
            FieldContent::YELLOW => write!(f, "Y"),
            FieldContent::RED => write!(f, "R"),
            FieldContent::GREEN => write!(f, "G"),
            FieldContent::EMPTY => write!(f, "-"),
        }
    }
}