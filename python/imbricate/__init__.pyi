from typing import Any, List
from ._internal import sample as sample

class Date:
    """A date."""

    def __init__(self) -> None:
        self.day: int = ...
        self.month: int = ...
        self.year: int = ...

class Address:
    """An address."""

    def __init__(self) -> None:
        self.street: str = ...
        self.city: str = ...
        self.state: str = ...
        self.zip_code: str = ...
        self.updated_at: Date = ...

class Person:
    """A persom."""

    def __init__(self) -> None:
        self.name: str = ...
        self.address: Address = ...
        self.created_at: Date = ...
        self.updated_at: Date = ...

class Item:
    def __init__(self) -> None:
        self.name: str = ...
        self.code: str = ...
        self.quantity: int = ...
        self.producer: Person = ...

class Order:
    def __init__(self) -> None:
        self.placed: Date = ...
        self.items: List[Item] = ...
        self.customer: Person = ...
        self.clerk: Person = ...
        self.comments: str = ...

def fake_order() -> Order: ...
def create_json(count: int) -> None: ...