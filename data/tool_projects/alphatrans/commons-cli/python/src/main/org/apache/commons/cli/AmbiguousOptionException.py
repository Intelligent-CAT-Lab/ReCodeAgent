from __future__ import annotations
import re
from io import StringIO
import io
import typing
from src.main.org.apache.commons.cli.UnrecognizedOptionException import *


class AmbiguousOptionException(UnrecognizedOptionException):

    __matchingOptions: typing.Collection[str] = None

    __serialVersionUID: int = 5829816121277947229

    def getMatchingOptions(self) -> typing.Collection[str]:
        return self.__matchingOptions

    def __init__(self, option: str, matchingOptions: typing.Collection[str]) -> None:
        super().__init__(self.__createMessage(option, matchingOptions), option)
        self.__matchingOptions = matchingOptions

    @staticmethod
    def __createMessage(option: str, matchingOptions: typing.Collection[str]) -> str:
        buf = io.StringIO()
        buf.write(f"Ambiguous option: '{option}'  (could be: ")

        it = iter(matchingOptions)
        for match in it:
            buf.write(f"'{match}'")
            if match != list(matchingOptions)[-1]:  # Check if it's not the last element
                buf.write(", ")

        buf.write(")")
        return buf.getvalue()
