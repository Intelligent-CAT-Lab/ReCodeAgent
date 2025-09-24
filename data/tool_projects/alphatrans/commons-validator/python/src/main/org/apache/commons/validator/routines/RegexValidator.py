from __future__ import annotations
import re
import enum
import io
import typing
from typing import *


class RegexValidator:

    __patterns: typing.List[re.Pattern] = None

    __serialVersionUID: int = -8832409930574867162

    def toString(self) -> str:
        buffer = ["RegexValidator{"]
        for i, pattern in enumerate(self.__patterns):
            if i > 0:
                buffer.append(",")
            buffer.append(pattern.pattern)
        buffer.append("}")
        return "".join(buffer)

    def validate(self, value: str) -> str:
        if value is None:
            return None
        for pattern in self.__patterns:
            matcher = pattern.match(value)
            if matcher:
                count = len(matcher.groups())
                if count == 1:
                    return matcher.group(1)
                buffer = []
                for j in range(count):
                    component = matcher.group(j + 1)
                    if component is not None:
                        buffer.append(component)
                return "".join(buffer)
        return None

    def match(self, value: str) -> typing.List[typing.List[str]]:
        if value is None:
            return None
        for pattern in self.__patterns:
            matcher = pattern.match(value)
            if matcher:
                count = len(matcher.groups())
                groups = [matcher.group(j + 1) for j in range(count)]
                return groups
        return None

    def isValid(self, value: str) -> bool:
        if value is None:
            return False
        for pattern in self.__patterns:
            if pattern.match(value):
                return True
        return False

    @staticmethod
    def RegexValidator3(regex: str) -> RegexValidator:
        return RegexValidator.RegexValidator2(regex, True)

    @staticmethod
    def RegexValidator2(regex: str, caseSensitive: bool) -> RegexValidator:
        return RegexValidator([regex], caseSensitive)

    @staticmethod
    def RegexValidator1(regexs: typing.List[str]) -> RegexValidator:
        return RegexValidator(regexs, True)

    def __init__(self, regexs: typing.List[str], caseSensitive: bool) -> None:
        if not regexs or len(regexs) == 0:
            raise ValueError("Regular expressions are missing")

        self.__patterns = []
        flags = 0 if caseSensitive else re.IGNORECASE

        for i, regex in enumerate(regexs):
            if not regex or len(regex) == 0:
                raise ValueError(f"Regular expression[{i}] is missing")
            self.__patterns.append(re.compile(regex, flags))
