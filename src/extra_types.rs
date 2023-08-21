use pyo3::{pyclass, pymethods, types::PyModule, PyResult, Python};

macro_rules! simple_wrapper {
    ($name:ident, $ttype:ty) => {
        #[pyclass]
        #[derive(Clone)]
        pub struct $name {
            inner: $ttype,
        }

        impl $name {
            #[must_use]
            pub fn get_value(&self) -> $ttype {
                self.inner
            }
        }

        #[pymethods]
        impl $name {
            #[new]
            #[must_use]
            pub fn py_new(val: $ttype) -> Self {
                Self { inner: val }
            }

            #[must_use]
            pub fn __str__(&self) -> String {
                format!("{}({})", stringify!($name), self.inner)
            }
        }
    };
}

simple_wrapper!(SmallInt, i16);
simple_wrapper!(TinyInt, i8);
simple_wrapper!(BigInt, i64);
simple_wrapper!(Double, f64);
simple_wrapper!(Counter, i64);

/// Create new module for extra types.
///
/// # Errors
///
/// May return error if module cannot be created,
/// or any of classes cannot be added.
pub fn add_module<'a>(py: Python<'a>, name: &'static str) -> PyResult<&'a PyModule> {
    let module = PyModule::new(py, name)?;
    module.add_class::<SmallInt>()?;
    module.add_class::<TinyInt>()?;
    module.add_class::<BigInt>()?;
    module.add_class::<Double>()?;
    Ok(module)
}
