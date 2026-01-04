from __future__ import annotations
import re
from io import StringIO
import io
import typing
from typing import *


class Token:

    content: typing.Union[typing.List[str], io.StringIO] = None  # LLM could not translate this field

    __INITIAL_TOKEN_LENGTH: int = 50
    isQuoted: bool = False

    isReady: bool = False

    type: typing.Type = None  # LLM could not translate this field

    def toString(self) -> str:
        content_str = "".join(self.content) if isinstance(self.content, list) else self.content.getvalue()
        return f"{self.type.__name__} [{content_str}]"

    def reset(self) -> None:
        self.content.clear()
        self.type = Type.INVALID
        self.isReady = False
        self.isQuoted = False


class Type:

    COMMENT: typing.Type = None

    EORECORD: typing.Type = None

    EOF: typing.Type = None

    TOKEN: typing.Type = None

    INVALID: typing.Type = None
