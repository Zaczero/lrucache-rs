from collections.abc import Hashable
from typing import Generic, TypeVar, overload

_K = TypeVar('_K', bound=Hashable)
_V = TypeVar('_V')
_D = TypeVar('_D')

class LRUCache(Generic[_K, _V]):
    def __init__(self, maxsize: int) -> None:
        """
        Initialize the LRUCache with a specified maximum size.
        """

    def __setitem__(self, key: _K, value: _V) -> None:
        """
        Add or update the value for a given key in the cache.

        If the cache reaches its maximum size, the least recently used item is automatically evicted.
        """

    @overload
    def get(self, key: _K, /) -> _V | None: ...
    @overload
    def get(self, key: _K, /, default: _D) -> _V | _D: ...
    def get(self, key: _K, /, default: _D | None = None) -> _V | _D | None:
        """
        Retrieve the value associated with the given key.

        If the key is not found, the default value is returned.
        """
