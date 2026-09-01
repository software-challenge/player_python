use pyo3::*;
use types::PyModule;

pub mod plugin2027;

use crate::plugin2027::utils::constants::Constants;
use crate::plugin2027::utils::coordinate::Coordinate;
use crate::plugin2027::utils::direction::Direction;
use crate::plugin2027::utils::game_rule_logic::GameRuleLogic;
use crate::plugin2027::utils::team::TeamEnum;
use crate::plugin2027::utils::vector::Vector;

use crate::plugin2027::board::Board;
use crate::plugin2027::color::Color;
use crate::plugin2027::field_content::FieldContent;
use crate::plugin2027::field::Field;
use crate::plugin2027::game_state::GameState;
use crate::plugin2027::r#move::Move;
use crate::plugin2027::piece_shape::PieceShape;
use crate::plugin2027::piece::Piece;
use crate::plugin2027::rotation::Rotation;

#[pymodule]
fn _socha(m: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_log::init();
    
    m.add_class::<Constants>()?;
    m.add_class::<Coordinate>()?;
    m.add_class::<Direction>()?;
    m.add_class::<GameRuleLogic>()?;
    m.add_class::<TeamEnum>()?;
    m.add_class::<Vector>()?;

    m.add_class::<Board>()?;
    m.add_class::<Color>()?;
    m.add_class::<FieldContent>()?;
    m.add_class::<Field>()?;
    m.add_class::<GameState>()?;
    m.add_class::<Move>()?;
    m.add_class::<PieceShape>()?;
    m.add_class::<Piece>()?;
    m.add_class::<Rotation>()?;

    Ok(())
}
