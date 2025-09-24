from __future__ import annotations
import re
import numbers
import io
import typing
from typing import *
import decimal
from src.main.org.apache.commons.validator.routines.AbstractNumberValidator import *
from src.main.org.apache.commons.validator.routines.BigDecimalValidator import *


class PercentValidator(BigDecimalValidator):

    __POINT_ZERO_ONE: decimal.Decimal = decimal.Decimal("0.01")
    __PERCENT_SYMBOL: str = "%"
    __VALIDATOR: PercentValidator = None
    __serialVersionUID: int = -3508241924961535772

    @staticmethod
    def initialize_fields() -> None:
        PercentValidator.__VALIDATOR: PercentValidator = (
            PercentValidator.PercentValidator1()
        )

    def _parse(self, value: str, formatter: Format) -> typing.Any:
        parsed_value = super()._parse(value, formatter)
        if parsed_value is not None or not isinstance(formatter, DecimalFormat):
            return parsed_value

        decimal_format = formatter
        pattern = decimal_format.toPattern()
        if self.__PERCENT_SYMBOL in pattern:
            buffer = []
            for char in pattern:
                if char != self.__PERCENT_SYMBOL:
                    buffer.append(char)
            decimal_format.applyPattern("".join(buffer))
            parsed_value = super()._parse(value, decimal_format)

            if parsed_value is not None:
                parsed_value = parsed_value * self.__POINT_ZERO_ONE

        return parsed_value

    @staticmethod
    def PercentValidator1() -> PercentValidator:
        return PercentValidator(True)

    def __init__(self, strict: bool) -> None:
        super().__init__(strict, self.PERCENT_FORMAT, True)

    @staticmethod
    def getInstance() -> BigDecimalValidator:
        return PercentValidator.__VALIDATOR


PercentValidator.initialize_fields()
