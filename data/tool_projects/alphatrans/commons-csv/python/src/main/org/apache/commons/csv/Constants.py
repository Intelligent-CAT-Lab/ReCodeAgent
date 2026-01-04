from __future__ import annotations
import re
import io
import typing
from typing import *


class Constants:

    US: str = chr(31)
    UNDEFINED: int = -2
    TAB: str = None  # LLM could not translate this field

    SQL_NULL_STRING: str = "\\N"
    SP: str = " "
    RS: str = None  # LLM could not translate this field

    PIPE: str = "|"
    PARAGRAPH_SEPARATOR: str = "\u2029"
    NEXT_LINE: str = "\u0085"
    LINE_SEPARATOR: str = "\u2028"
    LF: str = None  # LLM could not translate this field

    FF: str = "\f"
    END_OF_STREAM: int = -1
    EMPTY_STRING_ARRAY: typing.List[str] = []
    EMPTY: str = None  # LLM could not translate this field

    DOUBLE_QUOTE_CHAR: str = '"'
    CRLF: str = None  # LLM could not translate this field

    CR: str = "\r"
    COMMENT: str = "#"
    COMMA: str = ","
    BACKSPACE: str = "\b"
    BACKSLASH: str = "\\"

    def __init__(self) -> None:
        raise AssertionError("Constants class cannot be instantiated")
