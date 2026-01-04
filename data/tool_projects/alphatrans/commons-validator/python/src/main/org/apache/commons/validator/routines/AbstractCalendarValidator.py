from __future__ import annotations
import time
import locale
import re
from abc import ABC
import io
import typing
from typing import *
import datetime
import zoneinfo
from src.main.org.apache.commons.validator.routines.AbstractFormatValidator import *


class AbstractCalendarValidator(AbstractFormatValidator, ABC):

    __timeStyle: int = 0

    __dateStyle: int = 0

    __serialVersionUID: int = -1410008585975827379

    def _getFormat(self, pattern: str, locale: typing.Any) -> Format:
        return self._getFormat0(pattern, locale)

    def isValid3(self, value: str, pattern: str, locale: typing.Any) -> bool:
        parsedValue = self._parse(value, pattern, locale, None)
        return parsedValue is not None

    def _compareQuarters(
        self,
        value: typing.Union[
            datetime.datetime,
            datetime.date,
            datetime.time,
            datetime.timedelta,
            datetime.timezone,
        ],
        compare: typing.Union[
            datetime.datetime,
            datetime.date,
            datetime.time,
            datetime.timedelta,
            datetime.timezone,
        ],
        monthOfFirstQuarter: int,
    ) -> int:
        valueQuarter = self.__calculateQuarter(value, monthOfFirstQuarter)
        compareQuarter = self.__calculateQuarter(compare, monthOfFirstQuarter)
        if valueQuarter < compareQuarter:
            return -1
        elif valueQuarter > compareQuarter:
            return 1
        else:
            return 0

    def _compareTime(
        self,
        value: typing.Union[
            datetime.datetime,
            datetime.date,
            datetime.time,
            datetime.timedelta,
            datetime.timezone,
        ],
        compare: typing.Union[
            datetime.datetime,
            datetime.date,
            datetime.time,
            datetime.timedelta,
            datetime.timezone,
        ],
        field: int,
    ) -> int:
        result = 0

        # Compare HOUR_OF_DAY
        result = self.__calculateCompareResult(value, compare, self.HOUR_OF_DAY)
        if result != 0 or (field == self.HOUR or field == self.HOUR_OF_DAY):
            return result

        # Compare MINUTE
        result = self.__calculateCompareResult(value, compare, self.MINUTE)
        if result != 0 or field == self.MINUTE:
            return result

        # Compare SECOND
        result = self.__calculateCompareResult(value, compare, self.SECOND)
        if result != 0 or field == self.SECOND:
            return result

        # Compare MILLISECOND
        if field == self.MILLISECOND:
            return self.__calculateCompareResult(value, compare, self.MILLISECOND)

        raise ValueError(f"Invalid field: {field}")

    def _compare(
        self,
        value: typing.Union[
            datetime.datetime,
            datetime.date,
            datetime.time,
            datetime.timedelta,
            datetime.timezone,
        ],
        compare: typing.Union[
            datetime.datetime,
            datetime.date,
            datetime.time,
            datetime.timedelta,
            datetime.timezone,
        ],
        field: int,
    ) -> int:
        result = 0

        result = self.__calculateCompareResult(value, compare, self.YEAR)
        if result != 0 or field == self.YEAR:
            return result

        if field == self.WEEK_OF_YEAR:
            return self.__calculateCompareResult(value, compare, self.WEEK_OF_YEAR)

        if field == self.DAY_OF_YEAR:
            return self.__calculateCompareResult(value, compare, self.DAY_OF_YEAR)

        result = self.__calculateCompareResult(value, compare, self.MONTH)
        if result != 0 or field == self.MONTH:
            return result

        if field == self.WEEK_OF_MONTH:
            return self.__calculateCompareResult(value, compare, self.WEEK_OF_MONTH)

        result = self.__calculateCompareResult(value, compare, self.DATE)
        if result != 0 or (field == self.DATE or field == self.DAY_OF_WEEK or field == self.DAY_OF_WEEK_IN_MONTH):
            return result

        return self._compareTime(value, compare, field)

    def _getFormat1(self, locale: typing.Any) -> Format:
        formatter = None

        if self.__dateStyle >= 0 and self.__timeStyle >= 0:
            if locale is None:
                formatter = DateFormat.getDateTimeInstance(self.__dateStyle, self.__timeStyle)
            else:
                formatter = DateFormat.getDateTimeInstance(self.__dateStyle, self.__timeStyle, locale)
        elif self.__timeStyle >= 0:
            if locale is None:
                formatter = DateFormat.getTimeInstance(self.__timeStyle)
            else:
                formatter = DateFormat.getTimeInstance(self.__timeStyle, locale)
        else:
            useDateStyle = self.__dateStyle if self.__dateStyle >= 0 else 3
            if locale is None:
                formatter = DateFormat.getDateInstance(useDateStyle)
            else:
                formatter = DateFormat.getDateInstance(useDateStyle, locale)

        formatter.setLenient(False)
        return formatter

    def _getFormat0(self, pattern: str, locale: typing.Any) -> Format:
        formatter = None
        usePattern = pattern is not None and len(pattern) > 0
        if not usePattern:
            formatter = self._getFormat1(locale)
        elif locale is None:
            formatter = SimpleDateFormat(pattern)
        else:
            symbols = DateFormatSymbols(locale)
            formatter = SimpleDateFormat(pattern, symbols)
        formatter.setLenient(False)
        return formatter

    def _parse(
        self,
        value: str,
        pattern: str,
        locale: typing.Any,
        timeZone: typing.Union[zoneinfo.ZoneInfo, datetime.timezone],
    ) -> typing.Any:
        value = None if value is None else value.strip()
        if value is None or len(value) == 0:
            return None
        formatter = self._getFormat0(pattern, locale)
        if timeZone is not None:
            formatter.setTimeZone(timeZone)
        return super()._parse(value, formatter)

    def _format5(self, value: typing.Any, formatter: Format) -> str:
        if value is None:
            return None
        elif isinstance(value, datetime.datetime):
            value = value
        return formatter.format(value)

    def format4(
        self,
        value: typing.Any,
        pattern: str,
        locale: typing.Any,
        timeZone: typing.Union[zoneinfo.ZoneInfo, datetime.timezone],
    ) -> str:
        formatter = self._getFormat0(pattern, locale)
        if timeZone is not None:
            formatter.setTimeZone(timeZone)
        elif isinstance(value, datetime.datetime):
            if value.tzinfo is not None:
                formatter.setTimeZone(value.tzinfo)
        return self._format5(value, formatter)

    def format3(self, value: typing.Any, pattern: str, locale: typing.Any) -> str:
        return self.format4(value, pattern, locale, None)

    def format2(
        self,
        value: typing.Any,
        locale: typing.Any,
        timeZone: typing.Union[zoneinfo.ZoneInfo, datetime.timezone],
    ) -> str:
        return self.format4(value, None, locale, timeZone)

    def format1(
        self,
        value: typing.Any,
        pattern: str,
        timeZone: typing.Union[zoneinfo.ZoneInfo, datetime.timezone],
    ) -> str:
        return self.format4(value, pattern, None, timeZone)

    def format0(
        self,
        value: typing.Any,
        timeZone: typing.Union[zoneinfo.ZoneInfo, datetime.timezone],
    ) -> str:
        return self.format4(value, None, None, timeZone)

    def __init__(self, strict: bool, dateStyle: int, timeStyle: int) -> None:
        super().__init__(strict)
        self.__dateStyle = dateStyle
        self.__timeStyle = timeStyle

    def __calculateCompareResult(
        self,
        value: typing.Union[
            datetime.datetime,
            datetime.date,
            datetime.time,
            datetime.timedelta,
            datetime.timezone,
        ],
        compare: typing.Union[
            datetime.datetime,
            datetime.date,
            datetime.time,
            datetime.timedelta,
            datetime.timezone,
        ],
        field: int,
    ) -> int:
        # Map Java Calendar field constants to Python datetime attributes
        # This assumes field is a Calendar field constant (e.g., Calendar.YEAR, Calendar.MONTH, etc.)
        # and that value and compare are datetime objects with the appropriate attributes

        value_field = getattr(value, self._getFieldName(field), 0)
        compare_field = getattr(compare, self._getFieldName(field), 0)

        difference = value_field - compare_field
        if difference < 0:
            return -1
        elif difference > 0:
            return 1
        else:
            return 0

    def __calculateQuarter(
        self,
        calendar: typing.Union[
            datetime.datetime,
            datetime.date,
            datetime.time,
            datetime.timedelta,
            datetime.timezone,
        ],
        monthOfFirstQuarter: int,
    ) -> int:
        # Get year and month from the calendar object
        if isinstance(calendar, datetime.datetime):
            year = calendar.year
            month = calendar.month
        elif isinstance(calendar, datetime.date):
            year = calendar.year
            month = calendar.month
        else:
            raise ValueError("Calendar must be a datetime or date object")

        # Calculate relative month based on the first quarter month
        if month >= monthOfFirstQuarter:
            relativeMonth = month - monthOfFirstQuarter
        else:
            relativeMonth = month + (12 - monthOfFirstQuarter)

        # Calculate quarter (1-4)
        quarter = (relativeMonth // 3) + 1

        # Adjust year if month is before the first quarter month
        if month < monthOfFirstQuarter:
            year -= 1

        return (year * 10) + quarter

    def _processParsedValue(self, value: typing.Any, formatter: Format) -> typing.Any:
        pass
