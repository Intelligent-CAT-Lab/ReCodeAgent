from __future__ import annotations
import re
import enum
import numbers
import io
import typing
from typing import *
import os
from src.main.org.apache.commons.validator.routines.checkdigit.CheckDigit import *
from src.main.org.apache.commons.validator.routines.checkdigit.CheckDigitException import *
from src.main.org.apache.commons.validator.routines.checkdigit.ModulusCheckDigit import *


class ISINCheckDigit(ModulusCheckDigit):

    ISIN_CHECK_DIGIT: CheckDigit = None
    __POSITION_WEIGHT: typing.List[int] = [2, 1]
    __MAX_ALPHANUMERIC_VALUE: int = 35
    __serialVersionUID: int = -1239211208101323599

    @staticmethod
    def initialize_fields() -> None:
        ISINCheckDigit.ISIN_CHECK_DIGIT: CheckDigit = ISINCheckDigit()

    def _weightedValue(self, charValue: int, leftPos: int, rightPos: int) -> int:
        weight = self.__POSITION_WEIGHT[
            rightPos % 2
        ]  # Determine the weight based on the position
        weightedValue = charValue * weight  # Calculate the weighted value
        return ModulusCheckDigit.sumDigits(
            weightedValue
        )  # Sum the digits of the weighted value

    def _calculateModulus(self, code: str, includesCheckDigit: bool) -> int:
        transformed = []
        if includesCheckDigit:
            checkDigit = code[-1]  # fetch the last character
            if not checkDigit.isdigit():
                raise CheckDigitException.CheckDigitException1(
                    f"Invalid checkdigit[{checkDigit}] in {code}"
                )
        for i, char in enumerate(code):
            charValue = (
                int(char) if char.isdigit() else ord(char.upper()) - ord("A") + 10
            )
            if (
                charValue < 0
                or charValue > self._ISINCheckDigit__MAX_ALPHANUMERIC_VALUE
            ):
                raise CheckDigitException.CheckDigitException1(
                    f"Invalid Character[{i + 1}] = '{char}'"
                )
            transformed.append(str(charValue))
        transformed_code = "".join(transformed)
        return super()._calculateModulus(transformed_code, includesCheckDigit)

    def __init__(self) -> None:
        super().__init__(10)  # CHECKSTYLE IGNORE MagicNumber


ISINCheckDigit.initialize_fields()
