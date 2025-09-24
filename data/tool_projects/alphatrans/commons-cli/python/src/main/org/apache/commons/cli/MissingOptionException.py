from __future__ import annotations
import re
import enum
from io import StringIO
import io
import typing
from typing import *
from src.main.org.apache.commons.cli.ParseException import *


class MissingOptionException(ParseException):

    __missingOptions: typing.List[typing.Any] = None

    __serialVersionUID: int = 8161889051578563249

    def getMissingOptions(self) -> typing.List[typing.Any]:
        return self.__missingOptions

    @staticmethod
    def MissingOptionException1(
        constructorId: int, missingOptions: typing.List[typing.Any], message: str
    ) -> MissingOptionException:
        if constructorId == 1:
            return MissingOptionException(
                constructorId,
                missingOptions,
                MissingOptionException.__createMessage(missingOptions),
            )
        return MissingOptionException(constructorId, missingOptions, message)

    def __init__(
        self, constructorId: int, missingOptions: typing.List[typing.Any], message: str
    ) -> None:
        super().__init__(message)
        if constructorId == 1:
            self.__missingOptions = missingOptions

    @staticmethod
    def __createMessage(missingOptions: typing.List[typing.Any]) -> str:
        buf = io.StringIO()
        buf.write("Missing required option")
        buf.write("" if len(missingOptions) == 1 else "s")
        buf.write(": ")

        it = iter(missingOptions)
        for i, option in enumerate(missingOptions):
            buf.write(str(option))
            if i < len(missingOptions) - 1:
                buf.write(", ")

        return buf.getvalue()
