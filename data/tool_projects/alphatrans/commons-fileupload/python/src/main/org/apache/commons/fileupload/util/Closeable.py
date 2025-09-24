from __future__ import annotations
import re
from abc import ABC
import io
import os


class Closeable(ABC):

    def isClosed(self) -> bool:
        raise io.OSError("Method not implemented")

    def close(self) -> None:
        raise io.UnsupportedOperation("close method not implemented")
