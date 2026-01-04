from __future__ import annotations
import re
import io
import typing
from typing import *
import os
from src.main.org.apache.commons.validator.routines.checkdigit.CheckDigit import *
from src.main.org.apache.commons.validator.routines.checkdigit.CheckDigitException import *
from src.main.org.apache.commons.validator.routines.checkdigit.ModulusCheckDigit import *


class ISINCheckDigit(ModulusCheckDigit):

    ISIN_CHECK_DIGIT: CheckDigit = None  # Will be initialized after class definition
    __POSITION_WEIGHT: typing.List[int] = [2, 1]
    __MAX_ALPHANUMERIC_VALUE: int = 35
    __serialVersionUID: int = -1239211208101323599

    def _weightedValue(self, charValue: int, leftPos: int, rightPos: int) -> int:
        weight = self.__POSITION_WEIGHT[rightPos % 2]
        weightedValue = charValue * weight
        return ModulusCheckDigit.sumDigits(weightedValue)

    def _calculateModulus(self, code: str, includesCheckDigit: bool) -> int:
        transformed = []
        if includesCheckDigit:
            checkDigit = code[-1]
            if not checkDigit.isdigit():
                raise CheckDigitException.CheckDigitException1(f"Invalid checkdigit[{checkDigit}] in {code}")

        for i in range(len(code)):
            char = code[i]
            if char.isdigit():
                charValue = int(char)
            elif char.isalpha():
                charValue = ord(char.upper()) - ord("A") + 10
            else:
                charValue = -1

            if charValue < 0 or charValue > self._ISINCheckDigit__MAX_ALPHANUMERIC_VALUE:
                raise CheckDigitException.CheckDigitException1(f"Invalid Character[{i + 1}] = '{charValue}'")
            transformed.append(str(charValue))

        return super()._calculateModulus("".join(transformed), includesCheckDigit)

    def __init__(self) -> None:
        super().__init__(10)
