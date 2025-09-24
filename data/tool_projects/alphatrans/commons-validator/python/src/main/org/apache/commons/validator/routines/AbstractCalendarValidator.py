from __future__ import annotations
import time
import locale
import re
import os
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
        parsed_value = self._parse(value, pattern, locale, None)
        return parsed_value is not None

    def _compareQuarters(
        self,
        value: typing.Union[datetime.datetime, datetime.date],
        compare: typing.Union[datetime.datetime, datetime.date],
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
        result = self.__calculateCompareResult(value, compare, "hour")
        if result != 0 or field in ["hour", "hour_of_day"]:
            return result

        result = self.__calculateCompareResult(value, compare, "minute")
        if result != 0 or field == "minute":
            return result

        result = self.__calculateCompareResult(value, compare, "second")
        if result != 0 or field == "second":
            return result

        if field == "millisecond":
            return self.__calculateCompareResult(value, compare, "microsecond") // 1000

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
        result = self.__calculateCompareResult(value, compare, "year")
        if result != 0 or field == "year":
            return result

        if field == "week_of_year":
            return self.__calculateCompareResult(value, compare, "isocalendar_week")

        if field == "day_of_year":
            return self.__calculateCompareResult(
                value, compare, "timetuple_day_of_year"
            )

        result = self.__calculateCompareResult(value, compare, "month")
        if result != 0 or field == "month":
            return result

        if field == "week_of_month":
            # Custom logic for week_of_month may be required, as Python's datetime does not directly support it
            raise NotImplementedError(
                "Comparison for 'week_of_month' is not implemented"
            )

        result = self.__calculateCompareResult(value, compare, "day")
        if result != 0 or field in ["day", "day_of_week", "day_of_week_in_month"]:
            return result

        return self._compareTime(value, compare, field)

    def _getFormat1(self, locale: typing.Any) -> Format:
        formatter = None
        if self.__dateStyle >= 0 and self.__timeStyle >= 0:
            if locale is None:
                formatter = (
                    datetime.datetime.strftime
                )  # Simulating DateFormat.getDateTimeInstance
            else:
                formatter = (
                    datetime.datetime.strftime
                )  # Simulating DateFormat.getDateTimeInstance with locale
        elif self.__timeStyle >= 0:
            if locale is None:
                formatter = (
                    datetime.time.strftime
                )  # Simulating DateFormat.getTimeInstance
            else:
                formatter = (
                    datetime.time.strftime
                )  # Simulating DateFormat.getTimeInstance with locale
        else:
            use_date_style = (
                self.__dateStyle if self.__dateStyle >= 0 else datetime.date.strftime
            )  # Simulating 3
            if locale is None:
                formatter = (
                    datetime.date.strftime
                )  # Simulating DateFormat.getDateInstance
            else:
                formatter = (
                    datetime.date.strftime
                )  # Simulating DateFormat.getDateInstance with locale

        # Simulating formatter.setLenient(false) - Python's datetime is strict by default
        return formatter

    def _getFormat0(self, pattern: str, locale: typing.Any) -> Format:
        formatter = None
        use_pattern = pattern is not None and len(pattern) > 0
        if not use_pattern:
            formatter = self._getFormat1(locale)
        elif locale is None:
            formatter = (
                datetime.datetime.strptime
            )  # Simulating SimpleDateFormat(pattern)
        else:
            # Simulating SimpleDateFormat(pattern, symbols) with locale
            formatter = (
                datetime.datetime.strptime
            )  # Python's datetime does not directly support locale-based symbols
            # Note: You may need to use a library like Babel for full locale support

        # Simulating formatter.setLenient(false) - Python's datetime is strict by default
        return formatter

    def _parse(
        self,
        value: str,
        pattern: str,
        locale: typing.Any,
        timeZone: typing.Union[zoneinfo.ZoneInfo, datetime.timezone],
    ) -> typing.Any:
        value = value.strip() if value is not None else None
        if value is None or len(value) == 0:
            return None

        formatter = self._getFormat0(pattern, locale)
        if timeZone is not None:
            # Simulating formatter.setTimeZone(timeZone)
            # Python's datetime does not have a direct equivalent for setting a timezone on a formatter.
            # Instead, we handle time zones during parsing or conversion.
            pass  # Time zone handling would be implemented here if needed.

        return self._parse(value, formatter)

    def _format5(self, value: typing.Any, formatter: Format) -> str:
        if value is None:
            return None
        elif isinstance(value, datetime.datetime):
            value = value.date()
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
            formatter.timezone = timeZone  # Simulating formatter.setTimeZone(timeZone)
        elif isinstance(value, datetime.datetime):
            formatter.timezone = (
                value.tzinfo
            )  # Simulating ((Calendar) value).getTimeZone()
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
        difference = getattr(value, field) - getattr(compare, field)
        if difference < 0:
            return -1
        elif difference > 0:
            return 1
        else:
            return 0

    def __calculateQuarter(
        self,
        calendar: typing.Union[datetime.datetime, datetime.date],
        monthOfFirstQuarter: int,
    ) -> int:
        year = calendar.year
        month = calendar.month
        relative_month = (
            (month - monthOfFirstQuarter)
            if month >= monthOfFirstQuarter
            else (month + (12 - monthOfFirstQuarter))
        )
        quarter = (relative_month // 3) + 1
        if month < monthOfFirstQuarter:
            year -= 1
        return (year * 10) + quarter

    def _processParsedValue(self, value: typing.Any, formatter: Format) -> typing.Any:
        raise NotImplementedError("Subclasses must implement this method")
