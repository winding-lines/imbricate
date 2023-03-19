use arrow::pyarrow::PyArrowConvert;
use arrow::record_batch::RecordBatch;
use parquet::arrow::arrow_reader::{ParquetRecordBatchReader, ParquetRecordBatchReaderBuilder};
use pyo3::{pyclass, pymethods, PyObject, PyRef, PyRefMut, PyResult, Python};
use std::fs::File;

use crate::{sample::Order, sample_serde::OrderStructArray};

#[pyclass]
pub struct BatchDeserializer {
    batch: OrderStructArray,
    pos: usize,
    len: usize,
}

#[pymethods]
impl BatchDeserializer {
    #[new]
    pub fn new(some: PyObject, py: Python) -> PyResult<Self> {
        let some = some.as_ref(py);
        let rb = RecordBatch::from_pyarrow(some)?;
        let batch = OrderStructArray::new(&rb);
        Ok(BatchDeserializer {
            batch,
            pos: 0,
            len: rb.num_rows(),
        })
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<Order> {
        if slf.pos >= slf.len {
            return None;
        }

        let order = slf.batch.value(slf.pos);
        slf.pos += 1;
        Some(order)
    }
}

// Read parquet file using a Arrow reader.
// This is slow and needs to be rewritten using the lower level API and rayon.
#[pyclass]
pub struct ArrowReader {
    batch_reader: ParquetRecordBatchReader,
}

#[pymethods]
impl ArrowReader {
    #[new]
    pub fn new(path: &str) -> Self {
        let file = File::open(path).unwrap();

        let builder = ParquetRecordBatchReaderBuilder::try_new(file).unwrap();

        let reader = builder.build().unwrap();

        ArrowReader {
            batch_reader: reader,
        }
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<BatchDeserializer> {
        let next = slf.batch_reader.next();
        if next.is_none() {
            return None;
        }
        let record_batch = next.unwrap().unwrap();
        Some(BatchDeserializer {
            batch: OrderStructArray::new(&record_batch),
            pos: 0,
            len: record_batch.num_rows(),
        })
    }
}
