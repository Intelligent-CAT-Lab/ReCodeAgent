from __future__ import annotations
import locale
import re
from abc import ABC
import io
import numbers
import typing
from typing import *
from src.main.org.apache.commons.validator.routines.AbstractFormatValidator import *


class AbstractNumberValidator(AbstractFormatValidator, ABC):

    PERCENT_FORMAT: int = 2
    CURRENCY_FORMAT: int = 1
    STANDARD_FORMAT: int = 0
    __formatType: int = 0

    __allowFractions: bool = False

    __serialVersionUID: int = -3088817875906765463

    def _getFormat(self, pattern: str, locale: typing.Any) -> Format:
        return self._getFormat0(pattern, locale)

    def isValid3(self, value: str, pattern: str, locale: typing.Any) -> bool:
        parsed_value = self._parse(value, pattern, locale)
        return False if parsed_value is None else True

    def _getFormat1(self, locale: typing.Any) -> Format:
        formatter = None
        if self.__formatType == self.CURRENCY_FORMAT:
            if locale is None:
                formatter = NumberFormat.getCurrencyInstance()
            else:
                formatter = NumberFormat.getCurrencyInstance(locale)
        elif self.__formatType == self.PERCENT_FORMAT:
            if locale is None:
                formatter = NumberFormat.getPercentInstance()
            else:
                formatter = NumberFormat.getPercentInstance(locale)
        else:
            if locale is None:
                formatter = NumberFormat.getInstance()
            else:
                formatter = NumberFormat.getInstance(locale)
            if not self.isAllowFractions():
                formatter.setParseIntegerOnly(True)
        return formatter

    def _determineScale(self, format_: typing.Any) -> int:
        if not self.isStrict():
            return -1
        if not self.isAllowFractions() or format_.isParseIntegerOnly():
            return 0
        minimum_fraction = format_.getMinimumFractionDigits()
        maximum_fraction = format_.getMaximumFractionDigits()
        if minimum_fraction != maximum_fraction:
            return -1
        scale = minimum_fraction
        if isinstance(format_, DecimalFormat):
            multiplier = format_.getMultiplier()
            if multiplier == 100:  # CHECKSTYLE IGNORE MagicNumber
                scale += 2  # CHECKSTYLE IGNORE MagicNumber
            elif multiplier == 1000:  # CHECKSTYLE IGNORE MagicNumber
                scale += 3  # CHECKSTYLE IGNORE MagicNumber
        elif self._AbstractNumberValidator__formatType == self.PERCENT_FORMAT:
            scale += 2  # CHECKSTYLE IGNORE MagicNumber
        return scale

    def _getFormat0(self, pattern: str, locale: typing.Any) -> Format:
        formatter = None
        use_pattern = pattern is not None and len(pattern) > 0

        if not use_pattern:
            formatter = self._getFormat1(locale)
        elif locale is None:
            formatter = DecimalFormat(pattern)
        else:
            symbols = DecimalFormatSymbols(locale)
            formatter = DecimalFormat(pattern, symbols)

        if not self.isAllowFractions():
            formatter.setParseIntegerOnly(True)

        return formatter

    def _parse(self, value: str, pattern: str, locale: typing.Any) -> typing.Any:
        value = None if value is None else value.strip()
        if value is None or len(value) == 0:
            return None
        formatter = self._getFormat0(pattern, locale)
        return self._parse(value, formatter)

    def maxValue(
        self,
        value: typing.Union[int, float, numbers.Number],
        max_: typing.Union[int, float, numbers.Number],
    ) -> bool:
        if self.isAllowFractions():
            return value <= max_
        return int(value) <= int(max_)

    def minValue(
        self,
        value: typing.Union[int, float, numbers.Number],
        min_: typing.Union[int, float, numbers.Number],
    ) -> bool:
        if self.isAllowFractions():
            return value >= min_
        return int(value) >= int(min_)

    def isInRange(
        self,
        value: typing.Union[int, float, numbers.Number],
        min_: typing.Union[int, float, numbers.Number],
        max_: typing.Union[int, float, numbers.Number],
    ) -> bool:
        return self.minValue(value, min_) and self.maxValue(value, max_)

    def getFormatType(self) -> int:
        return self.__formatType

    def isAllowFractions(self) -> bool:
        return self.__allowFractions

    def __init__(self, strict: bool, formatType: int, allowFractions: bool) -> None:

        pass  # LLM could not translate this method

    def _processParsedValue(self, value: typing.Any, formatter: Format) -> typing.Any:
        raise NotImplementedError("Subclasses must implement this method")
