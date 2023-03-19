//! Support classes to serialize and deserialize the sample data.
//! Thanks to Github copilot for writing the boilerplate part of the code.

use arrow::{
    array::{Int64Array, ListArray, StringArray, StructArray},
    record_batch::RecordBatch,
};

use crate::sample::{Address, Date, Item, Order};

pub struct DateStructArray {
    day: Int64Array,
    month: Int64Array,
    year: Int64Array,
}

impl DateStructArray {
    pub fn new(placed: &StructArray) -> Self {
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

    pub fn value(&self, index: usize) -> Date {
        Date {
            day: self.day.value(index) as u32,
            month: self.month.value(index) as u32,
            year: self.year.value(index) as u32,
        }
    }
}

struct AddressStructArray {
    street: StringArray,
    city: StringArray,
    state: StringArray,
    zip_code: StringArray,
    updated_at: DateStructArray,
}

impl AddressStructArray {
    pub fn new(address: &StructArray) -> Self {
        let streets = address
            .column_by_name("street")
            .unwrap()
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();
        let cities = address
            .column_by_name("city")
            .unwrap()
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();
        let states = address
            .column_by_name("state")
            .unwrap()
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();
        let zip_codes = address
            .column_by_name("zip_code")
            .unwrap()
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();
        let updated_ats = address
            .column_by_name("updated_at")
            .unwrap()
            .as_any()
            .downcast_ref::<StructArray>()
            .unwrap();
        let updated_ats = DateStructArray::new(updated_ats);

        AddressStructArray {
            street: streets.clone(),
            city: cities.clone(),
            state: states.clone(),
            zip_code: zip_codes.clone(),
            updated_at: updated_ats,
        }
    }

    pub fn value(&self, index: usize) -> Address {
        Address {
            street: self.street.value(index).to_string(),
            city: self.city.value(index).to_string(),
            state: self.state.value(index).to_string(),
            zip_code: self.zip_code.value(index).to_string(),
            updated_at: self.updated_at.value(index),
        }
    }
}

pub struct PersonStructArray {
    name: StringArray,
    address: AddressStructArray,
    created_at: DateStructArray,
    updated_at: DateStructArray,
}

impl PersonStructArray {
    pub fn new(person: &StructArray) -> Self {
        let names = person
            .column_by_name("name")
            .unwrap()
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();
        let addresses = person
            .column_by_name("address")
            .unwrap()
            .as_any()
            .downcast_ref::<StructArray>()
            .unwrap();
        let addresses = AddressStructArray::new(addresses);
        let created_ats = person
            .column_by_name("created_at")
            .unwrap()
            .as_any()
            .downcast_ref::<StructArray>()
            .unwrap();
        let created_ats = DateStructArray::new(created_ats);
        let updated_ats = person
            .column_by_name("updated_at")
            .unwrap()
            .as_any()
            .downcast_ref::<StructArray>()
            .unwrap();
        let updated_ats = DateStructArray::new(updated_ats);

        PersonStructArray {
            name: names.clone(),
            address: addresses,
            created_at: created_ats,
            updated_at: updated_ats,
        }
    }

    pub fn value(&self, index: usize) -> crate::sample::Person {
        crate::sample::Person {
            name: self.name.value(index).to_string(),
            address: self.address.value(index),
            created_at: self.created_at.value(index),
            updated_at: self.updated_at.value(index),
        }
    }
}

pub struct ItemStructArray {
    name: StringArray,
    code: StringArray,
    quantity: Int64Array,
    producer: StructArray,
}

impl ItemStructArray {
    pub fn new(item: &StructArray) -> Self {
        let names = item
            .column_by_name("name")
            .unwrap()
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();
        let codes = item
            .column_by_name("code")
            .unwrap()
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();
        let quantities = item
            .column_by_name("quantity")
            .unwrap()
            .as_any()
            .downcast_ref::<Int64Array>()
            .unwrap();
        let producers = item
            .column_by_name("producer")
            .unwrap()
            .as_any()
            .downcast_ref::<StructArray>()
            .unwrap();

        ItemStructArray {
            name: names.clone(),
            code: codes.clone(),
            quantity: quantities.clone(),
            producer: producers.clone(),
        }
    }

    pub fn value(&self, index: usize) -> Item {
        let person = PersonStructArray::new(&self.producer);
        Item {
            name: self.name.value(index).to_string(),
            code: self.code.value(index).to_string(),
            quantity: self.quantity.value(index) as u32,
            producer: person.value(index),
        }
    }
}

pub struct OrderStructArray {
    placed: DateStructArray,
    items: ListArray,
    customer: PersonStructArray,
    clerk: PersonStructArray,
    comments: StringArray,
}

impl OrderStructArray {
    pub fn new(order: &RecordBatch) -> Self {
        let placed = order
            .column_by_name("placed")
            .unwrap()
            .as_any()
            .downcast_ref::<StructArray>()
            .unwrap();
        let placed = DateStructArray::new(placed);
        let items = order
            .column_by_name("items")
            .unwrap()
            .as_any()
            .downcast_ref::<ListArray>()
            .unwrap();
        let customer = order
            .column_by_name("customer")
            .unwrap()
            .as_any()
            .downcast_ref::<StructArray>()
            .unwrap();
        let customer = PersonStructArray::new(customer);
        let clerk = order
            .column_by_name("clerk")
            .unwrap()
            .as_any()
            .downcast_ref::<StructArray>()
            .unwrap();
        let clerk = PersonStructArray::new(clerk);
        let comments = order
            .column_by_name("comments")
            .unwrap()
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();

        OrderStructArray {
            placed,
            items: items.clone(),
            customer,
            clerk,
            comments: comments.clone(),
        }
    }

    pub fn value(&self, index: usize) -> Order {
        let items = self.items.value(index);
        let len = items.len();
        let items = items.as_any().downcast_ref::<StructArray>().unwrap();
        let items = ItemStructArray::new(items);

        Order {
            placed: self.placed.value(index),
            items: (0..len).map(|i| items.value(i)).collect::<Vec<Item>>(),
            customer: self.customer.value(index),
            clerk: self.clerk.value(index),
            comments: self.comments.value(index).to_string(),
        }
    }
}
