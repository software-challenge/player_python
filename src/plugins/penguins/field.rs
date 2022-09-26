// The Field represents a field on the game board.
// It says where the itself is located and if there is a penguin or fish on it.

use pyo3::prelude::*;

use crate::plugins::penguins::coordinate::HexCoordinate;
use crate::plugins::penguins::penguin::Penguin;
use crate::plugins::penguins::team::Team;

#[pyclass]
#[derive(Clone)]
pub struct Field {
    #[pyo3(get, set)]
    pub coordinate: HexCoordinate,
    #[pyo3(get, set)]
    pub penguin: Option<Penguin>,
    #[pyo3(get, set)]
    pub fish: i32,
}

#[pymethods]
impl Field {
    #[new]
    pub fn new(coordinate: HexCoordinate, penguin: Option<Penguin>, fish: i32) -> Self {
        Field {
            coordinate,
            penguin,
            fish,
        }
    }

    pub fn empty(&self) -> bool {
        self.fish == 0 && self.penguin.is_none()
    }

    pub fn has_penguin(&self) -> bool {
        self.penguin.is_some()
    }

    pub fn get_penguin_team(&self) -> Option<Team> {
        match &self.penguin {
            Some(penguin) => Some(penguin.team.clone()),
            None => None,
        }
    }

    pub fn __repr__(&self) -> String {
        format!(
            "Field(coordinate={}, penguin={:?}, fish={})",
            self.coordinate, self.penguin, self.fish
        )
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Field(coordinate={}, penguin={:?}, fish={})",
            self.coordinate, self.penguin, self.fish
        )
    }
}

