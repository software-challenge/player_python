use pyo3::*;

use super::action::card::Card;

#[pyclass]
pub struct PluginConstants;

#[pymethods]
impl PluginConstants {
    pub const NUM_FIELDS: usize = 65;

    pub const INITIAL_SALADS: i32 = 5;
    pub const INITIAL_CARROTS: i32 = 68;

    pub const ROUND_LIMIT: usize = 30;

    pub const LAST_LETTUCE_POSITION: usize = 57;

    pub const MARKET_SELECTION: [Card; 4] = [
        Card::FallBack,
        Card::HurryAhead,
        Card::EatSalad,
        Card::SwapCarrots,
    ];
}
