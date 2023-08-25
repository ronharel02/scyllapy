from typing import Any

class Select:
    def __init__(self, table: str) -> None: ...
    def only(self, *columns: str) -> Select: ...
    def filter(self, clause: str, params: list[Any]) -> Select: ...
    def group_by(self, group: str) -> Select: ...
    def order_by(self, order: str, desc: bool = False) -> Select: ...
    def per_partition_limit(self, per_partition_limit: int) -> Select: ...
    def limit(self, limit: int) -> Select: ...
    def allow_filtering(self) -> Select: ...
    def distinct(self) -> Select: ...
    def timeout(self, timeout: int | str) -> Select: ...

class Insert:
    def __init__(self, table: str) -> None: ...
    def if_not_exists(self) -> Insert: ...
    def set(self, name: str, value: Any) -> Insert: ...
    def timeout(self, timeout: int | str) -> Insert: ...
    def timestamp(self, timestamp: int) -> Insert: ...
    def ttl(self, ttl: int) -> Insert: ...
