from __future__ import annotations
import copy
import re
import collections
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

        pass  # LLM could not translate this method

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
            raise ValueError("url")
        if charset is None:
            raise ValueError("charset")
        if format_ is None:
            raise ValueError("format")

        # Open URL and create a text reader with specified encoding
        if isinstance(url, str):
            url_string = url
        else:
            url_string = urllib.parse.urlunparse(url)

        response = urllib.request.urlopen(url_string)
        reader = io.TextIOWrapper(response, encoding=charset)

        return CSVParser.CSVParser1(reader, format_)

    @staticmethod
    def parse2(path: Path, charset: str, format_: CSVFormat) -> CSVParser:
        if path is None:
            raise ValueError("path")
        if format_ is None:
            raise ValueError("format")

        # Open the file at the given path in binary mode and pass to parse1
        with open(path, "rb") as file_stream:
            return CSVParser.parse1(file_stream, charset, format_)

    @staticmethod
    def parse1(
        inputStream: typing.Union[io.BytesIO, io.StringIO, io.BufferedReader],
        charset: str,
        format_: CSVFormat,
    ) -> CSVParser:
        if inputStream is None:
            raise ValueError("inputStream")
        if format_ is None:
            raise ValueError("format")

        # Wrap the input stream with a text wrapper using the specified charset
        reader = io.TextIOWrapper(inputStream, encoding=charset)
        return CSVParser.parse3(reader, format_)

    def __addRecordValue(self, lastRecord: bool) -> None:
        input_ = self.__reusableToken.trim1(str(self.__reusableToken.content))
        if lastRecord and input_ == "" and self.__format.getTrailingDelimiter():
            return
        self.__recordList.append(self.__handleNull(input_))

    def stream(self) -> typing.Iterable[CSVRecord]:
        return self.iterator()

    def isClosed(self) -> bool:
        return self._CSVParser__lexer.isClosed()

    def hasTrailerComment(self) -> bool:
        return self.__trailerComment is not None

    def hasHeaderComment(self) -> bool:

        pass  # LLM could not translate this method

    def getTrailerComment(self) -> str:

        pass  # LLM could not translate this method

    def getRecords(self) -> typing.List[CSVRecord]:
        return list(self.stream())

    def getRecordNumber(self) -> int:
        return self.__recordNumber

    def getHeaderNames(self) -> typing.List[str]:

        pass  # LLM could not translate this method

    def getHeaderMap(self) -> typing.Dict[str, int]:
        if self.__headers.headerMap is None:
            return None
        map_ = self.__createEmptyHeaderMap()
        map_.update(self.__headers.headerMap)
        return map_

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

        pass  # LLM could not translate this method

    def __init__(
        self,
        reader: typing.Union[io.TextIOWrapper, io.BufferedReader, io.TextIOBase],
        format_: CSVFormat,
        characterOffset: int,
        recordNumber: int,
    ) -> None:
        if reader is None:
            raise ValueError("reader")
        if format_ is None:
            raise ValueError("format_")

        self.__format = format_.copy()
        self.__lexer = Lexer(format_, ExtendedBufferedReader(reader))
        self.__csvRecordIterator = CSVRecordIterator()
        self.__headers = self.__createHeaders()
        self.__characterOffset = characterOffset
        self.__recordNumber = recordNumber - 1

    @staticmethod
    def parse4(string: str, format_: CSVFormat) -> CSVParser:
        if string is None:
            raise TypeError("string")
        if format_ is None:
            raise TypeError("format")

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
            raise ValueError("file")
        if not isinstance(file, pathlib.Path):
            file = pathlib.Path(file)
        return CSVParser.parse2(file, charset, format_)

    def __isStrictQuoteMode(self) -> bool:
        return (
            self.__format.getQuoteMode() == QuoteMode.ALL_NON_NULL
            or self.__format.getQuoteMode() == QuoteMode.NON_NUMERIC
        )

    def __handleNull(self, input_: str) -> str:
        isQuoted = self.__reusableToken.isQuoted
        nullString = self.__format.getNullString()
        strictQuoteMode = self.__isStrictQuoteMode()
        if nullString is not None and input_ == nullString:
            return input_ if strictQuoteMode and isQuoted else None
        return None if strictQuoteMode and nullString is None and input_ == "" and not isQuoted else input_

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
                for i in range(len(header_record)):
                    header = header_record[i]
                    blank_header = CSVFormat.isBlank(header)

                    if blank_header and not self.__format.getAllowMissingColumnNames():
                        raise ValueError(f"A header name is missing in {header_record}")

                    contains_header = observed_missing if blank_header else header in hdr_map
                    header_mode = self.__format.getDuplicateHeaderMode()
                    duplicates_allowed = header_mode == DuplicateHeaderMode.ALLOW_ALL
                    empty_duplicates_allowed = header_mode == DuplicateHeaderMode.ALLOW_EMPTY

                    if contains_header and not duplicates_allowed and not (blank_header and empty_duplicates_allowed):
                        raise ValueError(
                            f'The header contains a duplicate name: "{header}" in '
                            f"{header_record}. If this is valid then use "
                            f"CSVFormat.Builder.setDuplicateHeaderMode()."
                        )

                    observed_missing |= blank_header
                    if header is not None:
                        hdr_map[header] = i
                        if header_names is None:
                            header_names = []
                        header_names.append(header)

        if header_names is None:
            header_names = []

        return Headers(hdr_map, header_names)

    def __createEmptyHeaderMap(self) -> typing.Dict[str, int]:
        if self.__format.getIgnoreHeaderCase():
            # Case-insensitive dictionary
            from collections.abc import MutableMapping

            class CaseInsensitiveDict(MutableMapping):
                def __init__(self):
                    self._data = {}

                def __getitem__(self, key):
                    return self._data[key.lower()]

                def __setitem__(self, key, value):
                    self._data[key.lower()] = value

                def __delitem__(self, key):
                    del self._data[key.lower()]

                def __iter__(self):
                    return iter(self._data)

                def __len__(self):
                    return len(self._data)

            return CaseInsensitiveDict()
        else:
            # Regular dict maintains insertion order in Python 3.7+
            return {}

    def nextRecord(self) -> CSVRecord:
        result = None
        self.__recordList.clear()
        sb = None
        startCharPosition = self.__lexer.getCharacterPosition() + self.__characterOffset

        while True:
            self.__reusableToken.reset()
            self.__lexer.nextToken(self.__reusableToken)

            if self.__reusableToken.type == TOKEN:
                self.__addRecordValue(False)
            elif self.__reusableToken.type == EORECORD:
                self.__addRecordValue(True)
            elif self.__reusableToken.type == EOF:
                if self.__reusableToken.isReady:
                    self.__addRecordValue(True)
                elif sb is not None:
                    self.__trailerComment = sb
            elif self.__reusableToken.type == INVALID:
                raise IOError(f"(line {self.getCurrentLineNumber()}) invalid parse sequence")
            elif self.__reusableToken.type == COMMENT:
                if sb is None:
                    sb = self.__reusableToken.content
                else:
                    sb = sb + Constants.LF + self.__reusableToken.content
                self.__reusableToken.type = TOKEN
            else:
                raise ValueError(f"Unexpected Token type: {self.__reusableToken.type}")

            if self.__reusableToken.type != TOKEN:
                break

        if len(self.__recordList) > 0:
            self.__recordNumber += 1
            comment = None if sb is None else sb
            result = CSVRecord(
                self,
                self.__recordList[:],
                comment,
                self.__recordNumber,
                startCharPosition,
            )

        return result

    def getHeaderMapRaw(self) -> typing.Dict[str, int]:
        return self.__headers.headerMap


class CSVRecordIterator:

    __current: CSVRecord = None

    def remove(self) -> None:
        raise NotImplementedError("remove operation is not supported")

    def next_(self) -> CSVRecord:
        if self._CSVParser__outer.isClosed():
            raise StopIteration("CSVParser has been closed")
        next_record = self.__current
        self.__current = None

        if next_record is None:
            next_record = self.__getNextRecord()
            if next_record is None:
                raise StopIteration("No more CSV records available")

        return next_record

    def hasNext(self) -> bool:
        if CSVParser.this.isClosed():
            return False
        if self._CSVRecordIterator__current is None:
            self._CSVRecordIterator__current = self._CSVRecordIterator__getNextRecord()

        return self._CSVRecordIterator__current is not None

    def __getNextRecord(self) -> CSVRecord:
        try:
            return CSVParser.this.nextRecord()
        except IOError as e:
            raise IOError(f"{type(e).__name__} reading next record: {str(e)}") from e


class Headers:

    headerNames: typing.List[str] = None

    headerMap: typing.Dict[str, int] = None

    def __init__(self, headerMap: typing.Dict[str, int], headerNames: typing.List[str]) -> None:
        self.headerMap = headerMap
        self.headerNames = headerNames
