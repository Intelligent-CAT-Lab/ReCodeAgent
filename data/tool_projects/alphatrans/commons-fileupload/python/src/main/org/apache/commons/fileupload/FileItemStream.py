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
        pass

    def getFieldName(self) -> str:
        pass

    def getName(self) -> str:
        pass

    def getContentType(self) -> str:
        pass

    def openStream(self) -> typing.Union[io.BytesIO, io.StringIO, io.BufferedReader]:
        raise NotImplementedError("Subclasses must implement openStream()")


class ItemSkippedException:

    __serialVersionUID: int = -7280778431581963740
