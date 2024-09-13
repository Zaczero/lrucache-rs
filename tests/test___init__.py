import pytest
from lrucache_rs import LRUCache


def test_maxsize():
    cache: LRUCache[str, int] = LRUCache(2)
    cache['1'] = 1
    cache['2'] = 2
    cache['3'] = 3
    assert cache.get('1') is None
    assert cache.get('2') == 2
    assert cache.get('3') == 3

    cache.get('2')
    cache['4'] = 4
    assert cache.get('2') == 2
    assert cache.get('3') is None
    assert cache.get('4') == 4


def test_move_to_end():
    cache: LRUCache[str, int] = LRUCache(2)
    cache['1'] = 1
    cache['2'] = 2
    cache['3'] = 3
    assert cache.get('1') is None
    assert cache.get('2') == 2
    assert cache.get('3') == 3

    cache['2'] = 2
    cache['4'] = 4
    assert cache.get('2') == 2
    assert cache.get('3') is None
    assert cache.get('4') == 4


def test_default():
    cache: LRUCache[str, int] = LRUCache(1)
    cache['1'] = 1
    cache['2'] = 2
    assert cache.get('1') is None
    assert cache.get('1', None) is None
    not_found = object()
    assert cache.get('1', not_found) is not_found


def test_invalid_maxsize():
    with pytest.raises(OverflowError):
        LRUCache(-1)
    with pytest.raises(ValueError):
        LRUCache(0)
