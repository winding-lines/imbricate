//! Classes to support the sample parquet file.
//!
//! The intent is to generate a complex nested structure.
use fake::faker::address::en::{CityName, StateName, StreetName, ZipCode};
use fake::faker::company::en::Buzzword;
use fake::faker::name::en::Name;
use fake::{Dummy, Fake};
use pyo3::{pyclass, pymethods, types::PyModule, PyResult};
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Clone, Dummy, Debug, Serialize, Deserialize)]
pub struct Date {
    #[pyo3(get, set)]
    #[dummy(faker = "1..30")]
    pub day: u32,
    #[pyo3(get, set)]
    #[dummy(faker = "1..12")]
    pub month: u32,
    #[pyo3(get, set)]
    #[dummy(faker = "2010..2023")]
    pub year: u32,
}

#[pymethods]
impl Date {
    #[new]
    fn new(day: u32, month: u32, year: u32) -> Self {
        Date {
            day: day,
            month: month,
            year: year,
        }
    }
}

#[pyclass]
#[derive(Clone, Debug, Dummy, Serialize, Deserialize)]
pub struct Address {
    #[pyo3(get, set)]
    #[dummy(faker = "StreetName()")]
    pub street: String,
    #[pyo3(get, set)]
    #[dummy(faker = "CityName()")]
    pub city: String,
    #[pyo3(get, set)]
    #[dummy(faker = "StateName()")]
    pub state: String,
    #[pyo3(get, set)]
    #[dummy(faker = "ZipCode()")]
    pub zip_code: String,
    #[pyo3(get, set)]
    pub updated_at: Date,
}

#[pyclass]
#[derive(Clone, Debug, Dummy, Serialize, Deserialize)]
pub struct Person {
    #[pyo3(get, set)]
    #[dummy(faker = "Name()")]
    pub name: String,
    #[pyo3(get, set)]
    pub address: Address,
    #[pyo3(get, set)]
    pub created_at: Date,
    #[pyo3(get, set)]
    pub updated_at: Date,
}

#[pyclass]
#[derive(Clone, Debug, Dummy, Serialize, Deserialize)]
pub struct Item {
    #[pyo3(get, set)]
    #[dummy(faker = "Buzzword()")]
    pub name: String,
    #[pyo3(get, set)]
    pub code: String,
    #[pyo3(get, set)]
    #[dummy(faker = "1..10")]
    pub quantity: u32,
    #[pyo3(get, set)]
    pub producer: Person,
}

#[pyclass]
#[derive(Clone, Debug, Dummy, Serialize, Deserialize)]
pub struct Order {
    #[pyo3(get, set)]
    pub placed: Date,
    #[pyo3(get, set)]
    pub items: Vec<Item>,
    #[pyo3(get, set)]
    pub customer: Person,
    #[pyo3(get, set)]
    pub clerk: Person,
    #[pyo3(get, set)]
    pub comments: String,
}

#[pymethods]
impl Order {
    fn __str__(&self) -> String {
        format!("{:?}", self)
    }
}

pub(crate) fn init_module(m: &PyModule) -> PyResult<()> {
    m.add_class::<Date>()?;
    m.add_class::<Address>()?;
    m.add_class::<Person>()?;
    m.add_class::<Item>()?;
    m.add_class::<Order>()?;
    Ok(())
}
