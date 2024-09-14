use ordered_hash_map::OrderedHashMap;
use pyo3::{exceptions::PyValueError, prelude::*};
use std::hash::{Hash, Hasher};

struct PyObjectWrapper {
    hash: isize,
    obj: PyObject,
}

impl Hash for PyObjectWrapper {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

impl PartialEq for PyObjectWrapper {
    fn eq(&self, other: &Self) -> bool {
        Python::with_gil(|py| self.obj.bind(py).eq(other.obj.bind(py)).unwrap())
    }
}

impl Eq for PyObjectWrapper {}

#[pyclass]
struct LRUCache {
    maxsize: usize,
    cache: OrderedHashMap<PyObjectWrapper, PyObject>,
}

#[pymethods]
impl LRUCache {
    #[new]
    fn new(maxsize: usize) -> PyResult<Self> {
        if maxsize == 0 {
            Err(PyValueError::new_err("maxsize must be positive"))
        } else {
            Ok(Self {
                maxsize,
                cache: OrderedHashMap::with_capacity(maxsize),
            })
        }
    }

    fn __setitem__(
        mut self_: PyRefMut<'_, Self>,
        py: Python,
        key: PyObject,
        value: PyObject,
    ) -> PyResult<()> {
        let key = PyObjectWrapper {
            hash: key.bind(py).hash().unwrap(),
            obj: key,
        };
        if let Some(_) = self_.cache.get(&key) {
            self_.cache.move_to_back(&key);
        } else {
            if self_.cache.len() >= self_.maxsize {
                self_.cache.pop_front();
            }
            self_.cache.insert(key, value);
        }
        Ok(())
    }

    #[pyo3(signature = (key, /, default=None))]
    fn get(
        mut self_: PyRefMut<'_, Self>,
        py: Python,
        key: PyObject,
        default: Option<PyObject>,
    ) -> PyResult<PyObject> {
        let key = PyObjectWrapper {
            hash: key.bind(py).hash().unwrap(),
            obj: key,
        };
        if let Some(value) = self_.cache.get(&key) {
            let result = value.clone_ref(py);
            self_.cache.move_to_back(&key);
            Ok(result)
        } else {
            Ok(default.unwrap_or_else(|| py.None()))
        }
    }
}

#[pymodule]
#[pyo3(name = "_lib")]
fn lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<LRUCache>()?;
    Ok(())
}
