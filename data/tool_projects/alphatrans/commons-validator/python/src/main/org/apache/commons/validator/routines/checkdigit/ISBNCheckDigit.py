from __future__ import annotations
import re
import io
from src.main.org.apache.commons.validator.routines.checkdigit.CheckDigit import *
from src.main.org.apache.commons.validator.routines.checkdigit.CheckDigitException import *
from src.main.org.apache.commons.validator.routines.checkdigit.EAN13CheckDigit import *
from src.main.org.apache.commons.validator.routines.checkdigit.ISBN10CheckDigit import *


class ISBNCheckDigit(CheckDigit):

    ISBN_CHECK_DIGIT: CheckDigit = None
    ISBN13_CHECK_DIGIT: CheckDigit = EAN13CheckDigit.EAN13_CHECK_DIGIT
    ISBN10_CHECK_DIGIT: CheckDigit = None  # LLM could not translate this field

    __serialVersionUID: int = 1391849166205184558

    @staticmethod
    def initialize_fields() -> None:
        ISBNCheckDigit.ISBN_CHECK_DIGIT: CheckDigit = ISBNCheckDigit()

    def isValid(self, code: str) -> bool:
        if code is None:
            return False
        elif len(code) == 10:
            return ISBNCheckDigit.ISBN10_CHECK_DIGIT.isValid(code)
        elif len(code) == 13:
            return ISBNCheckDigit.ISBN13_CHECK_DIGIT.isValid(code)
        else:
            return False

    def calculate(self, code: str) -> str:
        if code is None or len(code) == 0:
            raise CheckDigitException.CheckDigitException1("ISBN Code is missing")
        elif len(code) == 9:
            return self.ISBN10_CHECK_DIGIT.calculate(code)
        elif len(code) == 12:
            return self.ISBN13_CHECK_DIGIT.calculate(code)
        else:
            raise CheckDigitException.CheckDigitException1(f"Invalid ISBN Length = {len(code)}")


ISBNCheckDigit.initialize_fields()
