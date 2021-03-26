#![allow(clippy::needless_return)]

use pyo3::prelude::*;

use std::cell::RefCell;

use ndarray::ArrayView2;
use numpy::{PyArray1, PyArray2};

#[pyclass]
pub struct VoronoiDecomposer {
    /// This is inherently unsafe since we are tricking the rust compiler into
    /// thinking that the reference is a `'static` one when the array could
    /// actually be changed under us at any time.
    decomposer: RefCell<voronoi_fps::VoronoiDecomposer<'static>>,
}

#[pymethods]
impl VoronoiDecomposer {
    #[new]
    fn new(points: &PyArray2<f64>, initial: usize) -> Self {
        let points = points.readonly();

        let points: ArrayView2<'static, f64> = unsafe {
            std::mem::transmute(points.as_array())
        };

        VoronoiDecomposer {
            decomposer: RefCell::new(voronoi_fps::VoronoiDecomposer::new(points, initial)),
        }
    }

    fn add_point(&self, new_point: usize) -> f64 {
        let mut decomposer = self.decomposer.borrow_mut();
        decomposer.add_point(new_point);
        return *decomposer.cells().last().unwrap().radius2;
    }

    fn next_point(&self) -> (usize, f64) {
        let decomposer = self.decomposer.borrow();
        return decomposer.next_point();
    }

    fn radius2<'a>(&self, py: Python<'a>) -> &'a PyArray1<f64> {
        let decomposer = self.decomposer.borrow();
        let r2 = decomposer.cells().radius2;

        return PyArray1::from_slice(py, r2);
    }
}


#[pymodule]
fn voronoi_fps(_: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "select_fps_voronoi")]
    fn select_fps_voronoi<'py>(py: Python<'py>, points: &PyArray2<f64>, n_select: usize, initial: usize) -> &'py PyArray1<usize> {
        let points = points.readonly();
        let selected = voronoi_fps::voronoi::select_fps(points.as_array(), n_select, initial);
        return PyArray1::from_vec(py, selected);
    }

    #[pyfn(m, "select_fps_simple")]
    fn select_fps_simple<'py>(py: Python<'py>, points: &PyArray2<f64>, n_select: usize, initial: usize) -> &'py PyArray1<usize> {
        let points = points.readonly();
        let selected = voronoi_fps::simple::select_fps(points.as_array(), n_select, initial);
        return PyArray1::from_vec(py, selected);
    }

    m.add_class::<VoronoiDecomposer>()?;
    Ok(())
}