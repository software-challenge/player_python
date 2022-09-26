use pyo3::prelude::*;

use crate::plugins::penguins::coordinate::HexCoordinate;
use crate::plugins::penguins::team::Team;

#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone,Debug, Hash)]
pub struct Penguin {
    #[pyo3(get, set)]
    pub position: HexCoordinate,
    #[pyo3(get, set)]
    pub team: Team,
}

#[pymethods]
impl Penguin {
    #[new]
    pub(crate) fn new(position: HexCoordinate, team: Team) -> Self {
        Penguin { position, team }
    }

    fn __repr__(&self) -> String {
        format!("Penguin(position={}, team={})", self.position, self.team)
    }
}

impl std::fmt::Display for Penguin {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Penguin(position={}, team={})", self.position, self.team)
    }
}