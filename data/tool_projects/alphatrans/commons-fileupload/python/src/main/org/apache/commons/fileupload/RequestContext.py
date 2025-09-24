from __future__ import annotations
import re
from abc import ABC
from io import StringIO
import io
from io import BytesIO
import typing
from typing import *


class RequestContext(ABC):

    def getInputStream(
        self,
    ) -> typing.Union[io.BytesIO, io.StringIO, io.BufferedReader]:
        raise NotImplementedError("Subclasses must implement this method")

    def getContentLength(self) -> int:
        return 0  # Replace with actual implementation if needed

    def getContentType(self) -> str:
        return ""

    def getCharacterEncoding(self) -> str:
        return "UTF-8"
