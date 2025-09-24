from __future__ import annotations
import re
import os
import enum
import io
from src.main.org.apache.commons.validator.routines.checkdigit.CheckDigit import *
from src.main.org.apache.commons.validator.routines.checkdigit.CheckDigitException import *


class IBANCheckDigit(CheckDigit):

    IBAN_CHECK_DIGIT: CheckDigit = None
    __MODULUS: int = 97
    __MAX: int = 999999999
    __MAX_ALPHANUMERIC_VALUE: int = 35  # Equivalent to Character.getNumericValue('Z')
    __serialVersionUID: int = -3600191725934382801
    __MIN_CODE_LEN: int = 5

    @staticmethod
    def initialize_fields() -> None:
        IBANCheckDigit.IBAN_CHECK_DIGIT: CheckDigit = IBANCheckDigit()

    def calculate(self, code: str) -> str:
        if code is None or len(code) < self.__MIN_CODE_LEN:
            raise CheckDigitException.CheckDigitException1(
                f"Invalid Code length={0 if code is None else len(code)}"
            )
        code = (
            code[:2] + "00" + code[4:]
        )  # Replace characters at positions 2 and 3 with "00"
        modulus_result = self.__calculateModulus(code)
        char_value = 98 - modulus_result
        check_digit = str(char_value)
        return check_digit if char_value > 9 else "0" + check_digit

    def isValid(self, code: str) -> bool:
        if code is None or len(code) < self.__MIN_CODE_LEN:
            return False
        check = code[2:4]  # Equivalent to code.substring(2, 4) in Java
        if check in {"00", "01", "99"}:
            return False
        try:
            modulus_result = self.__calculateModulus(code)
            return modulus_result == 1
        except CheckDigitException:
            return False

    def __init__(self) -> None:
        pass

    def __calculateModulus(self, code: str) -> int:
        reformatted_code = (
            code[4:] + code[:4]
        )  # Rearrange the code as in the Java version
        total = 0
        for i, char in enumerate(reformatted_code):
            char_value = (
                int(char) if char.isdigit() else ord(char.upper()) - ord("A") + 10
            )
            if char_value < 0 or char_value > self.__MAX_ALPHANUMERIC_VALUE:
                raise CheckDigitException.CheckDigitException1(
                    f"Invalid Character[{i}] = '{char}'"
                )
            total = (total * 100 if char_value > 9 else total * 10) + char_value
            if total > self.__MAX:
                total %= self.__MODULUS
        return total % self.__MODULUS


IBANCheckDigit.initialize_fields()
