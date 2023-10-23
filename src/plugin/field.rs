use pyo3::{pyclass, pymethods};

use super::coordinate::CubeDirection;

#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
#[pyclass]
pub struct Passenger {
    pub direction: CubeDirection,
    pub passenger: i32,
}

#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
#[pyclass]
pub enum FieldType {
    Water,
    Island,
    Passenger,
    Goal,
    Sandbank,
}

#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
#[pyclass]
pub struct Field {
    pub field_type: FieldType,
    pub passenger: Option<Passenger>,
}

#[pymethods]
impl Field {
    #[new]
    pub fn new(field_type: FieldType, passenger: Option<Passenger>) -> Self {
        Field {
            field_type,
            passenger,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.field_type == FieldType::Sandbank
            || self.field_type == FieldType::Water
            || self.field_type == FieldType::Goal
    }

    pub fn is_field_type(&self, field_type: FieldType) -> bool {
        self.field_type == field_type
    }
}
