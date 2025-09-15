use pyo3::*;

#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector {
    #[pyo3(get, set)]
    pub delta_x: isize,
    #[pyo3(get, set)]
    pub delta_y: isize,
}

#[pymethods]
impl Vector {
    #[new]
    pub fn new(delta_x: isize, delta_y: isize) -> Self {
        Self {
            delta_x, delta_y
        }
    }

    fn __str__(&self) -> String {self.to_string()}
    fn __repr__(&self) -> String {format!("{:?}", self)}
    fn __eq__(&self, other: &Vector) -> bool {self == other}
    fn __ne__(&self, other: &Vector) -> bool {self != other}
    fn deepcopy(&self) -> Vector {*self}

    pub fn add_vector(&self, other: &Vector) -> Vector {
        Vector {
            delta_x: self.delta_x + other.delta_x,
            delta_y: self.delta_y + other.delta_y
        }
    }
    
    pub fn add_vector_mut(&mut self, other: &Vector) {
        self.delta_x += other.delta_x;
        self.delta_y += other.delta_y;
    }

    pub fn scale(&self, scalar: isize) -> Vector {
        Vector {
            delta_x: self.delta_x * scalar,
            delta_y: self.delta_y * scalar
        }
    }

    pub fn scale_mut(&mut self, scalar: isize) {
        self.delta_x *= scalar;
        self.delta_y *= scalar;
    }

    pub fn get_length(&self) -> Option<f32> {
        let squared_length = self.delta_x * self.delta_x + self.delta_y * self.delta_y;

        if squared_length < 0 {
            None  // Return None for negative numbers
        } else {
            Some((squared_length as f32).sqrt())  // Convert to f32 then compute sqrt
        }
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "vec({}, {})", self.delta_x, self.delta_y)
    }
}