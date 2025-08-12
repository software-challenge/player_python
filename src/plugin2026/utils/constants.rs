use pyo3::*;

#[pyclass]
pub struct PluginConstants;

#[pymethods]
impl PluginConstants {
    pub const BOARD_WIDTH: usize = 10;
    pub const BOARD_HEIGHT: usize = 10;

    pub const ROUND_LIMIT: usize = 30;
}
