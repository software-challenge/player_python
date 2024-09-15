use pyo3::{exceptions::PyException, *};

create_exception!(_socha, HUIError, PyException);
