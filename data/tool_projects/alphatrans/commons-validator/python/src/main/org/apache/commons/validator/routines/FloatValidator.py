from __future__ import annotations
import locale
import re
import sys
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
        double_value = float(value)

        if double_value > 0:
            if double_value < sys.float_info.min:
                return None
            if double_value > sys.float_info.max:
                return None
        elif double_value < 0:
            pos_double = double_value * -1
            if pos_double < sys.float_info.min:
                return None
            if pos_double > sys.float_info.max:
                return None

        return float(double_value)

    def maxValue1(self, value: float, max_: float) -> bool:
        return self.maxValue0(value, max_)

    def maxValue0(self, value: float, max_: float) -> bool:
        return value <= max_

    def minValue1(self, value: float, min_: float) -> bool:
        return self.minValue0(value, min_)

    def minValue0(self, value: float, min_: float) -> bool:
        return value >= min_

    def isInRange1(self, value: float, min_: float, max_: float) -> bool:
        return self.isInRange0(value, min_, max_)

    def isInRange0(self, value: float, min_: float, max_: float) -> bool:
        return min_ <= value <= max_

    def validate3(self, value: str, pattern: str, locale: typing.Any) -> float:

        pass  # LLM could not translate this method

    def validate2(self, value: str, locale: typing.Any) -> float:

        pass  # LLM could not translate this method

    def validate1(self, value: str, pattern: str) -> float:

        pass  # LLM could not translate this method

    def validate0(self, value: str) -> float:

        pass  # LLM could not translate this method

    @staticmethod
    def FloatValidator1() -> FloatValidator:
        return FloatValidator(True, AbstractNumberValidator.STANDARD_FORMAT)

    def __init__(self, strict: bool, formatType: int) -> None:
        super().__init__(strict, formatType, True)

    @staticmethod
    def getInstance() -> FloatValidator:
        return FloatValidator.__VALIDATOR


FloatValidator.initialize_fields()
