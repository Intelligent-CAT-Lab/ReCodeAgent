from __future__ import annotations
import locale
import re
import io
import typing
from typing import *
from src.main.org.apache.commons.validator.routines.CodeValidator import *
from src.main.org.apache.commons.validator.routines.checkdigit.CheckDigit import *
from src.main.org.apache.commons.validator.routines.checkdigit.ISINCheckDigit import *


class ISINValidator:

    __checkCountryCode: bool = False

    __SPECIALS: typing.List[typing.List[str]] = [
        "EZ",  # http://www.anna-web.org/standards/isin-iso-6166/
        "XS",  # https://www.isin.org/isin/
    ]
    __CCODES: typing.List[typing.List[str]] = [
        locale.upper() for locale in typing.cast(typing.List[str], __import__("locale").locale_alias.keys())
    ]
    __ISIN_VALIDATOR_TRUE: ISINValidator = None
    __ISIN_VALIDATOR_FALSE: ISINValidator = None
    __ISIN_REGEX: str = "([A-Z]{2}[A-Z0-9]{9}[0-9])"
    __serialVersionUID: int = -5964391439144260936
    __VALIDATOR: CodeValidator = CodeValidator.CodeValidator4(__ISIN_REGEX, 12, ISINCheckDigit.ISIN_CHECK_DIGIT)

    @staticmethod
    def run_static_init():
        ISINValidator.__CCODES.sort()
        ISINValidator.__SPECIALS.sort()

    def validate(self, code: str) -> typing.Any:
        validate = self.__VALIDATOR.validate(code)
        if validate is not None and self.__checkCountryCode:
            return validate if self.__checkCode(code[0:2]) else None
        return validate

    def isValid(self, code: str) -> bool:
        valid = self.__VALIDATOR.isValid(code)
        if valid and self.__checkCountryCode:
            return self.__checkCode(code[0:2])
        return valid

    @staticmethod
    def getInstance(checkCountryCode: bool) -> ISINValidator:
        return ISINValidator.__ISIN_VALIDATOR_TRUE if checkCountryCode else ISINValidator.__ISIN_VALIDATOR_FALSE

    def __checkCode(self, code: str) -> bool:
        import bisect

        # Binary search requires sorted arrays
        # Assuming __CCODES and __SPECIALS are already sorted
        ccodes_sorted = sorted(self.__CCODES)
        specials_sorted = sorted(self.__SPECIALS)

        # bisect_left returns the insertion point; if the element exists,
        # we need to verify it matches
        ccodes_idx = bisect.bisect_left(ccodes_sorted, code)
        in_ccodes = ccodes_idx < len(ccodes_sorted) and ccodes_sorted[ccodes_idx] == code

        specials_idx = bisect.bisect_left(specials_sorted, code)
        in_specials = specials_idx < len(specials_sorted) and specials_sorted[specials_idx] == code

        return in_ccodes or in_specials

    def __init__(self, checkCountryCode: bool) -> None:
        self.__checkCountryCode = checkCountryCode


ISINValidator.run_static_init()
