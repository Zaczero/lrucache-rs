import pytest
from lrucache_rs import LRUCache


def test_invalid_maxsize():
    with pytest.raises(OverflowError):
        LRUCache(-1)
    with pytest.raises(ValueError):
        LRUCache(0)


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


def test_len_and_delitem():
    cache: LRUCache[str, int] = LRUCache(2)
    assert len(cache) == 0
    cache['1'] = 1
    cache['2'] = 2
    assert len(cache) == 2
    cache['3'] = 3
    assert len(cache) == 2
    del cache['3']
    assert len(cache) == 1
    del cache['2']
    assert len(cache) == 0
    with pytest.raises(KeyError, match="'1'"):
        del cache['1']


def test_contains():
    cache: LRUCache[str, int] = LRUCache(2)
    assert '1' not in cache
    cache['1'] = 1
    assert '1' in cache
    del cache['1']
    assert '1' not in cache


def test_iter():
    cache: LRUCache[str, int] = LRUCache(2)
    cache['1'] = 1
    cache['2'] = 2
    iterator = iter(cache)
    del cache['1']
    del cache['2']
    assert len(cache) == 0
    assert next(iterator) == '1'
    assert next(iterator) == '2'
    with pytest.raises(StopIteration):
        next(iterator)


def test_getitem():
    cache: LRUCache[str, int] = LRUCache(2)
    cache['1'] = 1
    cache['2'] = 2
    assert cache['1'] == 1
    assert cache['2'] == 2
    with pytest.raises(KeyError, match="'3'"):
        cache['3']
