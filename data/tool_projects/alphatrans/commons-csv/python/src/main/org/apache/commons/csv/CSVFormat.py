from __future__ import annotations
import time
import copy
import re
import sys
import enum
from io import StringIO
import pathlib
from io import IOBase
import io
import typing
from typing import *
import os
from src.main.org.apache.commons.csv.CSVParser import *
from src.main.org.apache.commons.csv.CSVPrinter import *
from src.main.org.apache.commons.csv.Constants import *
from src.main.org.apache.commons.csv.DuplicateHeaderMode import *
from src.main.org.apache.commons.csv.ExtendedBufferedReader import *
from src.main.org.apache.commons.csv.IOUtils import *
from src.main.org.apache.commons.csv.QuoteMode import *


class CSVFormat:

    DEFAULT: CSVFormat = None
    __trim: bool = False

    __trailingDelimiter: bool = False

    __skipHeaderRecord: bool = False

    __recordSeparator: str = ""

    __quoteMode: QuoteMode = None

    __quotedNullString: str = ""

    __quoteCharacter: str = ""

    __nullString: str = ""

    __ignoreSurroundingSpaces: bool = False

    __ignoreHeaderCase: bool = False

    __ignoreEmptyLines: bool = False

    __headerComments: typing.List[typing.List[str]] = None

    __headers: typing.List[typing.List[str]] = None

    __escapeCharacter: str = ""

    __delimiter: str = ""

    __commentMarker: str = ""

    __autoFlush: bool = False

    __allowMissingColumnNames: bool = False

    __duplicateHeaderMode: DuplicateHeaderMode = None

    __serialVersionUID: int = 2

    TDF: CSVFormat = None  # LLM could not translate this field

    RFC4180: CSVFormat = None  # LLM could not translate this field

    POSTGRESQL_TEXT: CSVFormat = None  # LLM could not translate this field

    POSTGRESQL_CSV: CSVFormat = None  # LLM could not translate this field

    ORACLE: CSVFormat = None  # LLM could not translate this field

    MYSQL: CSVFormat = None  # LLM could not translate this field

    MONGODB_TSV: CSVFormat = None  # LLM could not translate this field

    MONGODB_CSV: CSVFormat = None  # LLM could not translate this field

    INFORMIX_UNLOAD_CSV: CSVFormat = None  # LLM could not translate this field

    INFORMIX_UNLOAD: CSVFormat = None  # LLM could not translate this field

    EXCEL: CSVFormat = None  # LLM could not translate this field

    @staticmethod
    def initialize_fields() -> None:
        CSVFormat.DEFAULT: CSVFormat = CSVFormat(
            0,
            False,
            False,
            Constants.COMMA,
            None,
            None,
            False,
            False,
            None,
            None,
            False,
            Constants.DOUBLE_QUOTE_CHAR,
            None,
            True,
            DuplicateHeaderMode.ALLOW_ALL,
            None,
            False,
            False,
            None,
            Constants.CRLF,
        )

    def withTrim1(self, trim: bool) -> CSVFormat:
        return self.builder().setTrim(trim).build()

    def withTrim0(self) -> CSVFormat:
        return self.builder().setTrim(True).build()

    def withTrailingDelimiter1(self, trailingDelimiter: bool) -> CSVFormat:
        return self.builder().setTrailingDelimiter(trailingDelimiter).build()

    def withTrailingDelimiter0(self) -> CSVFormat:
        return self.builder().setTrailingDelimiter(True).build()

    def withSystemRecordSeparator(self) -> CSVFormat:
        return self.builder().setRecordSeparator1(os.linesep).build()

    def withSkipHeaderRecord1(self, skipHeaderRecord: bool) -> CSVFormat:
        return self.builder().setSkipHeaderRecord(skipHeaderRecord).build()

    def withSkipHeaderRecord0(self) -> CSVFormat:
        return self.builder().setSkipHeaderRecord(True).build()

    def withRecordSeparator1(self, recordSeparator: str) -> CSVFormat:
        return self.builder().setRecordSeparator1(recordSeparator).build()

    def withRecordSeparator0(self, recordSeparator: str) -> CSVFormat:
        return self.builder().setRecordSeparator0(recordSeparator).build()

    def withQuoteMode(self, quoteMode: QuoteMode) -> CSVFormat:
        return self.builder().setQuoteMode(quoteMode).build()

    def withQuote1(self, quoteChar: str) -> CSVFormat:
        return self.builder().setQuote1(quoteChar).build()

    def withQuote0(self, quoteChar: str) -> CSVFormat:
        return self.builder().setQuote0(quoteChar).build()

    def withNullString(self, nullString: str) -> CSVFormat:
        return self.builder().setNullString(nullString).build()

    def withIgnoreSurroundingSpaces1(self, ignoreSurroundingSpaces: bool) -> CSVFormat:
        return (
            self.builder().setIgnoreSurroundingSpaces(ignoreSurroundingSpaces).build()
        )

    def withIgnoreSurroundingSpaces0(self) -> CSVFormat:
        return self.builder().setIgnoreSurroundingSpaces(True).build()

    def withIgnoreHeaderCase1(self, ignoreHeaderCase: bool) -> CSVFormat:
        return self.builder().setIgnoreHeaderCase(ignoreHeaderCase).build()

    def withIgnoreHeaderCase0(self) -> CSVFormat:
        return self.builder().setIgnoreHeaderCase(True).build()

    def withIgnoreEmptyLines1(self, ignoreEmptyLines: bool) -> CSVFormat:
        return self.builder().setIgnoreEmptyLines(ignoreEmptyLines).build()

    def withIgnoreEmptyLines0(self) -> CSVFormat:
        return self.builder().setIgnoreEmptyLines(True).build()

    def withHeaderComments(self, headerComments: typing.List[typing.Any]) -> CSVFormat:
        return self.builder().setHeaderComments0(headerComments).build()

    def withEscape1(self, escape: str) -> CSVFormat:
        return self.builder().setEscape1(escape).build()

    def withEscape0(self, escape: str) -> CSVFormat:
        return self.builder().setEscape0(escape).build()

    def withDelimiter(self, delimiter: str) -> CSVFormat:
        return self.builder().setDelimiter0(delimiter).build()

    def withCommentMarker1(self, commentMarker: str) -> CSVFormat:
        return self.builder().setCommentMarker1(commentMarker).build()

    def withCommentMarker0(self, commentMarker: str) -> CSVFormat:
        return self.builder().setCommentMarker0(commentMarker).build()

    def withAutoFlush(self, autoFlush: bool) -> CSVFormat:
        return self.builder().setAutoFlush(autoFlush).build()

    def withAllowMissingColumnNames1(self, allowMissingColumnNames: bool) -> CSVFormat:
        return (
            self.builder().setAllowMissingColumnNames(allowMissingColumnNames).build()
        )

    def withAllowMissingColumnNames0(self) -> CSVFormat:
        return self.builder().setAllowMissingColumnNames(True).build()

    def withAllowDuplicateHeaderNames1(
        self, allowDuplicateHeaderNames: bool
    ) -> CSVFormat:
        mode = (
            DuplicateHeaderMode.ALLOW_ALL
            if allowDuplicateHeaderNames
            else DuplicateHeaderMode.ALLOW_EMPTY
        )
        return self.builder().setDuplicateHeaderMode(mode).build()

    def withAllowDuplicateHeaderNames0(self) -> CSVFormat:
        return (
            self.builder().setDuplicateHeaderMode(DuplicateHeaderMode.ALLOW_ALL).build()
        )

    def toString(self) -> str:
        sb = []
        sb.append(f"Delimiter=<{self.__delimiter}>")
        if self.isEscapeCharacterSet():
            sb.append(f" Escape=<{self.__escapeCharacter}>")
        if self.isQuoteCharacterSet():
            sb.append(f" QuoteChar=<{self.__quoteCharacter}>")
        if self.__quoteMode is not None:
            sb.append(f" QuoteMode=<{self.__quoteMode}>")
        if self.isCommentMarkerSet():
            sb.append(f" CommentStart=<{self.__commentMarker}>")
        if self.isNullStringSet():
            sb.append(f" NullString=<{self.__nullString}>")
        if self.__recordSeparator is not None:
            sb.append(f" RecordSeparator=<{self.__recordSeparator}>")
        if hasattr(self, "_CSVFormat__ignoreEmptyLines") and self.getIgnoreEmptyLines():
            sb.append(" EmptyLines:ignored")
        if (
            hasattr(self, "_CSVFormat__ignoreSurroundingSpaces")
            and self.getIgnoreSurroundingSpaces()
        ):
            sb.append(" SurroundingSpaces:ignored")
        if hasattr(self, "_CSVFormat__ignoreHeaderCase") and self.getIgnoreHeaderCase():
            sb.append(" IgnoreHeaderCase:ignored")
        sb.append(f" SkipHeaderRecord:{self.__skipHeaderRecord}")
        if self.__headerComments is not None:
            sb.append(f" HeaderComments:{self.__headerComments}")
        if self.__headers is not None:
            sb.append(f" Header:{self.__headers}")
        return " ".join(sb)

    def print4(self, out: Path, charset: str) -> CSVPrinter:
        return self.print0(out.open(mode="w", encoding=charset))

    def print1(self, out: pathlib.Path, charset: str) -> CSVPrinter:
        with out.open("w", encoding=charset) as file:
            return CSVPrinter(file, self)

    def hashCode(self) -> int:
        prime = 31
        result = 1
        result = prime * result + hash(tuple(self.__headers)) if self.__headers else 0
        result = (
            prime * result + hash(tuple(self.__headerComments))
            if self.__headerComments
            else 0
        )
        result = prime * result + hash(
            (
                self.__duplicateHeaderMode,
                self.__allowMissingColumnNames,
                self.__autoFlush,
                self.__commentMarker,
                self.__delimiter,
                self.__escapeCharacter,
                self.__ignoreEmptyLines,
                self.__ignoreHeaderCase,
                self.__ignoreSurroundingSpaces,
                self.__nullString,
                self.__quoteCharacter,
                self.__quoteMode,
                self.__quotedNullString,
                self.__recordSeparator,
                self.__skipHeaderRecord,
                self.__trailingDelimiter,
                self.__trim,
            )
        )
        return result

    def getDelimiter(self) -> str:
        return self.__delimiter[0]

    def getAllowDuplicateHeaderNames(self) -> bool:
        return self.__duplicateHeaderMode == DuplicateHeaderMode.ALLOW_ALL

    def equals(self, obj: typing.Any) -> bool:
        if self is obj:
            return True
        if obj is None or not isinstance(obj, CSVFormat):
            return False
        return (
            self.__duplicateHeaderMode == obj.__duplicateHeaderMode
            and self.__allowMissingColumnNames == obj.__allowMissingColumnNames
            and self.__autoFlush == obj.__autoFlush
            and self.__commentMarker == obj.__commentMarker
            and self.__delimiter == obj.__delimiter
            and self.__escapeCharacter == obj.__escapeCharacter
            and (
                self.__headers == obj.__headers
                if self.__headers is not None and obj.__headers is not None
                else self.__headers is obj.__headers
            )
            and (
                self.__headerComments == obj.__headerComments
                if self.__headerComments is not None
                and obj.__headerComments is not None
                else self.__headerComments is obj.__headerComments
            )
            and self.__ignoreEmptyLines == obj.__ignoreEmptyLines
            and self.__ignoreHeaderCase == obj.__ignoreHeaderCase
            and self.__ignoreSurroundingSpaces == obj.__ignoreSurroundingSpaces
            and self.__nullString == obj.__nullString
            and self.__quoteCharacter == obj.__quoteCharacter
            and self.__quoteMode == obj.__quoteMode
            and self.__quotedNullString == obj.__quotedNullString
            and self.__recordSeparator == obj.__recordSeparator
            and self.__skipHeaderRecord == obj.__skipHeaderRecord
            and self.__trailingDelimiter == obj.__trailingDelimiter
            and self.__trim == obj.__trim
        )

    @staticmethod
    def clone(values: typing.List[typing.Any]) -> typing.List[typing.Any]:
        return None if values is None else values[:]

    def printRecord(
        self,
        appendable: typing.Union[typing.List, io.TextIOBase],
        values: typing.List[typing.Any],
    ) -> None:
        for i, value in enumerate(values):
            self.print2(value, appendable, i == 0)
        self.println(appendable)

    def println(self, appendable: typing.Union[typing.List, io.TextIOBase]) -> None:
        if self.getTrailingDelimiter():
            self.__append1(self.getDelimiterString(), appendable)
        if self.__recordSeparator is not None:
            self.__append1(self.__recordSeparator, appendable)

    def printer(self) -> CSVPrinter:
        return CSVPrinter(sys.stdout, self)

    def print2(
        self,
        value: typing.Any,
        out: typing.Union[typing.List, io.TextIOBase],
        newRecord: bool,
    ) -> None:
        if value is None:
            if self.__nullString is None:
                charSequence = Constants.EMPTY
            elif self.__quoteMode == QuoteMode.ALL:
                charSequence = self.__quotedNullString
            else:
                charSequence = self.__nullString
        elif isinstance(value, str):
            charSequence = value
        elif isinstance(value, io.TextIOBase):
            self.__print5(value, out, newRecord)
            return
        else:
            charSequence = str(value)

        charSequence = self.trim0(charSequence) if self.getTrim() else charSequence
        self.__print3(value, charSequence, out, newRecord)

    def print0(self, out: typing.Union[typing.List, io.TextIOBase]) -> CSVPrinter:
        return CSVPrinter(out, self)

    def parse(
        self, reader: typing.Union[io.TextIOWrapper, io.BufferedReader, io.TextIOBase]
    ) -> CSVParser:
        return CSVParser.CSVParser1(reader, self)

    def isQuoteCharacterSet(self) -> bool:
        return self.__quoteCharacter is not None

    def isNullStringSet(self) -> bool:
        return self.__nullString is not None

    def isEscapeCharacterSet(self) -> bool:
        return self.__escapeCharacter is not None

    def isCommentMarkerSet(self) -> bool:
        return self.__commentMarker is not None

    def getTrim(self) -> bool:
        return self.__trim

    def getTrailingDelimiter(self) -> bool:
        return self.__trailingDelimiter

    def getSkipHeaderRecord(self) -> bool:
        return self.__skipHeaderRecord

    def getRecordSeparator(self) -> str:
        return self.__recordSeparator

    def getQuoteMode(self) -> QuoteMode:
        return self.__quoteMode

    def getQuoteCharacter(self) -> Optional[str]:
        return self.__quoteCharacter if self.__quoteCharacter else None

    def getNullString(self) -> str:
        return self.__nullString

    def getIgnoreSurroundingSpaces(self) -> bool:
        return self.__ignoreSurroundingSpaces

    def getIgnoreHeaderCase(self) -> bool:
        return self.__ignoreHeaderCase

    def getIgnoreEmptyLines(self) -> bool:
        return self.__ignoreEmptyLines

    def getHeaderComments(self) -> typing.Optional[typing.List[str]]:
        return (
            self.__headerComments.copy() if self.__headerComments is not None else None
        )

    def getHeader(self) -> typing.Optional[typing.List[str]]:
        return self.__headers.copy() if self.__headers is not None else None

    def getEscapeCharacter(self) -> Optional[str]:
        return self.__escapeCharacter if self.__escapeCharacter else None

    def getDuplicateHeaderMode(self) -> DuplicateHeaderMode:
        return self.__duplicateHeaderMode

    def getDelimiterString(self) -> str:
        return self.__delimiter

    def getCommentMarker(self) -> Optional[str]:
        return self.__commentMarker if self.__commentMarker else None

    def getAutoFlush(self) -> bool:
        return self.__autoFlush

    def getAllowMissingColumnNames(self) -> bool:
        return self.__allowMissingColumnNames

    def format_(self, values: typing.List[typing.Any]) -> str:
        out = io.StringIO()
        try:
            csv_printer = CSVPrinter(out, self)
            csv_printer.printRecord1(values)
            res = out.getvalue()
            len_ = (
                len(res) - len(self.__recordSeparator)
                if self.__recordSeparator
                else len(res)
            )
            return res[:len_]
        except IOError as e:
            raise RuntimeError(e)

    def builder(self) -> Builder:
        return Builder.create1(self)

    def __init__(
        self,
        constructorId: int,
        autoFlush: bool,
        skipHeaderRecord: bool,
        delimiter: str,
        nullString: str,
        escape: str,
        ignoreSurroundingSpaces: bool,
        trim: bool,
        builder: Builder,
        commentStart: str,
        ignoreHeaderCase: bool,
        quoteChar: str,
        quoteMode: QuoteMode,
        ignoreEmptyLines: bool,
        duplicateHeaderMode: DuplicateHeaderMode,
        header: typing.List[typing.List[str]],
        allowMissingColumnNames: bool,
        trailingDelimiter: bool,
        headerComments: typing.List[typing.Any],
        recordSeparator: str,
    ) -> None:

        pass  # LLM could not translate this method

    @staticmethod
    def valueOf(format_: str) -> CSVFormat:
        return Predefined.__dict__[format_].getFormat()

    @staticmethod
    def trim0(charSequence: str) -> str:
        if isinstance(charSequence, str):
            return charSequence.strip()

        count = len(charSequence)
        length = count
        pos = 0

        while pos < length and CSVFormat.__isTrimChar1(charSequence, pos):
            pos += 1
        while pos < length and CSVFormat.__isTrimChar1(charSequence, length - 1):
            length -= 1

        return charSequence[pos:length] if pos > 0 or length < count else charSequence

    @staticmethod
    def toStringArray(
        values: typing.Optional[typing.List[typing.Any]],
    ) -> typing.Optional[typing.List[str]]:
        if values is None:
            return None
        return [str(value) if value is not None else None for value in values]

    @staticmethod
    def newFormat(delimiter: str) -> CSVFormat:
        return CSVFormat(
            constructorId=0,
            autoFlush=False,
            skipHeaderRecord=False,
            delimiter=delimiter,
            nullString=None,
            escape=None,
            ignoreSurroundingSpaces=False,
            trim=False,
            builder=None,
            commentStart=None,
            ignoreHeaderCase=False,
            quoteChar=None,
            quoteMode=None,
            ignoreEmptyLines=False,
            duplicateHeaderMode=DuplicateHeaderMode.ALLOW_ALL,
            header=None,
            allowMissingColumnNames=False,
            trailingDelimiter=False,
            headerComments=None,
            recordSeparator=None,
        )

    @staticmethod
    def isBlank(value: str) -> bool:
        return value is None or value.strip() == ""

    def __validate(self) -> None:
        if self.__containsLineBreak(self.__delimiter):
            raise ValueError("The delimiter cannot be a line break")

        if self.__quoteCharacter and self.__contains(
            self.__delimiter, self.__quoteCharacter
        ):
            raise ValueError(
                f"The quoteChar character and the delimiter cannot be the same ('{self.__quoteCharacter}')"
            )

        if self.__escapeCharacter and self.__contains(
            self.__delimiter, self.__escapeCharacter
        ):
            raise ValueError(
                f"The escape character and the delimiter cannot be the same ('{self.__escapeCharacter}')"
            )

        if self.__commentMarker and self.__contains(
            self.__delimiter, self.__commentMarker
        ):
            raise ValueError(
                f"The comment start character and the delimiter cannot be the same ('{self.__commentMarker}')"
            )

        if self.__quoteCharacter and self.__quoteCharacter == self.__commentMarker:
            raise ValueError(
                f"The comment start character and the quoteChar cannot be the same ('{self.__commentMarker}')"
            )

        if self.__escapeCharacter and self.__escapeCharacter == self.__commentMarker:
            raise ValueError(
                f"The comment start and the escape character cannot be the same ('{self.__commentMarker}')"
            )

        if not self.__escapeCharacter and self.__quoteMode == QuoteMode.NONE:
            raise ValueError("No quotes mode set but no escape character is set")

        if (
            self.__headers
            and self.__duplicateHeaderMode != DuplicateHeaderMode.ALLOW_ALL
        ):
            dup_check_set = set()
            empty_duplicates_allowed = (
                self.__duplicateHeaderMode == DuplicateHeaderMode.ALLOW_EMPTY
            )
            for header in self.__headers:
                blank = self.isBlank(header)
                contains_header = not dup_check_set.add("" if blank else header)
                if contains_header and not (blank and empty_duplicates_allowed):
                    raise ValueError(
                        f'The header contains a duplicate name: "{header}" in {self.__headers}. '
                        "If this is valid then use CSVFormat.Builder.setDuplicateHeaderMode()."
                    )

    def __printWithQuotes1(
        self,
        reader: typing.Union[io.TextIOWrapper, io.BufferedReader, io.TextIOBase],
        appendable: typing.Union[typing.List, io.TextIOBase],
    ) -> None:
        if self.getQuoteMode() == QuoteMode.NONE:
            self.__printWithEscapes1(reader, appendable)
            return

        pos = 0
        quote = self.getQuoteCharacter()
        builder = []

        self.__append0(quote, appendable)

        while True:
            c = reader.read(1)
            if not c:  # End of stream
                break

            builder.append(c)
            if c == quote:
                if pos > 0:
                    self.__append1("".join(builder[:pos]), appendable)
                    self.__append0(quote, appendable)
                    builder = []
                    pos = -1

                self.__append0(c, appendable)

            pos += 1

        if pos > 0:
            self.__append1("".join(builder[:pos]), appendable)

        self.__append0(quote, appendable)

    def __printWithQuotes0(
        self,
        object_: typing.Any,
        charSeq: str,
        out: typing.Union[typing.List, io.TextIOBase],
        newRecord: bool,
    ) -> None:
        quote = False
        start = 0
        pos = 0
        length = len(charSeq)

        delim = list(self.getDelimiterString())
        delimLength = len(delim)
        quoteChar = self.getQuoteCharacter()
        escapeChar = (
            self.getEscapeCharacter() if self.isEscapeCharacterSet() else quoteChar
        )

        quoteModePolicy = self.getQuoteMode()
        if quoteModePolicy is None:
            quoteModePolicy = QuoteMode.MINIMAL

        if (
            quoteModePolicy == QuoteMode.ALL
            or quoteModePolicy == QuoteMode.ALL_NON_NULL
        ):
            quote = True
        elif quoteModePolicy == QuoteMode.NON_NUMERIC:
            quote = not isinstance(object_, (int, float))
        elif quoteModePolicy == QuoteMode.NONE:
            self.__printWithEscapes0(charSeq, out)
            return
        elif quoteModePolicy == QuoteMode.MINIMAL:
            if length <= 0:
                if newRecord:
                    quote = True
            else:
                c = charSeq[pos]
                if ord(c) <= ord(Constants.COMMENT):
                    quote = True
                else:
                    while pos < length:
                        c = charSeq[pos]
                        if (
                            c == Constants.LF
                            or c == Constants.CR
                            or c == quoteChar
                            or c == escapeChar
                            or self.__isDelimiter(c, charSeq, pos, delim, delimLength)
                        ):
                            quote = True
                            break
                        pos += 1

                    if not quote:
                        pos = length - 1
                        c = charSeq[pos]
                        if self.__isTrimChar0(c):
                            quote = True

            if not quote:
                out.write(charSeq[start:length])
                return
        else:
            raise ValueError(f"Unexpected Quote value: {quoteModePolicy}")

        if not quote:
            out.write(charSeq[start:length])
            return

        out.write(quoteChar)

        while pos < length:
            c = charSeq[pos]
            if c == quoteChar or c == escapeChar:
                out.write(charSeq[start:pos])
                out.write(escapeChar)  # now output the escape
                start = pos  # and restart with the matched char
            pos += 1

        out.write(charSeq[start:pos])
        out.write(quoteChar)

    def __printWithEscapes1(
        self,
        reader: typing.Union[io.TextIOWrapper, io.BufferedReader, io.TextIOBase],
        appendable: typing.Union[typing.List, io.TextIOBase],
    ) -> None:
        start = 0
        pos = 0

        # Temporary reader on input reader
        bufferedReader = ExtendedBufferedReader(reader)
        delim = list(self.getDelimiterString())
        delimLength = len(delim)
        escape = self.getEscapeCharacter()
        builder = []

        while True:
            c = bufferedReader.read0()
            if c == Constants.END_OF_STREAM:
                break

            builder.append(chr(c))
            isDelimiterStart = self.__isDelimiter(
                chr(c),
                "".join(builder) + "".join(bufferedReader.lookAhead2(delimLength - 1)),
                pos,
                delim,
                delimLength,
            )

            if (
                c == ord(Constants.CR)
                or c == ord(Constants.LF)
                or c == ord(escape)
                or isDelimiterStart
            ):
                if pos > start:
                    self.__append1("".join(builder[start:pos]), appendable)
                    builder = []
                    pos = -1

                if c == ord(Constants.LF):
                    c = ord("n")
                elif c == ord(Constants.CR):
                    c = ord("r")

                self.__append0(escape, appendable)
                self.__append0(chr(c), appendable)

                if isDelimiterStart:
                    for i in range(1, delimLength):
                        c = bufferedReader.read0()
                        self.__append0(escape, appendable)
                        self.__append0(chr(c), appendable)

                start = pos + 1  # Start on the current char after this one

            pos += 1

        if pos > start:
            self.__append1("".join(builder[start:pos]), appendable)

    def __printWithEscapes0(
        self, charSeq: str, appendable: typing.Union[typing.List, io.TextIOBase]
    ) -> None:
        start = 0
        pos = 0
        end = len(charSeq)

        delim = list(self.getDelimiterString())
        delimLength = len(delim)
        escape = self.getEscapeCharacter()

        while pos < end:
            c = charSeq[pos]
            isDelimiterStart = self.__isDelimiter(c, charSeq, pos, delim, delimLength)
            if (
                c == Constants.CR
                or c == Constants.LF
                or c == escape
                or isDelimiterStart
            ):
                if pos > start:
                    appendable.write(charSeq[start:pos])
                if c == Constants.LF:
                    c = "n"
                elif c == Constants.CR:
                    c = "r"

                appendable.write(escape)
                appendable.write(c)

                if isDelimiterStart:
                    for i in range(1, delimLength):
                        pos += 1
                        c = charSeq[pos]
                        appendable.write(escape)
                        appendable.write(c)

                start = pos + 1  # start on the current char after this one
            pos += 1

        if pos > start:
            appendable.write(charSeq[start:pos])

    def __print5(
        self,
        reader: typing.Union[io.TextIOWrapper, io.BufferedReader, io.TextIOBase],
        out: typing.Union[typing.List, io.TextIOBase],
        newRecord: bool,
    ) -> None:
        if not newRecord:
            self.__append1(self.getDelimiterString(), out)
        if self.isQuoteCharacterSet():
            self.__printWithQuotes1(reader, out)
        elif self.isEscapeCharacterSet():
            self.__printWithEscapes1(reader, out)
        elif isinstance(out, io.TextIOBase):
            IOUtils.copyLarge0(reader, out)
        else:
            IOUtils.copy0(reader, out)

    def __print3(
        self,
        object_: typing.Any,
        value: str,
        out: typing.Union[typing.List, io.TextIOBase],
        newRecord: bool,
    ) -> None:
        offset = 0
        length = len(value)
        if not newRecord:
            out.write(self.getDelimiterString())
        if object_ is None:
            out.write(value)
        elif self.isQuoteCharacterSet():
            self.__printWithQuotes0(object_, value, out, newRecord)
        elif self.isEscapeCharacterSet():
            self.__printWithEscapes0(value, out)
        else:
            out.write(value[offset:length])

    def __isDelimiter(
        self,
        ch: str,
        charSeq: str,
        startIndex: int,
        delimiter: typing.List[str],
        delimiterLength: int,
    ) -> bool:
        if ch != delimiter[0]:
            return False
        length = len(charSeq)
        if startIndex + delimiterLength > length:
            return False
        for i in range(1, delimiterLength):
            if charSeq[startIndex + i] != delimiter[i]:
                return False
        return True

    def __append1(
        self, csq: str, appendable: typing.Union[typing.List, io.TextIOBase]
    ) -> None:
        appendable.append(csq)

    def __append0(
        self, c: str, appendable: typing.Union[typing.List, io.TextIOBase]
    ) -> None:
        appendable.append(c)

    @staticmethod
    def __isTrimChar1(charSequence: str, pos: int) -> bool:
        return CSVFormat.__isTrimChar0(charSequence[pos])

    @staticmethod
    def __isTrimChar0(ch: str) -> bool:
        return ord(ch) <= ord(Constants.SP)

    @staticmethod
    def __isLineBreak1(c: Optional[str]) -> bool:
        return c is not None and CSVFormat.__isLineBreak0(c)

    @staticmethod
    def __isLineBreak0(c: str) -> bool:
        return c == Constants.LF or c == Constants.CR

    @staticmethod
    def __containsLineBreak(source: str) -> bool:
        return CSVFormat.__contains(source, Constants.CR) or CSVFormat.__contains(
            source, Constants.LF
        )

    @staticmethod
    def __contains(source: str, searchCh: str) -> bool:
        if source is None:
            raise ValueError("source cannot be None")
        return searchCh in source

    def trim1(self, value: str) -> str:
        return value.strip() if self.getTrim() else value

    def copy(self) -> CSVFormat:
        return self.builder().build()


