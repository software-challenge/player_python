use pyo3::prelude::*;

use crate::plugin::coordinate::{ CubeCoordinates, CubeDirection };
use crate::plugin::field::Field;

use super::coordinate::CartesianCoordinate;

#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
pub struct Segment {
    pub direction: CubeDirection,
    pub center: CubeCoordinates,
    pub fields: Vec<Vec<Field>>,
}

#[pymethods]
impl Segment {
    #[new]
    pub fn new(direction: CubeDirection, center: CubeCoordinates, fields: Vec<Vec<Field>>) -> Self {
        Segment {
            direction,
            center,
            fields,
        }
    }

    pub fn tip(&self) -> CubeCoordinates {
        self.center + self.direction.vector() * ((self.fields.len() as i32) / 2)
    }

    pub fn get(&self, coordinates: CubeCoordinates) -> Option<Field> {
        let local: CubeCoordinates = self.global_to_local(coordinates);

        let local_cart: CartesianCoordinate = self.array_coords(local);
        self.fields
            .get(local_cart.x as usize)
            .and_then(|c| c.get(local_cart.y as usize))
            .cloned()
    }

    pub fn local_to_global(&self, coordinates: CubeCoordinates) -> CubeCoordinates {
        coordinates.rotated_by(CubeDirection::Right.turn_count_to(self.direction)) + self.center
    }

    pub fn global_to_local(&self, coordinates: CubeCoordinates) -> CubeCoordinates {
        (coordinates - self.center).rotated_by(self.direction.turn_count_to(CubeDirection::Right))
    }

    pub fn contains(&self, coordinates: CubeCoordinates) -> bool {
        self.get(coordinates).is_some()
    }

    fn array_coords(&self, coords: CubeCoordinates) -> CartesianCoordinate {
        CartesianCoordinate::new(coords.array_x() + 1, coords.y() + 2)
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(
            format!(
                "Segment(direction={:?}, center={:?}, fields={:?})",
                self.direction,
                self.center,
                self.fields
            )
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin::{ coordinate::{ CubeCoordinates, CubeDirection }, field::FieldType };

    #[test]
    fn test_tip() {
        let segment: Segment = Segment {
            direction: CubeDirection::Right,
            center: CubeCoordinates::new(0, 0),
            fields: vec![vec![Field::new(FieldType::Water, None); 4]; 5],
        };
        assert_eq!(segment.tip(), CubeCoordinates::new(2, 0));
    }

    #[test]
    fn test_get() {
        let mut segment: Segment = Segment {
            direction: CubeDirection::Right,
            center: CubeCoordinates::new(0, 0),
            fields: vec![vec![Field::new(FieldType::Water, None); 5]; 4],
        };
        segment.fields[0][0] = Field::new(FieldType::Island, None);
        segment.fields[3][4] = Field::new(FieldType::Island, None);

        println!("{:?}", segment.fields);

        assert_eq!(
            segment.get(CubeCoordinates::new(-1, -2)),
            Some(Field::new(FieldType::Island, None))
        );
        assert_eq!(
            segment.get(CubeCoordinates::new(0, 2)),
            Some(Field::new(FieldType::Island, None))
        );
        assert_eq!(
            segment.get(CubeCoordinates::new(1, 0)),
            Some(Field::new(FieldType::Water, None))
        );
        assert_eq!(
            segment.get(CubeCoordinates::new(2, 0)),
            Some(Field::new(FieldType::Water, None))
        );
        assert_eq!(segment.get(CubeCoordinates::new(3, 0)), None);
        assert_eq!(segment.get(CubeCoordinates::new(-2, 0)), None);
    }

    #[test]
    fn test_local_to_global() {
        let segment: Segment = Segment {
            direction: CubeDirection::Left,
            center: CubeCoordinates::new(3, 0),
            fields: vec![vec![Field::new(FieldType::Water, None); 4]; 5],
        };
        assert_eq!(segment.local_to_global(CubeCoordinates::new(3, 0)), CubeCoordinates::new(0, 0));
        assert_eq!(segment.local_to_global(CubeCoordinates::new(2, 0)), CubeCoordinates::new(1, 0));
        assert_eq!(segment.local_to_global(CubeCoordinates::new(1, 0)), CubeCoordinates::new(2, 0));
        assert_eq!(segment.local_to_global(CubeCoordinates::new(0, 0)), CubeCoordinates::new(3, 0));
        assert_eq!(
            segment.local_to_global(CubeCoordinates::new(-1, 0)),
            CubeCoordinates::new(4, 0)
        );
        assert_eq!(
            segment.local_to_global(CubeCoordinates::new(-2, 0)),
            CubeCoordinates::new(5, 0)
        );
        assert_eq!(
            segment.local_to_global(CubeCoordinates::new(-3, 0)),
            CubeCoordinates::new(6, 0)
        );
        assert_eq!(
            segment.local_to_global(CubeCoordinates::new(-4, 0)),
            CubeCoordinates::new(7, 0)
        );
    }

    #[test]
    fn test_global_to_local() {
        let segment: Segment = Segment {
            direction: CubeDirection::Left,
            center: CubeCoordinates::new(3, 0),
            fields: vec![vec![Field::new(FieldType::Water, None); 4]; 5],
        };
        assert_eq!(segment.global_to_local(CubeCoordinates::new(0, 0)), CubeCoordinates::new(3, 0));
        assert_eq!(segment.global_to_local(CubeCoordinates::new(1, 0)), CubeCoordinates::new(2, 0));
        assert_eq!(segment.global_to_local(CubeCoordinates::new(2, 0)), CubeCoordinates::new(1, 0));
        assert_eq!(segment.global_to_local(CubeCoordinates::new(3, 0)), CubeCoordinates::new(0, 0));
        assert_eq!(
            segment.global_to_local(CubeCoordinates::new(4, 0)),
            CubeCoordinates::new(-1, 0)
        );
        assert_eq!(
            segment.global_to_local(CubeCoordinates::new(5, 0)),
            CubeCoordinates::new(-2, 0)
        );
        assert_eq!(
            segment.global_to_local(CubeCoordinates::new(6, 0)),
            CubeCoordinates::new(-3, 0)
        );
        assert_eq!(
            segment.global_to_local(CubeCoordinates::new(7, 0)),
            CubeCoordinates::new(-4, 0)
        );
    }

    #[test]
    fn test_contains() {}
}
