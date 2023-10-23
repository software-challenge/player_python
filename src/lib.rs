pub mod plugin;

use pyo3::prelude::*;
use crate::plugin::actions::accelerate::Accelerate;
use crate::plugin::actions::Action;
use crate::plugin::actions::advance::Advance;
use crate::plugin::actions::push::Push;
use crate::plugin::actions::turn::Turn;

use crate::plugin::board::Board;
use crate::plugin::constants::PluginConstants;
use crate::plugin::coordinate::{CartesianCoordinate, CubeDirection};
use crate::plugin::errors::acceleration_errors::AccelerationProblem;
use crate::plugin::errors::ActionProblem;
use crate::plugin::errors::advance_errors::AdvanceProblem;
use crate::plugin::errors::invalid_move_exception::InvalidMoveException;
use crate::plugin::errors::movement_error::MoveMistake;
use crate::plugin::errors::push_error::PushProblem;
use crate::plugin::errors::turn_error::TurnProblem;
use crate::plugin::field::Field;
use crate::plugin::game_state::{GameState};
use crate::plugin::r#move::Move;
use crate::plugin::segment::Segment;
use crate::plugin::ship::Ship;


#[pymodule]
fn socha(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Accelerate>()?;
    m.add_class::<Advance>()?;
    m.add_class::<Push>()?;
    m.add_class::<Turn>()?;

    m.add_class::<AccelerationProblem>()?;
    m.add_class::<AdvanceProblem>()?;
    m.add_class::<InvalidMoveException>()?;
    m.add_class::<MoveMistake>()?;
    m.add_class::<PushProblem>()?;
    m.add_class::<TurnProblem>()?;

    m.add_class::<PluginConstants>()?;

    m.add_class::<CubeDirection>()?;
    m.add_class::<CartesianCoordinate>()?;
    m.add_class::<Ship>()?;
    m.add_class::<Field>()?;
    m.add_class::<Move>()?;
    m.add_class::<Segment>()?;
    m.add_class::<Board>()?;
    m.add_class::<GameState>()?;

    Ok(())
}