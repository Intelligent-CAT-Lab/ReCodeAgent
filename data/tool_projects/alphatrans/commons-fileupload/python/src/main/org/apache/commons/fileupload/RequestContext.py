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
        raise NotImplementedError

    def getContentLength(self) -> int:
        pass

    def getContentType(self) -> str:
        pass

    def getCharacterEncoding(self) -> str:
        pass
