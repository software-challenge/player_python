use pyo3::prelude::*;

#[pyclass]
pub struct PluginConstants;

#[pymethods]
impl PluginConstants {
    pub const ROUND_LIMIT: i32 = 30;

    // Ship Properties
    pub const START_COAL: i32 = 6;
    pub const MIN_SPEED: i32 = 1;
    pub const MAX_SPEED: i32 = 6;
    pub const FREE_ACC: i32 = 1;

    // Points
    pub const FINISH_POINTS: i32 = 6;
    pub const POINTS_PER_PASSENGER: i32 = 5;
    pub const POINTS_PER_SEGMENT: i32 = 5;

    pub const NUMBER_OF_PASSENGERS: i32 = 5;

    // Board
    pub const SEGMENT_FIELDS_WIDTH: i32 = 4;
    pub const SEGMENT_FIELDS_HEIGHT: i32 = 5;
    pub const NUMBER_OF_SEGMENTS: i32 = 8;

    // Board Fields
    pub const MAX_SPECIAL: i32 = 0;
    // Sandbanks disabled
    pub const MIN_SPECIAL: i32 = 0;
    pub const MAX_ISLANDS: i32 = 3;
    pub const MIN_ISLANDS: i32 = 2;
}
