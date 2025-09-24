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

    __SPECIALS: List[str] = [
        "EZ",  # http://www.anna-web.org/standards/isin-iso-6166/
        "XS",  # https://www.isin.org/isin/
    ]
    __CCODES: List[str] = list(locale.locale_alias.keys())
    __ISIN_VALIDATOR_TRUE: ISINValidator = None
    __ISIN_VALIDATOR_FALSE: ISINValidator = None
    __ISIN_REGEX: str = "([A-Z]{2}[A-Z0-9]{9}[0-9])"
    __serialVersionUID: int = -5964391439144260936
    __VALIDATOR: CodeValidator = CodeValidator.CodeValidator4(
        __ISIN_REGEX, 12, ISINCheckDigit.ISIN_CHECK_DIGIT
    )

    @staticmethod
    def run_static_init():
        ISINValidator.__CCODES.sort()  # we cannot assume the codes are sorted
        ISINValidator.__SPECIALS.sort()  # Just in case ...

    @staticmethod
    def initialize_fields() -> None:
        ISINValidator.__ISIN_VALIDATOR_TRUE: ISINValidator = ISINValidator(True)

        ISINValidator.__ISIN_VALIDATOR_FALSE: ISINValidator = ISINValidator(False)

    def validate(self, code: str) -> typing.Any:
        validate = self.__VALIDATOR.validate(code)
        if validate is not None and self.__checkCountryCode:
            return validate if self.__checkCode(code[:2]) else None
        return validate

    def isValid(self, code: str) -> bool:
        valid = self.__VALIDATOR.isValid(code)
        if valid and self.__checkCountryCode:
            return self.__checkCode(code[:2])
        return valid

    @staticmethod
    def getInstance(checkCountryCode: bool) -> ISINValidator:
        return (
            ISINValidator.__ISIN_VALIDATOR_TRUE
            if checkCountryCode
            else ISINValidator.__ISIN_VALIDATOR_FALSE
        )

    def __checkCode(self, code: str) -> bool:
        return code in self.__CCODES or code in self.__SPECIALS

    def __init__(self, checkCountryCode: bool) -> None:
        self.__checkCountryCode = checkCountryCode


ISINValidator.run_static_init()

ISINValidator.initialize_fields()
