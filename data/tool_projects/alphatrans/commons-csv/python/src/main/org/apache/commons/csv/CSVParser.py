from __future__ import annotations
import time
import copy
import re
import collections
import enum
import pathlib
from io import IOBase
from io import StringIO
import io
from io import BytesIO
import numbers
import typing
from typing import *
import os
import urllib
from src.main.org.apache.commons.csv.CSVFormat import *
from src.main.org.apache.commons.csv.CSVRecord import *
from src.main.org.apache.commons.csv.Constants import *
from src.main.org.apache.commons.csv.DuplicateHeaderMode import *
from src.main.org.apache.commons.csv.ExtendedBufferedReader import *
from src.main.org.apache.commons.csv.Lexer import *
from src.main.org.apache.commons.csv.QuoteMode import *
from src.main.org.apache.commons.csv.Token import *


class CSVParser:

    __reusableToken: Token = Token()
    __characterOffset: int = 0

    __recordNumber: int = 0

    __recordList: typing.List[str] = []

    __csvRecordIterator: CSVRecordIterator = None

    __lexer: Lexer = None

    __headers: Headers = None

    __format: CSVFormat = None

    __trailerComment: str = ""

    __headerComment: str = ""

    def iterator(self) -> typing.Iterator[CSVRecord]:
        return self.__csvRecordIterator

    def close(self) -> None:
        if self.__lexer is not None:
            self.__lexer.close()

    @staticmethod
    def parse5(
        url: typing.Union[
            urllib.parse.ParseResult,
            urllib.parse.SplitResult,
            urllib.parse.DefragResult,
            str,
        ],
        charset: str,
        format_: CSVFormat,
    ) -> CSVParser:
        if url is None:
            raise ValueError("url cannot be None")
        if charset is None:
            raise ValueError("charset cannot be None")
        if format_ is None:
            raise ValueError("format cannot be None")

        with urllib.request.urlopen(url) as response:
            reader = io.TextIOWrapper(response, encoding=charset)
            return CSVParser.CSVParser1(reader, format_)

    @staticmethod
    def parse2(path: Path, charset: str, format_: CSVFormat) -> CSVParser:
        if path is None:
            raise ValueError("path cannot be None")
        if format_ is None:
            raise ValueError("format cannot be None")
        with path.open("rb") as inputStream:
            return CSVParser.parse1(inputStream, charset, format_)

    @staticmethod
    def parse1(
        inputStream: typing.Union[io.BytesIO, io.StringIO, io.BufferedReader],
        charset: str,
        format_: CSVFormat,
    ) -> CSVParser:
        if inputStream is None:
            raise ValueError("inputStream cannot be None")
        if format_ is None:
            raise ValueError("format cannot be None")
        reader = io.TextIOWrapper(inputStream, encoding=charset)
        return CSVParser.parse3(reader, format_)

    def __addRecordValue(self, lastRecord: bool) -> None:
        input_ = self.__format.trim1(self.__reusableToken.content.getvalue())
        if lastRecord and not input_ and self.__format.getTrailingDelimiter():
            return
        self.__recordList.append(self.__handleNull(input_))

    def stream(self) -> typing.Iterable[CSVRecord]:
        return iter(self.iterator())

    def isClosed(self) -> bool:
        return self.__lexer.isClosed()

    def hasTrailerComment(self) -> bool:
        return self.__trailerComment is not None

    def hasHeaderComment(self) -> bool:
        return self.__headerComment is not None

    def getTrailerComment(self) -> str:
        return self.__trailerComment

    def getRecords(self) -> typing.List[CSVRecord]:
        return list(self.stream())

    def getRecordNumber(self) -> int:
        return self.__recordNumber

    def getHeaderNames(self) -> typing.List[str]:
        return list(self.__headers.headerNames)

    def getHeaderMap(self) -> typing.Optional[typing.Dict[str, int]]:
        if self.__headers.headerMap is None:
            return None
        map = self.__createEmptyHeaderMap()
        map.update(self.__headers.headerMap)
        return map

    def getHeaderComment(self) -> str:
        return self.__headerComment

    def getFirstEndOfLine(self) -> str:
        return self.__lexer.getFirstEol()

    def getCurrentLineNumber(self) -> int:
        return self.__lexer.getCurrentLineNumber()

    @staticmethod
    def CSVParser1(
        reader: typing.Union[io.TextIOWrapper, io.BufferedReader, io.TextIOBase],
        format_: CSVFormat,
    ) -> CSVParser:
        return CSVParser(reader, format_, 0, 1)

    def __init__(
        self,
        reader: typing.Union[io.TextIOWrapper, io.BufferedReader, io.TextIOBase],
        format_: CSVFormat,
        characterOffset: int,
        recordNumber: int,
    ) -> None:
        if reader is None:
            raise ValueError("reader cannot be None")
        if format_ is None:
            raise ValueError("format cannot be None")

        self.__format = format_.copy()
        self.__lexer = Lexer(format_, ExtendedBufferedReader(reader))
        self.__csvRecordIterator = CSVRecordIterator()
        self.__headers = self.__createHeaders()
        self.__characterOffset = characterOffset
        self.__recordNumber = recordNumber - 1

    @staticmethod
    def parse4(string: str, format_: CSVFormat) -> CSVParser:
        if string is None:
            raise ValueError("string must not be None")
        if format_ is None:
            raise ValueError("format must not be None")

        return CSVParser.CSVParser1(StringIO(string), format_)

    @staticmethod
    def parse3(
        reader: typing.Union[io.TextIOWrapper, io.BufferedReader, io.TextIOBase],
        format_: CSVFormat,
    ) -> CSVParser:
        return CSVParser.CSVParser1(reader, format_)

    @staticmethod
    def parse0(file: pathlib.Path, charset: str, format_: CSVFormat) -> CSVParser:
        if file is None:
            raise ValueError("file cannot be None")
        if format_ is None:
            raise ValueError("format cannot be None")
        return CSVParser.parse2(file, charset, format_)

    def __isStrictQuoteMode(self) -> bool:
        return self.__format is not None and self.__format.getQuoteMode() in {
            QuoteMode.ALL_NON_NULL,
            QuoteMode.NON_NUMERIC,
        }

    def __handleNull(self, input_: str) -> str:
        is_quoted = self.__reusableToken.isQuoted
        null_string = self.__format.getNullString()
        strict_quote_mode = self.__isStrictQuoteMode()

        if input_ == null_string:
            return input_ if strict_quote_mode and is_quoted else None

        return (
            None
            if strict_quote_mode
            and null_string is None
            and not input_
            and not is_quoted
            else input_
        )

    def __createHeaders(self) -> Headers:
        hdr_map = None
        header_names = None
        format_header = self.__format.getHeader()

        if format_header is not None:
            hdr_map = self.__createEmptyHeaderMap()
            header_record = None

            if len(format_header) == 0:
                next_record = self.nextRecord()
                if next_record is not None:
                    header_record = next_record.values()
                    self.__headerComment = next_record.getComment()
            else:
                if self.__format.getSkipHeaderRecord():
                    next_record = self.nextRecord()
                    if next_record is not None:
                        self.__headerComment = next_record.getComment()
                header_record = format_header

            if header_record is not None:
                observed_missing = False
                for i, header in enumerate(header_record):
                    blank_header = CSVFormat.isBlank(header)
                    if blank_header and not self.__format.getAllowMissingColumnNames():
                        raise ValueError(f"A header name is missing in {header_record}")

                    contains_header = (
                        blank_header if observed_missing else header in hdr_map
                    )
                    header_mode = self.__format.getDuplicateHeaderMode()
                    duplicates_allowed = header_mode == DuplicateHeaderMode.ALLOW_ALL
                    empty_duplicates_allowed = (
                        header_mode == DuplicateHeaderMode.ALLOW_EMPTY
                    )

                    if (
                        contains_header
                        and not duplicates_allowed
                        and not (blank_header and empty_duplicates_allowed)
                    ):
                        raise ValueError(
                            f'The header contains a duplicate name: "{header}" in {header_record}. '
                            f"If this is valid then use CSVFormat.Builder.setDuplicateHeaderMode()."
                        )

                    observed_missing |= blank_header
                    if header is not None:
                        hdr_map[header] = i
                        if header_names is None:
                            header_names = []
                        header_names.append(header)

        if header_names is None:
            header_names = []  # immutable
        else:
            header_names = tuple(header_names)  # make immutable

        return Headers(hdr_map, header_names)

    def __createEmptyHeaderMap(self) -> typing.Dict[str, int]:
        return (
            dict()
            if not self.__format.getIgnoreHeaderCase()
            else collections.defaultdict(int)
        )

    def nextRecord(self) -> CSVRecord:
        result = None
        self.__recordList.clear()
        sb = None
        start_char_position = (
            self.__lexer.getCharacterPosition() + self.__characterOffset
        )

        while True:
            self.__reusableToken.reset()
            self.__lexer.nextToken(self.__reusableToken)

            if self.__reusableToken.type == Constants.TOKEN:
                self.__addRecordValue(False)
            elif self.__reusableToken.type == Constants.EORECORD:
                self.__addRecordValue(True)
            elif self.__reusableToken.type == Constants.EOF:
                if self.__reusableToken.isReady:
                    self.__addRecordValue(True)
                elif sb is not None:
                    self.__trailerComment = sb.getvalue()
                break
            elif self.__reusableToken.type == Constants.INVALID:
                raise IOError(
                    f"(line {self.getCurrentLineNumber()}) invalid parse sequence"
                )
            elif self.__reusableToken.type == Constants.COMMENT:
                if sb is None:
                    sb = io.StringIO()
                else:
                    sb.write(Constants.LF)
                sb.write(self.__reusableToken.content.getvalue())
                self.__reusableToken.type = Constants.TOKEN  # Read another token
            else:
                raise RuntimeError(
                    f"Unexpected Token type: {self.__reusableToken.type}"
                )

        if self.__recordList:
            self.__recordNumber += 1
            comment = None if sb is None else sb.getvalue()
            result = CSVRecord(
                self,
                self.__recordList.copy(),
                comment,
                self.__recordNumber,
                start_char_position,
            )

        return result

    def getHeaderMapRaw(self) -> typing.Dict[str, int]:
        return self.__headers.headerMap


class Headers:

    headerNames: typing.List[str] = None

    headerMap: typing.Dict[str, int] = None

    def __init__(
        self, headerMap: typing.Dict[str, int], headerNames: typing.List[str]
    ) -> None:
        self.headerMap = headerMap
        self.headerNames = headerNames


class CSVRecordIterator:

    __current: CSVRecord = None

    def remove(self) -> None:
        raise NotImplementedError("NotImplementedError")

    def next_(self) -> CSVRecord:
        if CSVParser().isClosed():
            raise RuntimeError("CSVParser has been closed")

        next_record = self.__current
        self.__current = None

        if next_record is None:
            next_record = self.__getNextRecord()
            if next_record is None:
                raise RuntimeError("No more CSV records available")

        return next_record

    def hasNext(self) -> bool:
        if CSVParser().isClosed():
            return False
        if self.__current is None:
            self.__current = self.__getNextRecord()
        return self.__current is not None

    def __getNextRecord(self) -> CSVRecord:
        try:
            return CSVParser().nextRecord()
        except IOError as e:
            raise UncheckedOSError(
                f"{e.__class__.__name__} reading next record: {str(e)}"
            ) from e
