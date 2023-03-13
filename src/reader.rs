use arrow::{
    array::{Int64Array, StringArray, StructArray},
    record_batch::RecordBatch,
};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use pyo3::{pyfunction, PyErr, PyResult};
use std::fs::File;

use crate::sample::{Date, Order};

struct DateStructArray {
    day: Int64Array,
    month: Int64Array,
    year: Int64Array,
}

impl DateStructArray {
    fn new(placed: &StructArray) -> Self {
        let days = placed
            .column_by_name("day")
            .unwrap()
            .as_any()
            .downcast_ref::<Int64Array>()
            .unwrap();
        let months = placed
            .column_by_name("month")
            .unwrap()
            .as_any()
            .downcast_ref::<Int64Array>()
            .unwrap();
        let years = placed
            .column_by_name("year")
            .unwrap()
            .as_any()
            .downcast_ref::<Int64Array>()
            .unwrap();

        DateStructArray {
            day: days.clone(),
            month: months.clone(),
            year: years.clone(),
        }
    }

    fn value(&self, index: usize) -> Date {
        Date {
            day: self.day.value(index) as u32,
            month: self.month.value(index) as u32,
            year: self.year.value(index) as u32,
        }
    }
}

fn deser_order(record_batch: RecordBatch) -> PyResult<Order> {
    let comments = record_batch
        .column_by_name("comments")
        .unwrap()
        .as_any()
        .downcast_ref::<StringArray>()
        .unwrap();
    println!("first comment {:?}", comments.value(0));

    // Deserialize the "placed" column into a Date struct.
    let placed = record_batch
        .column_by_name("placed")
        .unwrap()
        .as_any()
        .downcast_ref::<StructArray>()
        .unwrap();
    let placed = DateStructArray::new(placed);

    let order = Order {
        items: vec![],
        placed: placed.value(0),
        comments: comments.value(0).to_string(),
    };

    Ok(order)
}

#[pyfunction]
pub fn read_parquet(path: &str) -> PyResult<Order> {
    let file = File::open(path).unwrap();

    let builder = ParquetRecordBatchReaderBuilder::try_new(file).unwrap();

    let schema = builder.schema();
    let mut reader = builder.build().unwrap();

    let record_batch = reader.next().unwrap().unwrap();

    println!(
        "Read first batch, has {} records, memory size {}.",
        record_batch.num_rows(),
        record_batch.get_array_memory_size()
    );

    Ok(deser_order(record_batch))
}
