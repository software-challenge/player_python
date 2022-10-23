pub mod plugin;

use pyo3::prelude::*;

use crate::plugin::bitboard::BitBoard;
use crate::plugin::board::Board;
use crate::plugin::coordinate::{CartesianCoordinate, HexCoordinate};
use crate::plugin::field::Field;
use crate::plugin::game_state::{GameState, Progress, Score, WelcomeMessage};
use crate::plugin::penguin::Penguin;
use crate::plugin::r#move::Move;
use crate::plugin::team::{Team, TeamEnum};
use crate::plugin::vector::Vector;


#[pymodule]
fn socha(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Vector>()?;
    m.add_class::<HexCoordinate>()?;
    m.add_class::<CartesianCoordinate>()?;
    m.add_class::<Team>()?;
    m.add_class::<TeamEnum>()?;
    m.add_class::<Penguin>()?;
    m.add_class::<Field>()?;
    m.add_class::<Move>()?;
    m.add_class::<Board>()?;
    m.add_class::<BitBoard>()?;
    m.add_class::<GameState>()?;
    m.add_class::<WelcomeMessage>()?;
    m.add_class::<Score>()?;
    m.add_class::<Progress>()?;

    Ok(())
}