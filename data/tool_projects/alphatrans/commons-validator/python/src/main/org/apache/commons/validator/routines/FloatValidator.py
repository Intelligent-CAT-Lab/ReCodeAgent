from __future__ import annotations
import locale
import re
import os
import io
import numbers
import typing
from typing import *
from src.main.org.apache.commons.validator.routines.AbstractNumberValidator import *


class FloatValidator(AbstractNumberValidator):

    __VALIDATOR: FloatValidator = None
    __serialVersionUID: int = -4513245432806414267

    @staticmethod
    def initialize_fields() -> None:
        FloatValidator.__VALIDATOR: FloatValidator = FloatValidator.FloatValidator1()

    def _processParsedValue(self, value: typing.Any, formatter: Format) -> typing.Any:
        double_value = float(
            value
        )  # Cast value to float (equivalent to Java's doubleValue())

        if double_value > 0:
            if double_value < float.fromhex(
                "0x1.0p-149"
            ):  # Equivalent to Float.MIN_VALUE
                return None
            if double_value > float.fromhex(
                "0x1.fffffep127"
            ):  # Equivalent to 3.4028235E38
                return None
        elif double_value < 0:
            pos_double = double_value * -1
            if pos_double < float.fromhex(
                "0x1.0p-149"
            ):  # Equivalent to Float.MIN_VALUE
                return None
            if pos_double > float.fromhex(
                "0x1.fffffep127"
            ):  # Equivalent to 3.4028235E38
                return None

        return float(double_value)  # Return the value as a float

    def maxValue1(self, value: float, max_: float) -> bool:
        return self.maxValue0(float(value), max_)

    def maxValue0(self, value: float, max_: float) -> bool:
        return value <= max_

    def minValue1(self, value: float, min_: float) -> bool:
        return self.minValue0(value, min_)

    def minValue0(self, value: float, min_: float) -> bool:
        return value >= min_

    def isInRange1(self, value: float, min_: float, max_: float) -> bool:
        return self.isInRange0(float(value), min_, max_)

    def isInRange0(self, value: float, min_: float, max_: float) -> bool:
        return min_ <= value <= max_

    def validate3(self, value: str, pattern: str, locale: typing.Any) -> float:
        return float(self._parse(value, pattern, locale))

    def validate2(self, value: str, locale: typing.Any) -> float:
        return self._parse(value, None, locale)

    def validate1(self, value: str, pattern: str) -> float:
        return float(self._parse(value, pattern, None))

    def validate0(self, value: str) -> float:
        return self._parse(value, None, None)

    @staticmethod
    def FloatValidator1() -> FloatValidator:
        return FloatValidator(True, AbstractNumberValidator.STANDARD_FORMAT)

    def __init__(self, strict: bool, formatType: int) -> None:
        super().__init__(strict, formatType, True)

    @staticmethod
    def getInstance() -> FloatValidator:
        return FloatValidator.__VALIDATOR


FloatValidator.initialize_fields()
