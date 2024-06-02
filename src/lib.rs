use pyo3::*;
use types::PyModule;

pub mod plugin;

use crate::plugin::constants::PluginConstants;
use crate::plugin::field::Field;
use crate::plugin::r#move::Move;
use crate::plugin::hare::TeamEnum;
use crate::plugin::hare::Hare;
use crate::plugin::board::Board;
use crate::plugin::game_state::GameState;
use crate::plugin::action::advance::Advance;
use crate::plugin::action::card::Card;
use crate::plugin::action::eat_salad::EatSalad;
use crate::plugin::action::exchange_carrots::ExchangeCarrots;
use crate::plugin::action::fall_back::FallBack;

#[pyfunction]
pub fn calculates_carrots(distance: usize) -> i32 {
    let distancce_i32: i32 = distance.try_into().unwrap();
    (distancce_i32 * (distancce_i32 + 1)) / 2
}

#[pymodule]
fn _socha(m: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_log::init();

    m.add_function(wrap_pyfunction!(calculates_carrots, m)?)?;

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

    Ok(())
}
