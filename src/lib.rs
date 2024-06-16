use plugin::rules_engine::RulesEngine;
use pyo3::*;
use types::PyModule;

pub mod plugin;

use crate::plugin::action::advance::Advance;
use crate::plugin::action::card::Card;
use crate::plugin::action::eat_salad::EatSalad;
use crate::plugin::action::exchange_carrots::ExchangeCarrots;
use crate::plugin::action::fall_back::FallBack;
use crate::plugin::board::Board;
use crate::plugin::constants::PluginConstants;
use crate::plugin::field::Field;
use crate::plugin::game_state::GameState;
use crate::plugin::hare::Hare;
use crate::plugin::hare::TeamEnum;
use crate::plugin::r#move::Move;

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
