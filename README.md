# lrucache-rs

[![PyPI - Python Version](https://shields.monicz.dev/pypi/pyversions/lrucache-rs)](https://pypi.org/project/lrucache-rs)
[![Liberapay Patrons](https://shields.monicz.dev/liberapay/patrons/Zaczero?logo=liberapay&label=Patrons)](https://liberapay.com/Zaczero/)
[![GitHub Sponsors](https://shields.monicz.dev/github/sponsors/Zaczero?logo=github&label=Sponsors&color=%23db61a2)](https://github.com/sponsors/Zaczero)

An efficient LRU cache written in Rust with Python bindings. Unlike other LRU cache implementations, this one behaves like a Python dictionary and does not wrap around a function.

## Installation

The recommended installation method is through the PyPI package manager. The project is implemented in Rust and several pre-built binary wheels are available for Linux, macOS, and Windows, with support for both x64 and ARM architectures.

```sh
pip install lrucache-rs
```

## Basic usage

```py
from lrucache_rs import LRUCache

cache: LRUCache[str, int] = LRUCache(maxsize=2)
cache['1'] = 1
cache['2'] = 2
cache['3'] = 3
assert cache.get('1') is None
assert cache.get('2') == 2
assert cache.get('3') == 3
```
