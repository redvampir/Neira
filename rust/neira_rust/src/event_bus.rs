use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use pyo3::prelude::*;
use pyo3::ffi;

#[pyclass]
#[derive(Clone)]
pub struct Event {
    #[pyo3(get)]
    pub event_type: String,
    #[pyo3(get)]
    pub payload: Py<PyAny>,
}

#[pymethods]
impl Event {
    #[new]
    pub fn new(event_type: String, payload: Py<PyAny>) -> Self {
        Self { event_type, payload }
    }
}

#[pyclass]
pub struct EventBus {
    subscribers: Arc<Mutex<HashMap<String, Vec<Py<PyAny>>>>>,
    sender: mpsc::Sender<Event>,
}

#[pymethods]
impl EventBus {
    #[new]
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<Event>();
        let subs = Arc::new(Mutex::new(HashMap::new()));
        let subs_clone = Arc::clone(&subs);
        thread::spawn(move || {
            for event in rx {
                let handlers: Option<Vec<Py<PyAny>>> = {
                    let map = subs_clone.lock().unwrap();
                    map.get(&event.event_type).cloned()
                };
                if let Some(handlers) = handlers {
                    Python::with_gil(|py| {
                        let py_event = Py::new(py, event).unwrap();
                        for handler in handlers {
                            unsafe {
                                let ret = ffi::PyObject_CallOneArg(handler.as_ptr(), py_event.as_ptr());
                                if ret.is_null() {
                                    PyErr::fetch(py).print(py);
                                } else {
                                    ffi::Py_DECREF(ret);
                                }
                            }
                        }
                    });
                }
            }
        });
        Self {
            subscribers: subs,
            sender: tx,
        }
    }

    pub fn subscribe(&self, event_type: String, handler: Py<PyAny>) {
        let mut subs = self.subscribers.lock().unwrap();
        subs.entry(event_type).or_default().push(handler);
    }

    pub fn publish(&self, event: Event) {
        let _ = self.sender.send(event);
    }
}
