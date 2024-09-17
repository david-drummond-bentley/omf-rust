/// Python bindings.
use pyo3::prelude::*;

mod element;
mod file;
mod geometry;
mod omf1;
mod project;
mod validate;

use element::PyElement;
use file::reader::{PyFileInfo, PyLimits, PyReader};
use geometry::{PyGeometry, PyPointSet};
use omf1::converter::{detect_open, PyConverter};
use project::PyProject;

#[pymodule]
fn omf_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyElement>()?;
    m.add_class::<PyGeometry>()?;
    m.add_class::<PyPointSet>()?;
    m.add_class::<PyProject>()?;
    m.add_class::<PyReader>()?;
    m.add_class::<PyFileInfo>()?;
    m.add_class::<PyLimits>()?;

    let omf1_submodule = PyModule::new_bound(m.py(), "omf1")?;
    omf1_submodule.add_function(wrap_pyfunction!(detect_open, m)?)?;
    omf1_submodule.add_class::<PyConverter>()?;
    m.add_submodule(&omf1_submodule)?;

    Ok(())
}
