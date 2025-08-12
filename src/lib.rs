use pyo3::*;
use types::PyModule;

pub mod plugin2026;

use crate::plugin2026::utils::vector::Vector;
use crate::plugin2026::utils::direction::Direction;
use crate::plugin2026::utils::coordinate::Coordinate;
use crate::plugin2026::utils::constants::PluginConstants;
use crate::plugin2026::utils::team::TeamEnum;

use crate::plugin2026::game_state::GameState;
use crate::plugin2026::board::Board;
use crate::plugin2026::field_type::FieldType;
use crate::plugin2026::r#move::Move;

use crate::plugin2026::rules_engine::RulesEngine;

#[pymodule]
fn _socha(m: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_log::init();

    m.add_class::<Vector>()?;
    m.add_class::<Direction>()?;
    m.add_class::<Coordinate>()?;
    m.add_class::<PluginConstants>()?;
    m.add_class::<TeamEnum>()?;

    m.add_class::<GameState>()?;
    m.add_class::<Board>()?;
    m.add_class::<FieldType>()?;
    m.add_class::<Move>()?;

    m.add_class::<RulesEngine>()?;

    Ok(())
}
