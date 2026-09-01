
use pyo3::*;

use crate::plugin2027::{
    utils::team::TeamEnum,
    field_content::FieldContent
};

#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    BLUE,
    YELLOW,
    RED,
    GREEN
}

#[pymethods]
impl Color {
    fn __str__(&self) -> String {self.to_string()}
    fn __repr__(&self) -> String {format!("{:?}", self)}
    fn __eq__(&self, other: &Color) -> bool {self == other}
    fn __ne__(&self, other: &Color) -> bool {self != other}
    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    pub fn next(&self) -> Color {
        match self {
            Color::BLUE => Color::YELLOW,
            Color::YELLOW => Color::RED,
            Color::RED => Color::GREEN,
            Color::GREEN => Color::BLUE
        }
    }

    pub fn team(&self) -> TeamEnum {
        match self {
            Color::BLUE | Color::RED => TeamEnum::One,
            Color::YELLOW | Color::GREEN => TeamEnum::Two,
        }
    }

    pub fn to_field_content(&self) -> FieldContent {
        match self {
            Color::BLUE => FieldContent::BLUE,
            Color::YELLOW => FieldContent::YELLOW,
            Color::RED => FieldContent::RED,
            Color::GREEN => FieldContent::GREEN
        }
    }

    pub fn name(&self) -> String {
        format!("{:?}", self)
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::BLUE => write!(f, "Color Blue"),
            Color::YELLOW => write!(f, "Color Yellow"),
            Color::RED => write!(f, "Color Red"),
            Color::GREEN => write!(f, "Color green"),
        }
    }
}