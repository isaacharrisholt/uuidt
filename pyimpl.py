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