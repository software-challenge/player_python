use pyo3::prelude::*;
use crate::plugins::penguins::coordinate::HexCoordinate;
use crate::plugins::penguins::team::Team;


#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
pub struct Move {
    #[pyo3(get, set)]
    pub from: Option<HexCoordinate>,
    #[pyo3(get, set)]
    pub to: HexCoordinate,
    #[pyo3(get, set)]
    pub team: Team,
}

#[pymethods]
impl Move {
    #[new]
    pub fn new(from: Option<HexCoordinate>, to: HexCoordinate, team: Team) -> Self {
        Move { from, to, team }
    }

    pub fn delta(&self) -> i32 {
        match &self.from {
            Some(from) => from.distance(&self.to),
            None => self.to.distance(&HexCoordinate::new(0, 0)),
        }
    }

    pub fn reverse(&self) -> Move {
        Move::new(Some(self.to.clone()),
                  self.from.clone().unwrap_or(HexCoordinate::new(0, 0)),
                  self.team.clone())
    }
}
