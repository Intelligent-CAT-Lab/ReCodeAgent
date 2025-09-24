from __future__ import annotations
import copy
import re
import numbers
import io
import typing
from typing import *
import os
from src.main.org.apache.commons.validator.routines.checkdigit.CheckDigitException import *
from src.main.org.apache.commons.validator.routines.checkdigit.ModulusCheckDigit import *


class ModulusTenCheckDigit(ModulusCheckDigit):

    __sumWeightedDigits: bool = False

    __useRightPos: bool = False

    __postitionWeight: typing.List[int] = None

    __serialVersionUID: int = -3752929983453368497

    def toString(self) -> str:
        return (
            f"{self.__class__.__name__}"
            f"[postitionWeight={self.__postitionWeight}, "
            f"useRightPos={self.__useRightPos}, "
            f"sumWeightedDigits={self.__sumWeightedDigits}]"
        )

    def _weightedValue(self, charValue: int, leftPos: int, rightPos: int) -> int:
        pos = rightPos if self.__useRightPos else leftPos
        weight = self.__postitionWeight[(pos - 1) % len(self.__postitionWeight)]
        weightedValue = charValue * weight
        if self.__sumWeightedDigits:
            weightedValue = ModulusCheckDigit.sumDigits(weightedValue)
        return weightedValue

    def _toInt(self, character: str, leftPos: int, rightPos: int) -> int:
        num = int(character) if character.isdigit() else -1
        if num < 0:
            raise CheckDigitException.CheckDigitException1(
                f"Invalid Character[{leftPos}] = '{character}'"
            )
        return num

    def isValid(self, code: str) -> bool:
        if code is None or len(code) == 0:
            return False
        if not code[-1].isdigit():
            return False

        return super().isValid(code)

    @staticmethod
    def ModulusTenCheckDigit2(
        postitionWeight: typing.List[int],
    ) -> ModulusTenCheckDigit:
        return ModulusTenCheckDigit(postitionWeight, False, False)

    @staticmethod
    def ModulusTenCheckDigit1(
        postitionWeight: typing.List[int], useRightPos: bool
    ) -> ModulusTenCheckDigit:
        return ModulusTenCheckDigit(postitionWeight, useRightPos, False)

    def __init__(
        self,
        postitionWeight: typing.List[int],
        useRightPos: bool,
        sumWeightedDigits: bool,
    ) -> None:
        super().__init__(10)  # CHECKSTYLE IGNORE MagicNumber
        self.__postitionWeight = postitionWeight.copy()
        self.__useRightPos = useRightPos
        self.__sumWeightedDigits = sumWeightedDigits
