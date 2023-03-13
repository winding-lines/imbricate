mod sample;

use fake::{Fake, Faker};

use pyo3::prelude::*;
use sample::*;

#[pyfunction]
fn fake_order() -> PyResult<Order> {
    let order: Order = Faker.fake();
    Ok(order)
}

#[pyfunction]
fn create_json(count: usize) {
    for _ in 0..count {
        let order: Order = Faker.fake();
        let json = serde_json::to_string(&order).unwrap();

        println!("{}", json);
    }
}

/// Low-level Imbricate internal package.
///
/// The internal module exported from python in `python/__init__.py`.
#[pymodule]
fn _internal(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fake_order, m)?)?;
    m.add_function(wrap_pyfunction!(create_json, m)?)?;

    // let sample = PyModule::new(_py, "sample")?;
    // sample::init_module(sample)?;
    // m.add_submodule(sample)?;
    sample::init_module(m)?;
    Ok(())
}