use pyo3::{exceptions::PyValueError, intern, prelude::*};

#[pyclass]
struct LRUCache {
    size: usize,
    maxsize: usize,
    cache: PyObject,
}

#[pymethods]
impl LRUCache {
    #[new]
    fn new(py: Python, maxsize: usize) -> PyResult<Self> {
        if maxsize == 0 {
            Err(PyValueError::new_err("maxsize must be positive"))
        } else {
            Ok(Self {
                size: 0,
                maxsize,
                cache: py
                    .import_bound(intern!(py, "collections"))?
                    .getattr(intern!(py, "OrderedDict"))?
                    .call0()?
                    .into(),
            })
        }
    }

    fn __setitem__(
        mut self_: PyRefMut<'_, Self>,
        py: Python,
        key: PyObject,
        value: PyObject,
    ) -> PyResult<()> {
        if self_
            .cache
            .call_method1(py, intern!(py, "__contains__"), (&key,))?
            .extract::<bool>(py)?
        {
            self_
                .cache
                .call_method1(py, intern!(py, "move_to_end"), (&key,))?;
        } else {
            if self_.size >= self_.maxsize {
                self_
                    .cache
                    .call_method1(py, intern!(py, "popitem"), (/*last=*/ false,))?;
                self_.size -= 1;
            }
            self_
                .cache
                .call_method1(py, intern!(py, "__setitem__"), (&key, &value))?;
            self_.size += 1;
        }
        Ok(())
    }

    #[pyo3(signature = (key, /, default=None))]
    fn get(
        self_: PyRef<'_, Self>,
        py: Python,
        key: PyObject,
        default: Option<PyObject>,
    ) -> PyResult<PyObject> {
        let value = self_
            .cache
            .call_method1(py, intern!(py, "get"), (&key, &self_))?
            .extract::<PyObject>(py)?;
        if value.is(&self_) {
            Ok(default.unwrap_or_else(|| py.None()))
        } else {
            self_
                .cache
                .call_method1(py, intern!(py, "move_to_end"), (&key,))?;
            Ok(value)
        }
    }
}

#[pymodule]
#[pyo3(name = "_lib")]
fn lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<LRUCache>()?;
    Ok(())
}
