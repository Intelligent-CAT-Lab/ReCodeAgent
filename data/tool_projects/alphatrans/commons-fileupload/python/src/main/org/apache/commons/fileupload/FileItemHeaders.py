from __future__ import annotations
import re
from abc import ABC
import io
import typing
from typing import *


class FileItemHeaders(ABC):

    def getHeaderNames(self) -> typing.Iterator[str]:
        return iter([])

    def getHeaders(self, name: str) -> typing.Iterator[str]:
        # Implementation would go here
        pass

    def getHeader(self, name: str) -> str:
        return name  # Replace this with the actual implementation logic
