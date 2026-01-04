from __future__ import annotations
import re
import io
import typing
from typing import *
import os
from src.main.org.apache.commons.validator.routines.checkdigit.CheckDigit import *
from src.main.org.apache.commons.validator.routines.checkdigit.CheckDigitException import *
from src.main.org.apache.commons.validator.routines.checkdigit.ModulusCheckDigit import *


class SedolCheckDigit(ModulusCheckDigit):

    SEDOL_CHECK_DIGIT: CheckDigit = None  # LLM could not translate this field

    __POSITION_WEIGHT: typing.List[int] = [1, 3, 1, 7, 3, 9, 1]
    __MAX_ALPHANUMERIC_VALUE: int = 35
    __serialVersionUID: int = -8976881621148878443

    def _toInt(self, character: str, leftPos: int, rightPos: int) -> int:
        # Get numeric value of character (0-9 for digits, 10-35 for A-Z)
        if len(character) != 1:
            raise CheckDigitException.CheckDigitException1(
                f"Invalid Character[{leftPos},{rightPos}] = '{character}' - must be single character"
            )

        # Convert character to numeric value
        if character.isdigit():
            charValue = int(character)
        elif character.isalpha():
            charValue = ord(character.upper()) - ord("A") + 10
        else:
            charValue = -1

        charMax = 9 if rightPos == 1 else self.__MAX_ALPHANUMERIC_VALUE

        if charValue < 0 or charValue > charMax:
            raise CheckDigitException.CheckDigitException1(
                f"Invalid Character[{leftPos},{rightPos}] = '{charValue}' out of range 0 to {charMax}"
            )

        return charValue

    def _weightedValue(self, charValue: int, leftPos: int, rightPos: int) -> int:
        return charValue * self.__POSITION_WEIGHT[leftPos - 1]

    def _calculateModulus(self, code: str, includesCheckDigit: bool) -> int:
        if len(code) > len(self.__POSITION_WEIGHT):
            raise CheckDigitException.CheckDigitException1(f"Invalid Code Length = {len(code)}")
        return super()._calculateModulus(code, includesCheckDigit)

    def __init__(self) -> None:
        ModulusCheckDigit.__init__(self, 10)
