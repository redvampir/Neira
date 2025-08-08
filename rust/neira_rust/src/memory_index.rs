use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use hnsw_rs::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

type HnswIndex = Hnsw<'static, f32, DistL2>;

#[derive(Serialize, Deserialize)]
struct MemoryIndexData {
    dim: usize,
    next_id: usize,
    items: HashMap<usize, (String, Vec<f32>)>,
}

#[pyclass]
pub struct MemoryIndex {
    data: Arc<RwLock<MemoryIndexData>>,
    hnsw: Arc<RwLock<HnswIndex>>,
}

#[pymethods]
impl MemoryIndex {
    #[new]
    pub fn new() -> Self {
        let data = MemoryIndexData { dim: 0, next_id: 0, items: HashMap::new() };
        let hnsw = Hnsw::<f32, DistL2>::new(16, 10_000, 16, 200, DistL2 {});
        MemoryIndex {
            data: Arc::new(RwLock::new(data)),
            hnsw: Arc::new(RwLock::new(hnsw)),
        }
    }

    pub fn add(&self, text: String, vector: Vec<f32>) -> PyResult<()> {
        let mut data = self.data.write().unwrap();
        if data.dim == 0 {
            data.dim = vector.len();
        }
        if vector.len() != data.dim {
            return Err(pyo3::exceptions::PyValueError::new_err("dimension mismatch"));
        }
        let id = data.next_id;
        data.next_id += 1;
        data.items.insert(id, (text, vector.clone()));
        drop(data);
        let mut hnsw = self.hnsw.write().unwrap();
        hnsw.insert((vector.as_slice(), id));
        Ok(())
    }

    pub fn similar(&self, vector: Vec<f32>, k: usize) -> PyResult<Vec<String>> {
        let data = self.data.read().unwrap();
        if data.dim == 0 || vector.len() != data.dim {
            return Ok(Vec::new());
        }
        drop(data);
        let hnsw = self.hnsw.read().unwrap();
        let ef = 50.max(k);
        let neighbours = hnsw.search(&vector, k, ef);
        drop(hnsw);
        let data = self.data.read().unwrap();
        let mut results = Vec::new();
        for n in neighbours {
            if let Some((text, _)) = data.items.get(&n.d_id) {
                results.push(text.clone());
            }
        }
        Ok(results)
    }

    pub fn save(&self, path: &str) -> PyResult<()> {
        let data = self.data.read().unwrap();
        let encoded = bincode::serialize(&*data)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        std::fs::write(path, encoded)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
        Ok(())
    }

    pub fn load(&self, path: &str) -> PyResult<()> {
        let bytes = std::fs::read(path)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
        let new_data: MemoryIndexData = bincode::deserialize(&bytes)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        {
            let mut data_lock = self.data.write().unwrap();
            *data_lock = new_data;
        }
        // rebuild hnsw
        let data = self.data.read().unwrap();
        let mut rebuilt = Hnsw::<f32, DistL2>::new(16, 10_000, 16, 200, DistL2 {});
        for (id, (_, vec)) in data.items.iter() {
            rebuilt.insert((vec.as_slice(), *id));
        }
        let mut hnsw = self.hnsw.write().unwrap();
        *hnsw = rebuilt;
        Ok(())
    }
}
