use crate::PyProject;
use omf::file::Limits;
use omf::file::Reader;
use pyo3::exceptions::PyIOError;
use pyo3::prelude::*;
use std::fs::File;

#[pyclass(name = "Limits")]
pub struct PyLimits {
    #[pyo3(get, set)]
    pub json_bytes: Option<u64>,
    #[pyo3(get, set)]
    pub image_bytes: Option<u64>,
    #[pyo3(get, set)]
    pub image_dim: Option<u32>,
    #[pyo3(get, set)]
    pub validation: Option<u32>,
}

#[pymethods]
impl PyLimits {
    #[new]
    pub fn new() -> PyResult<Self> {
        let limits = Limits::default();
        Ok(PyLimits {
            json_bytes: limits.json_bytes,
            image_bytes: limits.image_bytes,
            image_dim: limits.image_dim,
            validation: limits.validation,
        })
    }
}

#[pyclass(name = "Reader")]
pub struct PyReader {
    inner: Reader,
}

#[pymethods]
impl PyReader {
    #[new]
    pub fn new(filepath: &str) -> PyResult<Self> {
        let file = File::open(filepath).map_err(|e| PyErr::new::<PyIOError, _>(e.to_string()))?;
        let inner = Reader::new(file).map_err(|e| PyErr::new::<PyIOError, _>(e.to_string()))?;
        Ok(PyReader { inner })
    }

    #[getter]
    fn project(&self) -> PyResult<PyProject> {
        let (project, problems) = self
            .inner
            .project()
            .map_err(|e| PyErr::new::<PyIOError, _>(e.to_string()))?;

        if !problems.is_empty() {
            println!("Warnings while reading project: {:?}", problems);
        }

        Ok(PyProject { inner: project })
    }

    fn get_file_info(&self) -> PyResult<PyFileInfo> {
        let (project, problems) = self
            .inner
            .project()
            .map_err(|e| PyErr::new::<PyIOError, _>(e.to_string()))?;

        if !problems.is_empty() {
            println!("Warnings while reading project: {:?}", problems);
        }

        Ok(PyFileInfo {
            project_name: project.name,
            project_description: project.description,
            version: self.inner.version(),
        })
    }
}

#[pyclass]
pub struct PyFileInfo {
    version: [u32; 2],
    project_name: String,
    project_description: String,
}

#[pymethods]
impl PyFileInfo {
    #[getter]
    fn project_name(&self) -> PyResult<String> {
        Ok(self.project_name.clone())
    }
    #[getter]
    fn project_description(&self) -> PyResult<String> {
        Ok(self.project_description.clone())
    }
    #[getter]
    fn version(&self) -> PyResult<(u32, u32)> {
        Ok(self.version.into())
    }
}
