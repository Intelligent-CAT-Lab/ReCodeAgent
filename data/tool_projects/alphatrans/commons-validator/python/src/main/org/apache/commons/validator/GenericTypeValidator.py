from __future__ import annotations
import time
import locale
import re
import os
import decimal
import numbers
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
    def formatDate1(value: str, datePattern: str, strict: bool) -> typing.Union[datetime.datetime, datetime.date]:
        date = None

        if value is None or datePattern is None or len(datePattern) == 0:
            return None

        try:
            date = datetime.datetime.strptime(value, datePattern)

            if strict and len(datePattern) != len(value):
                date = None
        except ValueError as e:
            if GenericTypeValidator.__LOG.isEnabledFor(logging.DEBUG):
                GenericTypeValidator.__LOG.debug(
                    f"Date parse failed value=[{value}], " f"pattern=[{datePattern}], " f"strict=[{strict}] {e}"
                )

        return date

    @staticmethod
    def formatDate0(value: str, locale: typing.Any) -> typing.Union[datetime.datetime, datetime.date]:
        date = None

        if value is None:
            return None

        try:
            from babel.dates import parse_date
            import babel

            # Determine the locale to use
            if locale is not None:
                locale_str = locale
            else:
                locale_str = babel.default_locale()

            # Try parsing with different formats
            # Babel doesn't have direct SHORT/DEFAULT format equivalents,
            # so we'll try common date formats
            try:
                # Try short format (e.g., "1/1/23", "01/01/2023")
                date = parse_date(value, locale=locale_str)
            except (ValueError, babel.core.UnknownLocaleError):
                # Try with dateutil as fallback for more flexible parsing
                from dateutil import parser

                date = parser.parse(value, dayfirst=True).date()

        except Exception as e:
            if GenericTypeValidator.__LOG.isEnabledFor(logging.DEBUG):
                GenericTypeValidator.__LOG.debug(f"Date parse failed value=[{value}], locale=[{locale}] {e}")

        return date

    @staticmethod
    def formatDouble1(value: str, locale: typing.Any) -> float:
        result = None

        if value is not None:
            formatter = None
            if locale is not None:
                import locale as locale_module

                # Set the locale temporarily for parsing
                old_locale = locale_module.getlocale(locale_module.LC_NUMERIC)
                try:
                    locale_module.setlocale(locale_module.LC_NUMERIC, locale)
                    formatter = locale_module
                except:
                    formatter = None

            # In Python, we'll use a simpler approach with float conversion
            # since Python doesn't have the exact same NumberFormat/ParsePosition API
            try:
                # Remove any grouping separators and handle locale-specific decimal points
                if locale is not None:
                    import locale as locale_module

                    # Get the decimal point character for the locale
                    conv = locale_module.localeconv()
                    decimal_point = conv.get("decimal_point", ".")
                    thousands_sep = conv.get("thousands_sep", ",")
                    # Remove thousands separator and replace decimal point
                    cleaned_value = value.replace(thousands_sep, "").replace(decimal_point, ".")
                else:
                    cleaned_value = value

                num = float(cleaned_value)

                # Check if the value is within valid double range
                max_double = 1.7976931348623157e308  # Double.MAX_VALUE equivalent
                if -max_double <= num <= max_double:
                    result = num
            except (ValueError, AttributeError):
                result = None
            finally:
                if locale is not None and "old_locale" in locals():
                    try:
                        locale_module.setlocale(locale_module.LC_NUMERIC, old_locale)
                    except:
                        pass

        return result

    @staticmethod
    def formatDouble0(value: str) -> float:
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
                import locale as locale_module

                # Set the locale temporarily for parsing
                old_locale = locale_module.getlocale(locale_module.LC_NUMERIC)
                try:
                    locale_module.setlocale(locale_module.LC_NUMERIC, locale)
                    formatter = locale_module
                except:
                    formatter = None

            # Try to parse the float value
            try:
                # Remove any grouping separators and parse
                if formatter is not None:
                    # Get locale-specific decimal point
                    import locale as locale_module

                    num = locale_module.atof(value)
                else:
                    num = float(value)

                # Check if the entire string was parsed and value is within float range
                MAX_FLOAT = 3.4028235e38  # 3.4028235E38 equivalent
                if num >= -MAX_FLOAT and num <= MAX_FLOAT:
                    result = float(num)
            except (ValueError, Exception):
                result = None
            finally:
                if locale is not None and formatter is not None:
                    try:
                        locale_module.setlocale(locale_module.LC_NUMERIC, old_locale)
                    except:
                        pass

        return result

    @staticmethod
    def formatFloat0(value: str) -> float:
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
            import locale as locale_module

            # Save current locale
            old_locale = None
            if locale is not None:
                try:
                    old_locale = locale_module.getlocale(locale_module.LC_NUMERIC)
                    # Set the specified locale
                    locale_module.setlocale(locale_module.LC_NUMERIC, locale)
                except:
                    pass

            try:
                # Parse the value as a number
                # Remove any grouping separators and handle locale-specific formatting
                cleaned_value = value.strip()

                # Try to parse as integer/long
                num = locale_module.atof(cleaned_value)

                # Check if the entire string was parsed and value is within Long range
                if (
                    num >= -9223372036854775808 and num <= 9223372036854775807  # -9223372036854775808
                ):  # 9223372036854775807
                    result = int(num)
            except (ValueError, locale_module.Error):
                # Parsing failed
                result = None
            finally:
                # Restore original locale
                if old_locale is not None:
                    try:
                        locale_module.setlocale(locale_module.LC_NUMERIC, old_locale)
                    except:
                        pass

        return result

    @staticmethod
    def formatLong0(value: str) -> int:
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
                import locale as locale_module

                # Set the locale temporarily
                old_locale = locale_module.getlocale(locale_module.LC_NUMERIC)
                try:
                    locale_module.setlocale(locale_module.LC_NUMERIC, locale)
                    formatter = locale_module
                except:
                    formatter = None

            # Parse the number
            try:
                # Remove any grouping separators and parse
                if formatter is not None:
                    import locale as locale_module

                    # Get the decimal point for this locale
                    conv = locale_module.localeconv()
                    thousands_sep = conv.get("thousands_sep", ",")
                    decimal_point = conv.get("decimal_point", ".")

                    # Remove thousands separators
                    cleaned_value = value.replace(thousands_sep, "")

                    # Check if there's a decimal point - if so, reject (integer only)
                    if decimal_point in cleaned_value:
                        result = None
                    else:
                        num = float(cleaned_value)

                        # Check bounds
                        if num >= -2147483648 and num <= 2147483647:
                            result = int(num)
                else:
                    # Default parsing without locale
                    cleaned_value = value.replace(",", "")
                    if "." in cleaned_value:
                        result = None
                    else:
                        num = float(cleaned_value)
                        if num >= -2147483648 and num <= 2147483647:
                            result = int(num)
            except (ValueError, AttributeError):
                result = None
            finally:
                if locale is not None:
                    try:
                        locale_module.setlocale(locale_module.LC_NUMERIC, old_locale)
                    except:
                        pass

        return result

    @staticmethod
    def formatInt0(value: str) -> int:
        if value is None:
            return None

        try:
            return int(value)
        except ValueError:
            return None

    @staticmethod
    def formatShort1(value: str, locale: typing.Any) -> int:
        result = None

        if value is not None:
            import locale as locale_module

            # Save current locale
            old_locale = locale_module.getlocale(locale_module.LC_NUMERIC)

            try:
                # Set locale if provided
                if locale is not None:
                    try:
                        locale_module.setlocale(locale_module.LC_NUMERIC, locale)
                    except:
                        pass  # Use default if locale setting fails

                # Try to parse the value
                try:
                    # Remove any grouping separators and handle locale-specific decimal
                    cleaned_value = value.strip()

                    # Parse as float first to check range
                    num = locale_module.atof(cleaned_value)

                    # Check if it's an integer (no decimal part)
                    if num == int(num):
                        # Check if within Short range (-32768 to 32767)
                        if -32768 <= num <= 32767:
                            result = int(num)
                except (ValueError, locale_module.Error):
                    pass  # Return None if parsing fails

            finally:
                # Restore original locale
                try:
                    locale_module.setlocale(locale_module.LC_NUMERIC, old_locale)
                except:
                    pass

        return result

    @staticmethod
    def formatShort0(value: str) -> int:
        if value is None:
            return None

        try:
            return int(value)
        except ValueError:
            return None

    @staticmethod
    def formatByte1(value: str, locale: typing.Any) -> int:
        result = None

        if value is not None:
            import locale as locale_module

            # Save current locale
            old_locale = None
            if locale is not None:
                old_locale = locale_module.getlocale(locale_module.LC_NUMERIC)
                try:
                    # Try to set the locale (locale should be a string like 'en_US.UTF-8')
                    locale_module.setlocale(locale_module.LC_NUMERIC, locale)
                except:
                    pass

            try:
                # Parse the number
                # Remove any grouping separators and handle decimal point
                cleaned_value = value.strip()

                # Try to parse as float first to handle the parsing
                num = locale_module.atof(cleaned_value)

                # Check if it's an integer value (no decimal part)
                if num == int(num):
                    # Check if within byte range (-128 to 127)
                    if -128 <= num <= 127:
                        result = int(num)
            except (ValueError, locale_module.Error):
                # Parsing failed
                pass
            finally:
                # Restore old locale
                if old_locale is not None:
                    try:
                        locale_module.setlocale(locale_module.LC_NUMERIC, old_locale)
                    except:
                        pass

        return result

    @staticmethod
    def formatByte0(value: str) -> int:
        if value is None:
            return None

        try:
            return int(value)
        except ValueError:
            return None
