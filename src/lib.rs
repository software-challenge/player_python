pub mod plugins;

use pyo3::prelude::*;
use crate::plugins::penguins::bitboard::BitBoard;

use crate::plugins::penguins::board::Board;
use crate::plugins::penguins::coordinate::{CartesianCoordinate, HexCoordinate};
use crate::plugins::penguins::field::Field;
use crate::plugins::penguins::game_state::{GameState, Progress, Score, WelcomeMessage};
use crate::plugins::penguins::penguin::Penguin;
use crate::plugins::penguins::r#move::Move;
use crate::plugins::penguins::team::{Team, TeamEnum};
use crate::plugins::penguins::vector::Vector;


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