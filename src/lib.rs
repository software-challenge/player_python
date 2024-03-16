pub mod plugin;

use pyo3::exceptions::PyException;
use pyo3::prelude::*;

use crate::plugin::coordinate::CartesianCoordinate;
use crate::plugin::coordinate::CubeCoordinates;
use crate::plugin::coordinate::CubeDirection;
use crate::plugin::game_state::AdvanceInfo;
use crate::plugin::game_state::TeamPoints;

use crate::plugin::actions::accelerate::Accelerate;
use crate::plugin::actions::advance::Advance;
use crate::plugin::actions::push::Push;
use crate::plugin::actions::turn::Turn;

use crate::plugin::board::Board;
use crate::plugin::constants::PluginConstants;
use crate::plugin::field::Field;
use crate::plugin::field::FieldType;
use crate::plugin::field::Passenger;
use crate::plugin::game_state::GameState;
use crate::plugin::r#move::Move;
use crate::plugin::segment::Segment;
use crate::plugin::ship::Ship;
use crate::plugin::ship::TeamEnum;

pyo3::create_exception!(_socha, InvalidMoveException, PyException);
pyo3::create_exception!(_socha, MoveMistake, PyException);
pyo3::create_exception!(_socha, AccelerationProblem, PyException);
pyo3::create_exception!(_socha, AdvanceProblem, PyException);
pyo3::create_exception!(_socha, PushProblem, PyException);
pyo3::create_exception!(_socha, TurnProblem, PyException);

#[pymodule]
fn _socha(py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();

    m.add_class::<Accelerate>()?;
    m.add_class::<Advance>()?;
    m.add_class::<Push>()?;
    m.add_class::<Turn>()?;

    m.add("AccelerationProblem", py.get_type::<AccelerationProblem>())?;
    m.add("AdvanceProblem", py.get_type::<AdvanceProblem>())?;
    m.add(
        "InvalidMoveException",
        py.get_type::<InvalidMoveException>(),
    )?;
    m.add("MoveMistake", py.get_type::<MoveMistake>())?;
    m.add("PushProblem", py.get_type::<PushProblem>())?;
    m.add("TurnProblem", py.get_type::<TurnProblem>())?;

    m.add_class::<PluginConstants>()?;

    m.add_class::<CubeDirection>()?;
    m.add_class::<CartesianCoordinate>()?;
    m.add_class::<CubeCoordinates>()?;
    m.add_class::<TeamEnum>()?;
    m.add_class::<Ship>()?;
    m.add_class::<Passenger>()?;
    m.add_class::<FieldType>()?;
    m.add_class::<Field>()?;
    m.add_class::<Move>()?;
    m.add_class::<Segment>()?;
    m.add_class::<Board>()?;
    m.add_class::<TeamPoints>()?;
    m.add_class::<AdvanceInfo>()?;
    m.add_class::<GameState>()?;

    Ok(())
}
