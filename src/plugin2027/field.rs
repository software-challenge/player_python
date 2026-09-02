
use pyo3::*;

use crate::plugin2027::{
    color::Color, field_content::FieldContent, utils::coordinate::Coordinate
};

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Field {
    #[pyo3(get, set)]
    pub coordinate: Coordinate,
    #[pyo3(get, set)]
    pub content: FieldContent
}

#[pymethods]
impl Field {
    #[new]
    pub fn new(coordinate: Coordinate, color: Color) -> Self {
        Self {
            coordinate,
            content: color.to_field_content(),
        }
    }

    fn __str__(&self) -> String {self.to_string()}
    fn __repr__(&self) -> String {format!("{:?}", self)}
    fn __eq__(&self, other: &Field) -> bool {self == other}
    fn __ne__(&self, other: &Field) -> bool {self != other}
    fn deepcopy(&self) -> Field {self.clone()}

    pub fn is_empty(&self) -> bool {
        self.content == FieldContent::EMPTY
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Field at {} filled with {}", self.coordinate, self.content)
    }
}
