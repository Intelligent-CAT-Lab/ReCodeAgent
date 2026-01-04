from __future__ import annotations
import locale
import re
import io
import numbers
import typing
from typing import *
import decimal
from src.main.org.apache.commons.validator.routines.AbstractNumberValidator import *


class BigDecimalValidator(AbstractNumberValidator):

    __VALIDATOR: BigDecimalValidator = None  # LLM could not translate this field

    __serialVersionUID: int = -670320911490506772

    def _processParsedValue(self, value: typing.Any, formatter: Format) -> typing.Any:
        decimal_value = None
        if isinstance(value, int):
            decimal_value = decimal.Decimal(value)
        else:
            decimal_value = decimal.Decimal(str(value))

        scale = self._determineScale(formatter)
        if scale >= 0:
            decimal_value = decimal_value.quantize(decimal.Decimal(10) ** -scale, rounding=decimal.ROUND_DOWN)

        return decimal_value

    def maxValue(self, value: decimal.Decimal, max_: float) -> bool:
        return float(value) <= max_

    def minValue(self, value: decimal.Decimal, min_: float) -> bool:
        return float(value) >= min_

    def isInRange(self, value: decimal.Decimal, min_: float, max_: float) -> bool:
        return float(value) >= min_ and float(value) <= max_

    def validate3(self, value: str, pattern: str, locale: typing.Any) -> decimal.Decimal:

        pass  # LLM could not translate this method

    def validate2(self, value: str, locale: typing.Any) -> decimal.Decimal:

        pass  # LLM could not translate this method

    def validate1(self, value: str, pattern: str) -> decimal.Decimal:

        pass  # LLM could not translate this method

    def validate0(self, value: str) -> decimal.Decimal:

        pass  # LLM could not translate this method

    @staticmethod
    def BigDecimalValidator2() -> BigDecimalValidator:
        return BigDecimalValidator.BigDecimalValidator1(True)

    @staticmethod
    def BigDecimalValidator1(strict: bool) -> BigDecimalValidator:
        return BigDecimalValidator(strict, AbstractNumberValidator.STANDARD_FORMAT, True)

    def __init__(self, strict: bool, formatType: int, allowFractions: bool) -> None:
        super().__init__(strict, formatType, allowFractions)

    @staticmethod
    def getInstance() -> BigDecimalValidator:
        return BigDecimalValidator.__VALIDATOR
