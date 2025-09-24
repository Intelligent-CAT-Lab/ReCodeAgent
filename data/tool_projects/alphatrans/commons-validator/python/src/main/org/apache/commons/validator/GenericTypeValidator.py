from __future__ import annotations
import time
import locale
import re
import os
import decimal
import io
import typing
from typing import *
import datetime
import logging

# from src.main.org.apache.commons.logging.Log import *
# from src.main.org.apache.commons.logging.LogFactory import *
from src.main.org.apache.commons.validator.GenericValidator import *


class GenericTypeValidator:

    __LOG: logging.Logger = logging.getLogger(__name__)
    __serialVersionUID: int = 5487162314134261703

    @staticmethod
    def formatCreditCard(value: str) -> int:
        return int(value) if GenericValidator.isCreditCard(value) else None

    @staticmethod
    def formatDate1(
        value: str, datePattern: str, strict: bool
    ) -> typing.Optional[datetime.datetime]:
        date = None

        if value is None or datePattern is None or len(datePattern) == 0:
            return None

        try:
            formatter = datetime.datetime.strptime
            date = formatter(value, datePattern)

            if strict and len(datePattern) != len(value):
                date = None
        except ValueError as e:
            if GenericTypeValidator.__LOG.isEnabledFor(logging.DEBUG):
                GenericTypeValidator.__LOG.debug(
                    f"Date parse failed value=[{value}], "
                    f"pattern=[{datePattern}], "
                    f"strict=[{strict}] {e}"
                )

        return date

    @staticmethod
    def formatDate0(
        value: str, locale: typing.Any
    ) -> typing.Union[datetime.datetime, datetime.date]:
        if value is None:
            return None

        date = None
        try:
            if locale is not None:
                formatter_short = datetime.datetime.strptime
                formatter_default = datetime.datetime.strptime
            else:
                formatter_short = datetime.datetime.strptime
                formatter_default = datetime.datetime.strptime

            try:
                date = formatter_short(value, "%m/%d/%y")  # SHORT format
            except ValueError:
                date = formatter_default(value, "%b %d, %Y")  # DEFAULT format
        except ValueError as e:
            if GenericTypeValidator.__LOG.isEnabledFor(logging.DEBUG):
                GenericTypeValidator.__LOG.debug(
                    f"Date parse failed value=[{value}], locale=[{locale}] {e}"
                )

        return date

    @staticmethod
    def formatDouble1(value: str, locale: typing.Any) -> float:
        result = None

        if value is not None:
            formatter = None
            if locale is not None:
                formatter = locale.localeconv()
            else:
                import locale as loc

                formatter = loc.localeconv()

            try:
                # Parse the value using the locale's conventions
                num = float(
                    value.replace(formatter["thousands_sep"], "").replace(
                        formatter["decimal_point"], "."
                    )
                )

                if -1 * float("inf") <= num <= float("inf"):
                    result = num
            except ValueError:
                pass

    @staticmethod
    def formatDouble0(value: str) -> float | None:
        if value is None:
            return None

        try:
            return float(value)
        except ValueError:
            return None

    @staticmethod
    def formatFloat1(value: str, locale: typing.Any) -> float:
        result = None

        if value is not None:
            formatter = None
            if locale is not None:
                formatter = locale.localeconv()
            else:
                formatter = locale.localeconv()

            try:
                num = float(value)
                if (
                    -float("inf") < num <= float("inf")
                    and num >= -3.4028235e38
                    and num <= 3.4028235e38
                ):
                    result = num
            except ValueError:
                pass

        return result

    @staticmethod
    def formatFloat0(value: str) -> float | None:
        if value is None:
            return None

        try:
            return float(value)
        except ValueError:
            return None

    @staticmethod
    def formatLong1(value: str, locale: typing.Any) -> int:
        result = None

        if value is not None:
            if locale is not None:
                formatter = locale.localeconv()
            else:
                import locale as loc

                formatter = loc.localeconv()

            try:
                num = int(value)
                if num >= -9223372036854775808 and num <= 9223372036854775807:
                    result = num
            except ValueError:
                pass

        return result

    @staticmethod
    def formatLong0(value: str) -> int | None:
        if value is None:
            return None

        try:
            return int(value)
        except ValueError:
            return None

    @staticmethod
    def formatInt1(value: str, locale: typing.Any) -> int:
        result = None

        if value is not None:
            formatter = None
            if locale is not None:
                formatter = locale.get_number_instance()
            else:
                formatter = Locale.get_default().get_number_instance()

            formatter.set_parse_integer_only(True)
            pos = ParsePosition(0)
            num = formatter.parse(value, pos)

            if (
                pos.get_error_index() == -1
                and pos.get_index() == len(value)
                and -2147483648 <= num.double_value() <= 2147483647
            ):
                result = int(num.int_value())

        return result

    @staticmethod
    def formatInt0(value: str) -> int | None:
        if value is None:
            return None

        try:
            return int(value)
        except ValueError:
            return None

    @staticmethod
    def formatShort1(value: str, locale: typing.Any) -> int | None:
        result = None

        if value is not None:
            if locale is not None:
                formatter = locale.localeconv()
            else:
                import locale as loc

                formatter = loc.localeconv()

            try:
                # Attempt to parse the value as an integer
                num = int(value)
                if -32768 <= num <= 32767:  # -32768 and 32767
                    result = num
            except ValueError:
                pass

        return result

    @staticmethod
    def formatShort0(value: str) -> int | None:
        if value is None:
            return None

        try:
            return int(value) if -32768 <= int(value) <= 32767 else None
        except ValueError:
            return None

    @staticmethod
    def formatByte1(value: str, locale: typing.Any) -> int | None:
        result = None

        if value is not None:
            if locale is not None:
                formatter = locale.localeconv()
            else:
                import locale as loc

                formatter = loc.localeconv()

            try:
                # Attempt to parse the value as an integer
                num = int(value)
                if -128 <= num <= 127:  # -128 = -128, 127 = 127
                    result = num
            except ValueError:
                pass  # If parsing fails, result remains None

        return result

    @staticmethod
    def formatByte0(value: str) -> int:
        if value is None:
            return None

        try:
            return int(value) if -128 <= int(value) <= 127 else None
        except ValueError:
            return None
