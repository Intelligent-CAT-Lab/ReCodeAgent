from __future__ import annotations
import locale
import re
import io
import numbers
import typing
from typing import *
from src.main.org.apache.commons.validator.routines.AbstractNumberValidator import *


class ByteValidator(AbstractNumberValidator):

    __VALIDATOR: ByteValidator = None
    __serialVersionUID: int = 7001640945881854649

    @staticmethod
    def initialize_fields() -> None:
        ByteValidator.__VALIDATOR: ByteValidator = ByteValidator.ByteValidator1()

    def _processParsedValue(self, value: typing.Any, formatter: Format) -> typing.Any:
        if isinstance(value, int):  # Python's int can represent Java's Long
            if -128 <= value <= 127:
                return value  # Python does not have a separate Byte type, so we return the integer
        return None

    def maxValue1(self, value: int, max_: int) -> bool:
        return self.maxValue0(value, max_)

    def maxValue0(self, value: int, max_: int) -> bool:
        return value <= max_

    def minValue1(self, value: int, min_: int) -> bool:
        return self.minValue0(value, min_)

    def minValue0(self, value: int, min_: int) -> bool:
        return value >= min_

    def isInRange1(self, value: int, min_: int, max_: int) -> bool:
        return self.isInRange0(value, min_, max_)

    def isInRange0(self, value: int, min_: int, max_: int) -> bool:
        return min_ <= value <= max_

    def validate3(self, value: str, pattern: str, locale: typing.Any) -> int:
        return self._parse(value, pattern, locale)

    def validate2(self, value: str, locale: typing.Any) -> int:
        return self._parse(value, None, locale)

    def validate1(self, value: str, pattern: str) -> int:
        return int(self._parse(value, pattern, None))

    def validate0(self, value: str) -> int:
        return self._parse(value, None, None)

    @staticmethod
    def ByteValidator1() -> ByteValidator:
        return ByteValidator(True, AbstractNumberValidator.STANDARD_FORMAT)

    super().__init__(strict, formatType, False)

    @staticmethod
    def getInstance() -> ByteValidator:
        return ByteValidator.__VALIDATOR


ByteValidator.initialize_fields()
