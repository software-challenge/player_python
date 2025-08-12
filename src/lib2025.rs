use plugin2025::rules_engine::RulesEngine;
use pyo3::*;
use types::PyModule;

pub mod plugin2025;

use crate::plugin2025::action::advance::Advance;
use crate::plugin2025::action::card::Card;
use crate::plugin2025::action::eat_salad::EatSalad;
use crate::plugin2025::action::exchange_carrots::ExchangeCarrots;
use crate::plugin2025::action::fall_back::FallBack;
use crate::plugin2025::board::Board;
use crate::plugin2025::constants::PluginConstants;
use crate::plugin2025::field::Field;
use crate::plugin2025::game_state::GameState;
use crate::plugin2025::hare::Hare;
use crate::plugin2025::hare::TeamEnum;
use crate::plugin2025::r#move::Move;

#[pymodule]
fn _socha(m: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_log::init();

    m.add_class::<Advance>()?;
    m.add_class::<Card>()?;
    m.add_class::<EatSalad>()?;
    m.add_class::<ExchangeCarrots>()?;
    m.add_class::<FallBack>()?;

    m.add_class::<PluginConstants>()?;
    m.add_class::<Field>()?;
    m.add_class::<Move>()?;
    m.add_class::<TeamEnum>()?;
    m.add_class::<Hare>()?;
    m.add_class::<Board>()?;
    m.add_class::<GameState>()?;

    m.add_class::<RulesEngine>()?;

    Ok(())
}
