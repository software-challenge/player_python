use pyo3::prelude::*;
use crate::plugins::penguins::coordinate::HexCoordinate;
use crate::TeamEnum;

use super::coordinate::CartesianCoordinate;


#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
pub struct Move {
    #[pyo3(get, set)]
    pub _from: Option<HexCoordinate>,
    #[pyo3(get, set)]
    pub to: HexCoordinate,
    #[pyo3(get, set)]
    pub team: TeamEnum,
}

#[pymethods]
impl Move {
    #[new]
    pub fn new(_from: Option<HexCoordinate>, to: HexCoordinate, team: TeamEnum) -> Self {
        Move { _from, to, team }
    }

    pub fn delta(&self) -> i32 {
        match &self._from {
            Some(from) => from.to_cartesian().distance(&self.to.to_cartesian()),
            None => self.to.to_cartesian().distance(&CartesianCoordinate::new(0, 0)),
        }
    }

    pub fn reverse(&self) -> Move {
        Move::new(Some(self.to.clone()),
                  self._from.clone().unwrap_or(HexCoordinate::new(0, 0)),
                  self.team.clone())
    }

    pub fn __repr__(&self) -> String {
        format!("Move(from={:?}, to={}, team={})", self._from, self.to, self.team)
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Move(from={:?}, to={}, team={})", self._from, self.to, self.team)
    }
}
