from __future__ import annotations
import copy
import re
from io import IOBase
import io
import typing
from typing import *
import os
from src.main.org.apache.commons.csv.CSVFormat import *
from src.main.org.apache.commons.csv.Constants import *
from src.main.org.apache.commons.csv.IOUtils import *


class CSVPrinter(io.BufferedIOBase):

    __newRecord: bool = True
    __format: CSVFormat = None

    __appendable: typing.Union[typing.List, io.TextIOBase] = None

    def flush(self) -> None:
        if hasattr(self.__appendable, "flush"):
            self.__appendable.flush()

    def close(self) -> None:
        self.close1(True)

    def printRecords1(self, values: typing.List[typing.Any]) -> None:

        pass  # LLM could not translate this method

    def printRecords0(self, values: typing.Iterable[typing.Any]) -> None:
        for value in values:
            self.__printRecordObject(value)

    def printRecord2(self, values: typing.Iterable[typing.Any]) -> None:
        for value in values:
            self.print(value)
        self.println()

    def printRecord1(self, values: typing.List[typing.Any]) -> None:
        self.printRecord0(values)

    def printRecord0(self, values: typing.Iterable[typing.Any]) -> None:
        for value in values:
            self.print_(value)
        self.println()

    def println(self) -> None:
        self.__format.println(self.__appendable)
        self.__newRecord = True

    def printComment(self, comment: str) -> None:
        if comment is None or not self.__format.isCommentMarkerSet():
            return
        if not self.__newRecord:
            self.println()
        self.__appendable.write(self.__format.getCommentMarker())
        self.__appendable.write(SP)
        i = 0
        while i < len(comment):
            c = comment[i]
            if c == CR:
                if i + 1 < len(comment) and comment[i + 1] == LF:
                    i += 1
                self.println()
                self.__appendable.write(self.__format.getCommentMarker())
                self.__appendable.write(SP)
            elif c == LF:
                self.println()
                self.__appendable.write(self.__format.getCommentMarker())
                self.__appendable.write(SP)
            else:
                self.__appendable.write(c)
            i += 1
        self.println()

    def print_(self, value: typing.Any) -> None:
        self.__format.print2(value, self.__appendable, self.__newRecord)
        self.__newRecord = False

    def getOut(self) -> typing.Union[typing.List, io.TextIOBase]:
        return self.__appendable

    def close1(self, flush: bool) -> None:
        if flush or self.__format.getAutoFlush():
            self.flush()
        if hasattr(self.__appendable, "close"):
            self.__appendable.close()

    def close0(self) -> None:
        self.close1(False)

    def __init__(self, appendable: typing.Union[typing.List, io.TextIOBase], format_: CSVFormat) -> None:
        if appendable is None:
            raise ValueError("appendable cannot be None")
        if format_ is None:
            raise ValueError("format_ cannot be None")

        self.__appendable = appendable
        self.__format = format_.copy()

        headerComments = self.__format.getHeaderComments()
        if headerComments is not None:
            for line in headerComments:
                self.printComment(line)

        if self.__format.getHeader() is not None and not self.__format.getSkipHeaderRecord():
            self.printRecord1(self.__format.getHeader())

    def __printRecordObject(self, value: typing.Any) -> None:
        if isinstance(value, list) or isinstance(value, tuple):
            self.printRecord1(value)
        elif isinstance(value, typing.Iterable) and not isinstance(value, str):
            self.printRecord0(value)
        else:
            self.printRecord1([value])
