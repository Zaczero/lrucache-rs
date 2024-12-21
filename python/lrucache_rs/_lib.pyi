from collections.abc import Hashable, Iterator, MutableMapping
from typing import TypeVar, overload

_K = TypeVar('_K', bound=Hashable)
_V = TypeVar('_V')
_D = TypeVar('_D')

class LRUCache(MutableMapping[_K, _V]):
    def __init__(self, maxsize: int) -> None:
        """
        Initialize the LRUCache with a specified maximum size.
        """

    def __len__(self) -> int:
        """
        Return the number of items currently in the cache.
        """

    def __contains__(self, key: object, /) -> bool:
        """
        Check if the given key is present in the cache.
        """

    def __iter__(self) -> Iterator[_K]:
        """
        Return an iterator over the keys in the cache.
        """

    def __setitem__(self, key: _K, value: _V, /) -> None:
        """
        Add or update the value for a given key in the cache.

        If the cache reaches its maximum size, the least recently used item is automatically evicted.
        """

    def __getitem__(self, key: _K, /) -> _V:
        """
        Retrieve the value associated with the given key.

        :raises KeyError: If the key is not found.
        """

    def __delitem__(self, key: _K, /) -> None:
        """
        Remove the value associated with the given key.

        :raises KeyError: If the key is not found.
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
