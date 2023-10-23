use pyo3::prelude::*;

use crate::plugin::coordinate::{CubeCoordinates, CubeDirection};
use crate::plugin::field::Field;

#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
pub struct Segment {
    pub direction: CubeDirection,
    pub center: CubeCoordinates,
    pub fields: Vec<Vec<Field>>,
}

#[pymethods]
impl Segment {
    pub fn tip(&self) -> CubeCoordinates {
        self.center.clone() + self.direction.vector() * (self.fields.len() as i32 / 2)
    }

    pub fn get(&self, coordinates: CubeCoordinates) -> Option<Field> {
        let local_coordinates = self.global_to_local(coordinates);
        self.fields
            .get(local_coordinates.to_cartesian().x as usize + 1)
            .and_then(|row| row.get(local_coordinates.to_cartesian().y as usize + 2))
            .cloned()
    }

    pub fn get_index(&self, coordinates: CubeCoordinates) -> Option<u64> {
        coordinates.to_cartesian().to_index()
    }

    pub fn local_to_global(&self, coordinates: CubeCoordinates) -> CubeCoordinates {
        coordinates
            .rotated_by(CubeDirection::Right.turn_count_to(self.direction.clone()))
            + self.center.clone()
    }

    pub fn global_to_local(&self, coordinates: CubeCoordinates) -> CubeCoordinates {
        (coordinates - self.center.clone()).rotated_by(self.direction.turn_count_to(CubeDirection::Right))
    }

    pub fn contains(&self, coordinates: CubeCoordinates) -> bool {
        let local_coordinates = self.global_to_local(coordinates);
        let cartesian_coords = local_coordinates.to_cartesian();

        self.fields
            .get(cartesian_coords.x as usize + 1)
            .map_or(false, |row| row.get(cartesian_coords.y as usize + 2).is_some())
    }
}
