use pyo3::*;

#[pyclass]
pub struct Constants;

#[pymethods]
impl Constants {
    pub const BOARD_LENGTH: usize = 20;
    pub const ROUND_LIMIT: usize = 25;
    pub const TOTAL_PIECE_SHAPES: usize = 21;
    pub const COLORS: usize = 4;
    pub const VALIDATE_MOVE: bool = true;
}
