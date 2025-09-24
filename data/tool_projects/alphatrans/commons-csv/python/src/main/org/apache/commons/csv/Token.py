from __future__ import annotations
import re
from io import StringIO
import io
import typing
from typing import *


class Token:

    content: io.StringIO = io.StringIO(" " * 50)
    __INITIAL_TOKEN_LENGTH: int = 50
    isQuoted: bool = False

    isReady: bool = False

    type: typing.Type = None

    @staticmethod
    def initialize_fields() -> None:
        type: typing.Type = Type.INVALID

    def toString(self) -> str:
        return f"{self.type.name()} [{self.content.getvalue()}]"

    def reset(self) -> None:
        self.content = io.StringIO(" " * 50)  # Reset content to an empty StringIO
        self.type = Type.INVALID
        self.isReady = False
        self.isQuoted = False


class Type:

    COMMENT: typing.Type = None

    EORECORD: typing.Type = None

    EOF: typing.Type = None

    TOKEN: typing.Type = None

    INVALID: typing.Type = None


Token.initialize_fields()
