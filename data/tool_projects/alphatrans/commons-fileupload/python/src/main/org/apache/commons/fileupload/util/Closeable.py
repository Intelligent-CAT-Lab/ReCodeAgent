from __future__ import annotations
import re
from abc import ABC
import io
import os


class Closeable(ABC):

    def isClosed(self) -> bool:
        raise io.UnsupportedOperation("isClosed")

    def close(self) -> None:
        raise NotImplementedError()
