pub mod advance;
pub mod card;
pub mod eat_salad;
pub mod exchange_carrots;
pub mod fall_back;

use advance::Advance;
use eat_salad::EatSalad;
use exchange_carrots::ExchangeCarrots;
use fall_back::FallBack;

use pyo3::*;

use super::game_state::GameState;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, FromPyObject)]
pub enum Action {
    Advance(Advance),
    EatSalad(EatSalad),
    ExchangeCarrots(ExchangeCarrots),
    FallBack(FallBack),
}

impl Action {
    pub fn perform(&self, state: &mut GameState) -> Result<(), PyErr> {
        match self {
            Self::Advance(advance) => advance.perform(state),
            Self::EatSalad(eat_salad) => eat_salad.perform(state),
            Self::ExchangeCarrots(exchange_carrots) => exchange_carrots.perform(state),
            Self::FallBack(fall_back) => fall_back.perform(state),
        }
    }
}

impl IntoPy<PyObject> for Action {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            Self::Advance(advance) => advance.into_py(py),
            Self::EatSalad(eat_salad) => eat_salad.into_py(py),
            Self::ExchangeCarrots(exchange_carrots) => exchange_carrots.into_py(py),
            Self::FallBack(fall_back) => fall_back.into_py(py),
        }
    }
}
