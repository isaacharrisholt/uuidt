# UUIDT

Timestamp-orderable UUIDs for Python, written in Rust.

## Installation

```bash
pip install uuidt
```

## Usage

```python
import uuidt

# Create a new UUIDT
u = uuidt.new('my-namespace')

# Print the UUIDT properties
print(u.namespace)
print(u.timestamp)
print(u.hostname)
print(u.random_chars)

# Convert to a string
print(str(u))

# Extract the timestamp from a UUIDT string
print(uuidt.extract_timestamp("cr3su3qh-4ium-00bk-00ip-vqlpgpomk3dv"))
# 1678562753992474990
```

## Motivation

UUIDs are great for generating unique identifiers, but they are not
necessarily time-orderable. This is a problem if you want to generate a UUID
for a new record in a database, and then use that UUID to order the records
by creation time.

Many databases avoid this problem using auto-incrementing integer IDs, but
this isn't possible in distributed databases like
[CockroachDB](https://www.cockroachlabs.com/), so a UUID is typically used as
the primary key instead.

This library generates UUIDs that are time-orderable. The first 12 alphanumeric
characters of the UUID are a nanosecond-precision timestamp which has been
base-36 encoded, so they can be sorted lexicographically. The remaining 20
characters are a combination of a namespace, the hostname of the machine that
generated the UUID, and a random string.

Technically, UUID1s are also time-orderable, but they are not guaranteed to
be ordered by creation time, and it can be difficult to extract the
timestamp from a UUID1.

## Why Rust?

Mostly as a learning opportunity for me, though also for speed. The Rust
implementation is significantly faster than the Python implementation, which
used Numpy to convert to base-36.

### What if I don't want the Rust implementation?

UUIDT should be installable as a wheel on most systems, but if you hate Ferris
and want to use the Python implementation instead, here's some equivalent
code:

```python
import random
import socket
import time

import numpy as np

BASE = 36
DIVISOR = BASE - 1
CHARACTERS = list('0123456789abcdefghijklmnopqrstuvwxyz')[:BASE]


class UUIDT:
    def __init__(self, namespace: str, timestamp: int, hostname: str, random_chars: str):
        self.namespace = namespace
        self.timestamp = timestamp
        self.hostname = hostname
        self.random_chars = random_chars

    def __str__(self):
        hostname_enc = sum(self.hostname.encode('utf-8'))
        namespace_enc = sum(self.namespace.encode('utf-8'))

        timestamp_str = np.base_repr(self.timestamp, 36).lower()
        hostname_str = np.base_repr(hostname_enc, 36).lower()
        namespace_str = np.base_repr(namespace_enc, 36).lower()

        return (
            f'{timestamp_str[:8]}-{timestamp_str[8:]}-{hostname_str:0>4}-'
            f'{namespace_str:0>4}-{self.random_chars}'
        )


def new(namespace: str) -> UUIDT:
    timestamp = time.time_ns()
    hostname = socket.gethostname()
    random_chars = ''.join(random.choices(CHARACTERS, k=4))

    return UUIDT(namespace, timestamp, hostname, random_chars)
```

## License

MIT

## Using UUIDT in your project

While UUIDT is MIT licensed, I'm really curious to seeing the projects that
use it! If you use UUIDT in your project, I'd love to hear about it! Please
let me know by either opening an issue or sending me an email at the address
in the `pyproject.toml` file.

## Contributing

Contributions are welcome! Just open an issue or a pull request.
