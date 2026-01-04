from __future__ import annotations
import time
import locale
import re
import io
import typing
from typing import *
from src.main.org.apache.commons.validator.routines.AbstractCalendarValidator import *


class DateValidator:

    __DATE_VALIDATOR: DateValidator = None

    @staticmethod
    def initialize_fields() -> None:
        DateValidator.__DATE_VALIDATOR: DateValidator = DateValidator()

    def isValid1(self, value: str, locale: typing.Any) -> bool:
        from datetime import datetime

        if value is None:
            return False

        # Determine the locale to use
        if locale is not None:
            loc = locale
        else:
            import locale as locale_module

            loc = locale_module.getdefaultlocale()[0]

        # Try to parse the date string
        # Python doesn't have a direct equivalent to Java's 3
        # We'll try common short date formats
        date_formats = [
            "%m/%d/%y",  # US format: 12/31/99
            "%m/%d/%Y",  # US format: 12/31/1999
            "%d/%m/%y",  # European format: 31/12/99
            "%d/%m/%Y",  # European format: 31/12/1999
            "%Y-%m-%d",  # ISO format: 1999-12-31
            "%d.%m.%y",  # German format: 31.12.99
            "%d.%m.%Y",  # German format: 31.12.1999
        ]

        for fmt in date_formats:
            try:
                datetime.strptime(value, fmt)
                return True
            except ValueError:
                continue

        return False

    def isValid0(self, value: str, datePattern: str, strict: bool) -> bool:
        from datetime import datetime

        if value is None or datePattern is None or len(datePattern) <= 0:
            return False

        try:
            datetime.strptime(value, datePattern)
        except (ValueError, TypeError):
            return False

        if strict and (len(datePattern) != len(value)):
            return False

        return True

    def __init__(self) -> None:
        super().__init__()

    @staticmethod
    def getInstance() -> DateValidator:
        return DateValidator.__DATE_VALIDATOR


DateValidator.initialize_fields()
