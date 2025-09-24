from __future__ import annotations
import re
import numbers
from abc import ABC
import io
import os
from src.main.org.apache.commons.validator.routines.checkdigit.CheckDigit import *
from src.main.org.apache.commons.validator.routines.checkdigit.CheckDigitException import *


class ModulusCheckDigit(CheckDigit, ABC):

    __modulus: int = 0

    __serialVersionUID: int = 2948962251251528941

    def calculate(self, code: str) -> str:
        if code is None or len(code) == 0:
            raise CheckDigitException.CheckDigitException1("Code is missing")

        modulus_result = self._calculateModulus(code, False)
        char_value = (
            self._ModulusCheckDigit__modulus - modulus_result
        ) % self._ModulusCheckDigit__modulus
        return self._toCheckDigit(char_value)

    def isValid(self, code: str) -> bool:
        if code is None or len(code) == 0:
            return False
        try:
            modulus_result = self._calculateModulus(code, True)
            return modulus_result == 0
        except CheckDigitException:
            return False

    @staticmethod
    def sumDigits(number: int) -> int:
        total = 0
        todo = number
        while todo > 0:
            total += todo % 10  # Add the last digit to total
            todo = todo // 10  # Remove the last digit
        return total

    def _toCheckDigit(self, charValue: int) -> str:
        if 0 <= charValue <= 9:  # CHECKSTYLE IGNORE MagicNumber
            return str(charValue)
        raise CheckDigitException.CheckDigitException1(
            f"Invalid Check Digit Value = {charValue}"
        )

    def _toInt(self, character: str, leftPos: int, rightPos: int) -> int:
        if character.isdigit():
            return int(character)
        raise CheckDigitException.CheckDigitException1(
            f"Invalid Character[{leftPos}] = '{character}'"
        )

    def _calculateModulus(self, code: str, includesCheckDigit: bool) -> int:
        total = 0
        for i in range(len(code)):
            lth = len(code) + (0 if includesCheckDigit else 1)
            leftPos = i + 1
            rightPos = lth - i
            charValue = self._toInt(code[i], leftPos, rightPos)
            total += self._weightedValue(charValue, leftPos, rightPos)

        if total == 0:
            raise CheckDigitException.CheckDigitException1("Invalid code, sum is zero")

        return total % self._ModulusCheckDigit__modulus

    def getModulus(self) -> int:
        return self.__modulus

    def __init__(self, modulus: int) -> None:
        self.__modulus = modulus

    def _weightedValue(self, charValue: int, leftPos: int, rightPos: int) -> int:
        raise NotImplementedError("Subclasses must implement this method")