class Builder:

    __trim: bool = False

    __trailingDelimiter: bool = False

    __skipHeaderRecord: bool = False

    __recordSeparator: str = ""

    __quoteMode: QuoteMode = None

    __quotedNullString: str = ""

    __quoteCharacter: str = ""

    __nullString: str = ""

    __ignoreSurroundingSpaces: bool = False

    __ignoreHeaderCase: bool = False

    __ignoreEmptyLines: bool = False

    __headers: typing.List[typing.List[str]] = None

    __headerComments: typing.List[typing.List[str]] = None

    __escapeCharacter: str = ""

    __duplicateHeaderMode: DuplicateHeaderMode = None

    __delimiter: str = ""

    __commentMarker: str = ""

    __autoFlush: bool = False

    __allowMissingColumnNames: bool = False

    def setAllowDuplicateHeaderNames(self, allowDuplicateHeaderNames: bool) -> Builder:
        self.setDuplicateHeaderMode(
            DuplicateHeaderMode.ALLOW_ALL
            if allowDuplicateHeaderNames
            else DuplicateHeaderMode.ALLOW_EMPTY
        )
        return self

    def setTrim(self, trim: bool) -> Builder:
        self.__trim = trim
        return self

    def setTrailingDelimiter(self, trailingDelimiter: bool) -> Builder:
        self.__trailingDelimiter = trailingDelimiter
        return self

    def setSkipHeaderRecord(self, skipHeaderRecord: bool) -> Builder:
        self.__skipHeaderRecord = skipHeaderRecord
        return self

    def setRecordSeparator1(self, recordSeparator: str) -> Builder:
        self.__recordSeparator = recordSeparator
        return self

    def setRecordSeparator0(self, recordSeparator: str) -> Builder:
        self.__recordSeparator = str(recordSeparator)
        return self

    def setQuoteMode(self, quoteMode: QuoteMode) -> Builder:
        self.__quoteMode = quoteMode
        return self

    def setQuote1(self, quoteCharacter: Optional[str]) -> Builder:
        if CSVFormat._CSVFormat__isLineBreak1(quoteCharacter):
            raise ValueError("The quoteChar cannot be a line break")
        self.__quoteCharacter = quoteCharacter
        return self

    def setQuote0(self, quoteCharacter: str) -> Builder:
        self.setQuote1(quoteCharacter)
        return self

    def setNullString(self, nullString: str) -> Builder:
        self.__nullString = nullString
        self.__quotedNullString = (
            f"{self.__quoteCharacter}{nullString}{self.__quoteCharacter}"
        )
        return self

    def setIgnoreSurroundingSpaces(self, ignoreSurroundingSpaces: bool) -> Builder:
        self.__ignoreSurroundingSpaces = ignoreSurroundingSpaces
        return self

    def setIgnoreHeaderCase(self, ignoreHeaderCase: bool) -> Builder:
        self.__ignoreHeaderCase = ignoreHeaderCase
        return self

    def setIgnoreEmptyLines(self, ignoreEmptyLines: bool) -> Builder:
        self.__ignoreEmptyLines = ignoreEmptyLines
        return self

    def setHeaderComments1(self, headerComments: typing.List[str]) -> Builder:
        self.__headerComments = CSVFormat.clone(headerComments)
        return self

    def setHeaderComments0(self, headerComments: typing.List[typing.Any]) -> Builder:
        self.__headerComments = CSVFormat.clone(CSVFormat.toStringArray(headerComments))
        return self

    def setEscape1(self, escapeCharacter: str) -> Builder:
        if CSVFormat._CSVFormat__isLineBreak1(escapeCharacter):
            raise ValueError("The escape character cannot be a line break")
        self.__escapeCharacter = escapeCharacter
        return self

    def setEscape0(self, escapeCharacter: str) -> Builder:
        self.setEscape1(escapeCharacter)
        return self

    def setDuplicateHeaderMode(
        self, duplicateHeaderMode: DuplicateHeaderMode
    ) -> Builder:
        self.__duplicateHeaderMode = duplicateHeaderMode or ValueError(
            "duplicateHeaderMode cannot be None"
        )
        return self

    def setDelimiter1(self, delimiter: str) -> Builder:
        if "\n" in delimiter or "\r" in delimiter:  # Check for line breaks
            raise ValueError("The delimiter cannot be a line break")
        if not delimiter:  # Check if the delimiter is empty
            raise ValueError("The delimiter cannot be empty")
        self.__delimiter = delimiter  # Set the delimiter
        return self  # Return the current Builder instance

    def setDelimiter0(self, delimiter: str) -> Builder:
        return self.setDelimiter1(str(delimiter))

    def setCommentMarker1(self, commentMarker: str) -> Builder:
        if self.__isLineBreak1(commentMarker):
            raise ValueError(
                "The comment start marker character cannot be a line break"
            )
        self.__commentMarker = commentMarker
        return self

    def setCommentMarker0(self, commentMarker: str) -> Builder:
        self.setCommentMarker1(commentMarker)
        return self

    def setAutoFlush(self, autoFlush: bool) -> Builder:
        self.__autoFlush = autoFlush
        return self

    def setAllowMissingColumnNames(self, allowMissingColumnNames: bool) -> Builder:
        self.__allowMissingColumnNames = allowMissingColumnNames
        return self

    def build(self) -> CSVFormat:
        return CSVFormat(
            1,
            False,
            False,
            None,
            None,
            None,
            False,
            False,
            self,
            None,
            False,
            None,
            None,
            False,
            None,
            None,
            False,
            False,
            None,
            None,
        )

    @staticmethod
    def create1(csvFormat: CSVFormat) -> Builder:
        return Builder(csvFormat)

    @staticmethod
    def create0() -> Builder:
        return Builder(CSVFormat.DEFAULT)

    def __init__(self, csvFormat: CSVFormat) -> None:
        self.__delimiter = csvFormat._CSVFormat__delimiter
        self.__quoteCharacter = csvFormat._CSVFormat__quoteCharacter
        self.__quoteMode = csvFormat._CSVFormat__quoteMode
        self.__commentMarker = csvFormat._CSVFormat__commentMarker
        self.__escapeCharacter = csvFormat._CSVFormat__escapeCharacter
        self.__ignoreSurroundingSpaces = csvFormat._CSVFormat__ignoreSurroundingSpaces
        self.__allowMissingColumnNames = csvFormat._CSVFormat__allowMissingColumnNames
        self.__ignoreEmptyLines = csvFormat._CSVFormat__ignoreEmptyLines
        self.__recordSeparator = csvFormat._CSVFormat__recordSeparator
        self.__nullString = csvFormat._CSVFormat__nullString
        self.__headerComments = csvFormat._CSVFormat__headerComments
        self.__headers = csvFormat._CSVFormat__headers
        self.__skipHeaderRecord = csvFormat._CSVFormat__skipHeaderRecord
        self.__ignoreHeaderCase = csvFormat._CSVFormat__ignoreHeaderCase
        self.__trailingDelimiter = csvFormat._CSVFormat__trailingDelimiter
        self.__trim = csvFormat._CSVFormat__trim
        self.__autoFlush = csvFormat._CSVFormat__autoFlush
        self.__quotedNullString = csvFormat._CSVFormat__quotedNullString
        self.__duplicateHeaderMode = csvFormat._CSVFormat__duplicateHeaderMode


class Predefined:

    TDF: Predefined = None

    RFC4180: Predefined = None

    PostgreSQLText: Predefined = None

    PostgreSQLCsv: Predefined = None

    Oracle: Predefined = None

    MySQL: Predefined = None

    MongoDBTsv: Predefined = None

    MongoDBCsv: Predefined = None

    InformixUnloadCsv: Predefined = None

    InformixUnload: Predefined = None

    Excel: Predefined = None

    Default: Predefined = None

    __format: CSVFormat = None

    def getFormat(self) -> CSVFormat:
        return self.__format

    def __init__(self, format_: CSVFormat) -> None:
        self.__format = format_


CSVFormat.initialize_fields()
