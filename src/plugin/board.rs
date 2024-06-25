use pyo3::*;

use super::field::Field;

#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
pub struct Board {
    #[pyo3(get)]
    pub track: Vec<Field>,
}

#[pymethods]
impl Board {
    /// Creates a new board with the given fields.
    #[new]
    #[must_use]
    pub fn new(track: Vec<Field>) -> Self {
        Self { track }
    }

    /// Returns the field at the specified index, or `None` if the index is out of bounds.
    pub fn get_field(&self, index: usize) -> Option<Field> {
        self.track.get(index).copied()
    }

    /// Finds the index of the specified field within the given range.
    pub fn find_field(&self, field: Field, start: usize, end: usize) -> Option<usize> {
        (start..end).find(|&i| self.track.get(i) == Some(&field))
    }

    /// Finds the previous occurrence of the specified field before the given index.
    pub fn get_previous_field(&self, field: Field, index: usize) -> Option<usize> {
        if index == 0 {
            None
        } else {
            (0..index)
                .rev()
                .find(|&i| self.track.get(i) == Some(&field))
        }
    }

    /// Finds the next occurrence of the specified field after the given index.
    pub fn get_next_field(&self, field: Field, index: usize) -> Option<usize> {
        (index + 1..self.track.len()).find(|&i| self.track.get(i) == Some(&field))
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self)
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for field in &self.track {
            write!(f, "{}", field)?;
        }
        Ok(())
    }
}
