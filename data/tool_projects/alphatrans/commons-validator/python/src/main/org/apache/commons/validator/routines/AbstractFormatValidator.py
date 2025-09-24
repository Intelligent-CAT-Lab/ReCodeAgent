from __future__ import annotations
import locale
import re
import os
from abc import ABC
import io
import typing
from typing import *


class AbstractFormatValidator(ABC):

    __strict: bool = False

    __serialVersionUID: int = -4690687565200568258

    def _parse(self, value: str, formatter: Format) -> typing.Any:
        pos = ParsePosition(0)
        parsed_value = formatter.parseObject(value, pos)

        if pos.getErrorIndex() > -1:
            return None

        if self.isStrict() and pos.getIndex() < len(value):
            return None

        if parsed_value is not None:
            parsed_value = self._processParsedValue(parsed_value, formatter)

        return parsed_value

    def _format4(self, value: typing.Any, formatter: Format) -> str:
        return formatter.format(value)

    def format3(self, value: typing.Any, pattern: str, locale: typing.Any) -> str:
        formatter = self._getFormat(pattern, locale)
        return self._format4(value, formatter)

    def format2(self, value: typing.Any, locale: typing.Any) -> str:
        return self.format3(value, None, locale)

    def format1(self, value: typing.Any, pattern: str) -> str:
        return self.format3(value, pattern, None)

    def format0(self, value: typing.Any) -> str:
        return self.format3(value, None, None)

    def isValid2(self, value: str, locale: typing.Any) -> bool:
        return self.isValid3(value, None, locale)

    def isValid1(self, value: str, pattern: str) -> bool:
        return self.isValid3(value, pattern, None)

    def isValid0(self, value: str) -> bool:
        return self.isValid3(value, None, None)

    def isStrict(self) -> bool:
        return self.__strict

    def __init__(self, strict: bool) -> None:
        self.__strict = strict

    def _getFormat(self, pattern: str, locale: typing.Any) -> Format:
        # This method is abstract and should be implemented by subclasses
        raise NotImplementedError("Subclasses must implement this method")

    def _processParsedValue(self, value: typing.Any, formatter: Format) -> typing.Any:
        # This method is abstract in Java, so it should raise a NotImplementedError in Python
        raise NotImplementedError("Subclasses must implement this method")

    def isValid3(self, value: str, pattern: str, locale: typing.Any) -> bool:
        raise NotImplementedError("Subclasses must implement this method")
