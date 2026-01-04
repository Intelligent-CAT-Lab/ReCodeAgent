from __future__ import annotations
import locale
import re
import io
import numbers
import typing
from typing import *
from src.main.org.apache.commons.validator.routines.AbstractNumberValidator import *


class BigIntegerValidator(AbstractNumberValidator):

    __VALIDATOR: BigIntegerValidator = None  # LLM could not translate this field

    __serialVersionUID: int = 6713144356347139988

    def _processParsedValue(self, value: typing.Any, formatter: Format) -> typing.Any:
        return int(value)

    def maxValue(self, value: int, max_: int) -> bool:
        return value <= max_

    def minValue(self, value: int, min_: int) -> bool:
        return value >= min_

    def isInRange(self, value: int, min_: int, max_: int) -> bool:
        return min_ <= value <= max_

    def validate3(self, value: str, pattern: str, locale: typing.Any) -> int:

        pass  # LLM could not translate this method

    def validate2(self, value: str, locale: typing.Any) -> int:

        pass  # LLM could not translate this method

    def validate1(self, value: str, pattern: str) -> int:
        return self._parse(value, pattern, None)

    def validate0(self, value: str) -> int:

        pass  # LLM could not translate this method

    @staticmethod
    def BigIntegerValidator1() -> BigIntegerValidator:
        return BigIntegerValidator(True, AbstractNumberValidator.STANDARD_FORMAT)

    def __init__(self, strict: bool, formatType: int) -> None:
        super().__init__(strict, formatType, False)

    @staticmethod
    def getInstance() -> BigIntegerValidator:
        return BigIntegerValidator.__VALIDATOR
