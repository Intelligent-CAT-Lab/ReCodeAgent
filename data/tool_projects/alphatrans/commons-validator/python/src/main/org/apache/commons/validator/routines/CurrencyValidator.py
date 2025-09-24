from __future__ import annotations
import re
import decimal
import numbers
import io
import typing
from typing import *
from src.main.org.apache.commons.validator.routines.AbstractNumberValidator import *
from src.main.org.apache.commons.validator.routines.BigDecimalValidator import *


class CurrencyValidator(BigDecimalValidator):

    __CURRENCY_SYMBOL: str = "\u00a4"
    __VALIDATOR: CurrencyValidator = None
    __serialVersionUID: int = -4201640771171486514

    @staticmethod
    def initialize_fields() -> None:
        CurrencyValidator.__VALIDATOR: CurrencyValidator = (
            CurrencyValidator.CurrencyValidator1()
        )

    def _parse(self, value: str, formatter: Format) -> typing.Any:
        parsed_value = super()._parse(value, formatter)
        if parsed_value is not None or not isinstance(formatter, DecimalFormat):
            return parsed_value

        decimal_format: DecimalFormat = formatter
        pattern: str = decimal_format.toPattern()
        if self.__CURRENCY_SYMBOL in pattern:
            buffer = []
            for char in pattern:
                if char != self.__CURRENCY_SYMBOL:
                    buffer.append(char)
            decimal_format.applyPattern("".join(buffer))
            parsed_value = super()._parse(value, decimal_format)

        return parsed_value

    @staticmethod
    def CurrencyValidator1() -> CurrencyValidator:
        return CurrencyValidator(True, True)

    def __init__(self, strict: bool, allowFractions: bool) -> None:
        super().__init__(strict, self.CURRENCY_FORMAT, allowFractions)

    @staticmethod
    def getInstance() -> BigDecimalValidator:
        return CurrencyValidator.__VALIDATOR


CurrencyValidator.initialize_fields()
