use pyo3::*;

use crate::plugin2026::field_type::FieldType;

#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TeamEnum {
    One,
    Two,
}

#[pymethods]
impl TeamEnum {
    pub fn __str__(&self) -> String {self.to_string()}
    pub fn __repr__(&self) -> String {format!("{:?}", self)}
    pub fn __eq__(&self, other: &TeamEnum) -> bool {self == other}
    pub fn __ne_(&self, other: &TeamEnum) -> bool {self != other}

    pub fn get_fish_types(&self) -> Vec<FieldType> {
        match self {
            TeamEnum::One => vec![FieldType::OneS, FieldType::OneM, FieldType::OneL],
            TeamEnum::Two => vec![FieldType::TwoS, FieldType::TwoM, FieldType::TwoL]
        }
    }

    pub fn opponent(&self) -> TeamEnum {
        match self {
            TeamEnum::One => TeamEnum::Two,
            TeamEnum::Two => TeamEnum::One
        }
    }
}

impl std::fmt::Display for TeamEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::One => write!(f, "Team One"),
            Self::Two => write!(f, "Team Two")
        }
    }
}