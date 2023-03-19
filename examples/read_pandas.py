from time import monotonic
from pyarrow.parquet import read_pandas
from dataclasses import dataclass

# Replicate the class hierarcy as dataclasses.


@dataclass
class Date:
    year: int
    month: int
    day: int


@dataclass
class Address:
    street: str
    city: str
    state: str
    zip_code: str
    updated: Date


@dataclass
class Person:
    name: str
    address: Address
    created_at: Date
    updated_at: Date


@dataclass
class Item:
    name: str
    code: str
    quantity: int
    producer: Person


@dataclass
class Order:
    placed: Date
    items: list[Item]
    customer: Person
    clerk: Person
    comments: str


def deser_order(data):
    return Order(
        placed=Date(**data["placed"]),
        items=[Item(**item) for item in data["items"]],
        customer=Person(**data["customer"]),
        clerk=Person(**data["clerk"]),
        comments=data["comments"],
    )


def run():
    df = read_pandas("../out.parquet").to_pandas()
    count = 0
    len = df.shape[0]
    for i in range(len):
        count += 1
        data = df.iloc[i]
        order = deser_order(data)
        if count == 1:
            print(repr(order))
        pass
    return count


if __name__ == "__main__":
    start = monotonic()
    count = run()
    end = monotonic()
    print(f"Panda read {count} records in {end - start:.2f} seconds")
