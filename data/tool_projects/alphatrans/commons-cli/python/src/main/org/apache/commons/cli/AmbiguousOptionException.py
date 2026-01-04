from __future__ import annotations
import re
import os
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
        self._AmbiguousOptionException__matchingOptions = matchingOptions

    @staticmethod
    def __createMessage(option: str, matchingOptions: typing.Collection[str]) -> str:
        buf = io.StringIO()
        buf.write("Ambiguous option: '")
        buf.write(option)
        buf.write("'  (could be: ")

        it = iter(matchingOptions)
        for item in it:
            buf.write("'")
            buf.write(item)
            buf.write("'")
            try:
                next_item = next(it)
                buf.write(", ")
                # Put the item back by creating a new iterator with it prepended
                it = iter([next_item] + list(it))
            except StopIteration:
                pass

        buf.write(")")

        result = buf.getvalue()
        buf.close()
        return result
