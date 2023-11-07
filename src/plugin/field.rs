use pyo3::prelude::*;

use super::coordinate::CubeDirection;

#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash, Copy)]
#[pyclass]
pub struct Passenger {
    pub direction: CubeDirection,
    pub passenger: i32,
}

#[pymethods]
impl Passenger {
    #[new]
    pub fn new(direction: CubeDirection, passenger: i32) -> Self {
        Passenger {
            direction,
            passenger,
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "Passenger(direction={:?}, passenger={})",
            self.direction, self.passenger
        ))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash, Copy)]
#[pyclass]
pub enum FieldType {
    Water,
    Island,
    Passenger,
    Goal,
    Sandbank,
}

#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash, Copy)]
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
        self.field_type == FieldType::Sandbank ||
            self.field_type == FieldType::Water ||
            self.field_type == FieldType::Goal
    }

    pub fn is_field_type(&self, field_type: FieldType) -> bool {
        self.field_type == field_type
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "Field(field_type={:?}, passenger={:?})",
            self.field_type, self.passenger
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_field() {
        let field = Field::new(FieldType::Water, None);
        assert_eq!(field.field_type, FieldType::Water);
        assert_eq!(field.passenger, None);
    }

    #[test]
    fn test_is_empty() {
        let field1 = Field::new(FieldType::Sandbank, None);
        assert_eq!(field1.is_empty(), true);

        let field2 = Field::new(FieldType::Water, None);
        assert_eq!(field2.is_empty(), true);

        let field3 = Field::new(FieldType::Goal, None);
        assert_eq!(field3.is_empty(), true);

        let field4 = Field::new(FieldType::Island, None);
        assert_eq!(field4.is_empty(), false);

        let field5 = Field::new(
            FieldType::Passenger,
            Some(Passenger {
                direction: CubeDirection::DownRight,
                passenger: 1,
            })
        );
        assert_eq!(field5.is_empty(), false);
    }

    #[test]
    fn test_is_field_type() {
        let field = Field::new(FieldType::Water, None);
        assert_eq!(field.is_field_type(FieldType::Water), true);
        assert_eq!(field.is_field_type(FieldType::Island), false);
    }
}
