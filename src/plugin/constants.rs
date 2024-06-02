use pyo3::*;

#[pyclass]
pub struct PluginConstants;

#[pymethods]
impl PluginConstants {
    pub const NUM_FIELDS: usize = 65;

    pub const INITIAL_SALADS: i32 = 5;
    pub const INITIAL_CARROTS: i32 = 68;

    pub const ROUND_LIMIT: i32 = 30;
}
