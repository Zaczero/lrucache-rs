use ordered_hash_map::OrderedHashMap;
use pyo3::{
    exceptions::{PyKeyError, PyValueError},
    prelude::*,
    types::{PyIterator, PyTuple},
};
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

    fn __len__(&self) -> usize {
        self.cache.len()
    }

    fn __contains__(&self, py: Python, key: PyObject) -> bool {
        self.cache.contains_key(&PyObjectWrapper {
            hash: key.bind(py).hash().unwrap(),
            obj: key,
        })
    }

    fn __iter__<'p>(&self, py: Python<'p>) -> PyResult<Bound<'p, PyIterator>> {
        let objects: Vec<PyObject> = self.cache.keys().map(|key| key.obj.clone_ref(py)).collect();
        let tuple = PyTuple::new(py, objects)?;
        PyIterator::from_object(tuple.as_any())
    }

    fn __setitem__(&mut self, py: Python, key: PyObject, value: PyObject) {
        let key = PyObjectWrapper {
            hash: key.bind(py).hash().unwrap(),
            obj: key,
        };
        if let Some(_) = self.cache.get(&key) {
            self.cache.move_to_back(&key);
        } else {
            if self.cache.len() >= self.maxsize {
                self.cache.pop_front();
            }
            self.cache.insert(key, value);
        }
        ()
    }

    fn __getitem__(&mut self, py: Python, key: PyObject) -> PyResult<PyObject> {
        let cache_key = PyObjectWrapper {
            hash: key.bind(py).hash().unwrap(),
            obj: key.clone_ref(py),
        };
        if let Some(value) = self.cache.get(&cache_key) {
            let result = value.clone_ref(py);
            self.cache.move_to_back(&cache_key);
            Ok(result)
        } else {
            Err(PyKeyError::new_err(
                key.bind(py)
                    .repr()
                    .map_or(String::from("key not found"), |s| s.to_string()),
            ))
        }
    }

    fn __delitem__(&mut self, py: Python, key: PyObject) -> PyResult<()> {
        let cache_key = PyObjectWrapper {
            hash: key.bind(py).hash().unwrap(),
            obj: key.clone_ref(py),
        };
        if let Some(_) = self.cache.remove(&cache_key) {
            Ok(())
        } else {
            Err(PyKeyError::new_err(
                key.bind(py)
                    .repr()
                    .map_or(String::from("key not found"), |s| s.to_string()),
            ))
        }
    }

    #[pyo3(signature = (key, /, default=None))]
    fn get(&mut self, py: Python, key: PyObject, default: Option<PyObject>) -> PyObject {
        let cache_key = PyObjectWrapper {
            hash: key.bind(py).hash().unwrap(),
            obj: key,
        };
        if let Some(value) = self.cache.get(&cache_key) {
            let result = value.clone_ref(py);
            self.cache.move_to_back(&cache_key);
            result
        } else {
            default.unwrap_or_else(|| py.None())
        }
    }
}

#[pymodule]
#[pyo3(name = "_lib")]
fn lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<LRUCache>()?;
    Ok(())
}
