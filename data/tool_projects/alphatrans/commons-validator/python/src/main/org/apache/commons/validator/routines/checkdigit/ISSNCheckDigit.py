from __future__ import annotations
import re
import numbers
import io
import os
from src.main.org.apache.commons.validator.routines.checkdigit.CheckDigit import *
from src.main.org.apache.commons.validator.routines.checkdigit.CheckDigitException import *
from src.main.org.apache.commons.validator.routines.checkdigit.ModulusCheckDigit import *


class ISSNCheckDigit(ModulusCheckDigit):

    ISSN_CHECK_DIGIT: CheckDigit = None
    __serialVersionUID: int = 1

    @staticmethod
    def initialize_fields() -> None:
        ISSNCheckDigit.ISSN_CHECK_DIGIT: CheckDigit = ISSNCheckDigit()

    def _toInt(self, character: str, leftPos: int, rightPos: int) -> int:
        if rightPos == 1 and character == "X":
            return 10  # CHECKSTYLE IGNORE MagicNumber
        return super()._toInt(character, leftPos, rightPos)

    def _toCheckDigit(self, charValue: int) -> str:
        if charValue == 10:  # CHECKSTYLE IGNORE MagicNumber
            return "X"
        return super()._toCheckDigit(charValue)

    def _weightedValue(self, charValue: int, leftPos: int, rightPos: int) -> int:
        return charValue * (9 - leftPos)  # CHECKSTYLE IGNORE MagicNumber

    def __init__(self) -> None:
        super().__init__(11)  # CHECKSTYLE IGNORE MagicNumber


ISSNCheckDigit.initialize_fields()
