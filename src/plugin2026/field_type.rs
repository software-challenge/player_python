use pyo3::*;

use crate::plugin2026::utils::team::TeamEnum;

#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FieldType {
    OneS,
    OneM,
    OneL,
    TwoS,
    TwoM,
    TwoL,
    Squid,
    Empty
}

#[pymethods]
impl FieldType {
    pub fn __str__(&self) -> String {self.to_string()}
    pub fn __repr__(&self) -> String {format!("{:?}", self)}
    pub fn __eq__(&self, other: &FieldType) -> bool {self == other}
    pub fn __ne_(&self, other: &FieldType) -> bool {self != other}
    
    pub fn get_value(&self) -> usize {
        match self {
            FieldType::OneS => 1,
            FieldType::OneM => 2,
            FieldType::OneL => 3,
            FieldType::TwoS => 1,
            FieldType::TwoM => 2,
            FieldType::TwoL => 3,
            FieldType::Squid => 0,
            FieldType::Empty => 0,
        }
    }

    pub fn get_team(&self) -> Option<TeamEnum> {
        match self {
            FieldType::OneS => Some(TeamEnum::One),
            FieldType::OneM => Some(TeamEnum::One),
            FieldType::OneL => Some(TeamEnum::One),
            FieldType::TwoS => Some(TeamEnum::Two),
            FieldType::TwoM => Some(TeamEnum::Two),
            FieldType::TwoL => Some(TeamEnum::Two),
            _ => None
        }
    }

    #[staticmethod]
    pub fn all_field_types() -> Vec<FieldType> {
        vec![
            FieldType::OneS,
            FieldType::OneM,
            FieldType::OneL,
            FieldType::TwoS,
            FieldType::TwoM,
            FieldType::TwoL,
            FieldType::Squid,
            FieldType::Empty
        ]
    }
}

impl std::fmt::Display for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OneS => write!(f, "O1"),
            Self::OneM => write!(f, "O2"),
            Self::OneL => write!(f, "O3"),
            Self::TwoS => write!(f, "T1"),
            Self::TwoM => write!(f, "T2"),
            Self::TwoL => write!(f, "T3"),
            Self::Squid => write!(f, "SQ"),
            Self::Empty => write!(f, "--"),
        }
    }
}