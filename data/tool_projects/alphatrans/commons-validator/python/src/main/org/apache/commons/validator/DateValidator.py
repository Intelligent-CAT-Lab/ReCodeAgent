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
        if value is None:
            return False

        from datetime import datetime
        import locale as py_locale

        # Set the locale for date formatting
        if locale is not None:
            py_locale.setlocale(py_locale.LC_TIME, locale)
        else:
            py_locale.setlocale(py_locale.LC_TIME, py_locale.getdefaultlocale())

        # Define the date format (SHORT equivalent in Java)
        date_format = "%x"  # Locale's appropriate date representation

        try:
            # Try to parse the date
            datetime.strptime(value, date_format)
        except ValueError:
            return False

        return True

    def isValid0(self, value: str, datePattern: str, strict: bool) -> bool:
        if value is None or datePattern is None or len(datePattern) <= 0:
            return False

        from datetime import datetime

        try:
            parsed_date = datetime.strptime(value, datePattern)
        except ValueError:
            return False

        if strict and len(datePattern) != len(value):
            return False

        return True

    def __init__(self) -> None:
        super().__init__()

    @staticmethod
    def getInstance() -> DateValidator:
        return DateValidator.__DATE_VALIDATOR


DateValidator.initialize_fields()
