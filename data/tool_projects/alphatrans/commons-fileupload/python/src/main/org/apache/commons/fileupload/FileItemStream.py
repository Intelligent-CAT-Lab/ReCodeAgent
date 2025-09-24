from __future__ import annotations
import re
from abc import ABC
from io import StringIO
import io
from io import BytesIO
import typing
from typing import *
from src.main.org.apache.commons.fileupload.FileItemHeadersSupport import *


class FileItemStream(ABC):

    def isFormField(self) -> bool:
        return False  # Replace with the actual logic if needed

    def getFieldName(self) -> str:
        return self._field_name

    def getName(self) -> str:
        return self.name

    def getContentType(self) -> str:
        return "application/octet-stream"

    def openStream(self) -> typing.Union[io.BytesIO, io.StringIO, io.BufferedReader]:
        raise NotImplementedError("Subclasses must implement this method")


class ItemSkippedException:

    __serialVersionUID: int = -7280778431581963740
