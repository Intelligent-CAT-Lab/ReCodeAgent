from __future__ import annotations
import re
from functools import cmp_to_key
import enum
from io import IOBase
from io import StringIO
import io
import typing
from typing import *
import os
from src.main.org.apache.commons.cli.Option import *
from src.main.org.apache.commons.cli.OptionGroup import *
from src.main.org.apache.commons.cli.Options import *


class HelpFormatter:

    _optionComparator: typing.Callable[[Option, Option], int] = None
    defaultArgName: str = None  # LLM could not translate this field

    defaultOptPrefix: str = None  # LLM could not translate this field

    defaultNewLine: str = os.linesep
    defaultSyntaxPrefix: str = "usage: "
    defaultDescPad: int = 3
    DEFAULT_ARG_NAME: str = "arg"
    DEFAULT_LONG_OPT_SEPARATOR: str = " "
    DEFAULT_LONG_OPT_PREFIX: str = "--"
    DEFAULT_OPT_PREFIX: str = "-"
    DEFAULT_SYNTAX_PREFIX: str = "usage: "
    DEFAULT_DESC_PAD: int = 3
    DEFAULT_LEFT_PAD: int = 1
    DEFAULT_WIDTH: int = 74
    __longOptSeparator: str = DEFAULT_LONG_OPT_SEPARATOR
    defaultLongOptPrefix: str = DEFAULT_LONG_OPT_PREFIX
    defaultLeftPad: int = DEFAULT_LEFT_PAD
    defaultWidth: int = DEFAULT_WIDTH

    def setWidth(self, width: int) -> None:
        self.defaultWidth = width

    def setSyntaxPrefix(self, prefix: str) -> None:
        self.defaultSyntaxPrefix = prefix

    def setOptPrefix(self, prefix: str) -> None:
        self.defaultOptPrefix = prefix

    def setOptionComparator(self, comparator: typing.Callable[[Option, Option], int]) -> None:
        self._optionComparator = comparator

    def setNewLine(self, newline: str) -> None:
        self.defaultNewLine = newline

    def setLongOptSeparator(self, longOptSeparator: str) -> None:
        self._HelpFormatter__longOptSeparator = longOptSeparator

    def setLongOptPrefix(self, prefix: str) -> None:
        self.defaultLongOptPrefix = prefix

    def setLeftPadding(self, padding: int) -> None:
        self.defaultLeftPad = padding

    def setDescPadding(self, padding: int) -> None:
        self.defaultDescPad = padding

    def setArgName(self, name: str) -> None:
        self.defaultArgName = name

    def _rtrim(self, s: str) -> str:
        if s is None or len(s) == 0:
            return s

        pos = len(s)

        while pos > 0 and s[pos - 1].isspace():
            pos -= 1

        return s[0:pos]

    def _renderWrappedText(self, sb: io.StringIO, width: int, nextLineTabStop: int, text: str) -> io.StringIO:
        pos = self._findWrapPos(text, width, 0)

        if pos == -1:
            sb.write(self._rtrim(text))
            return sb

        sb.write(self._rtrim(text[0:pos]))
        sb.write(self.getNewLine())

        if nextLineTabStop >= width:
            nextLineTabStop = 1

        padding = self._createPadding(nextLineTabStop)

        while True:
            text = padding + text[pos:].strip()
            pos = self._findWrapPos(text, width, 0)

            if pos == -1:
                sb.write(text)
                return sb

            if len(text) > width and pos == nextLineTabStop - 1:
                pos = width

            sb.write(self._rtrim(text[0:pos]))
            sb.write(self.getNewLine())

    def _renderOptions(self, sb: io.StringIO, width: int, options: Options, leftPad: int, descPad: int) -> io.StringIO:

        pass  # LLM could not translate this method

    def printWrapped1(self, pw: typing.Union[io.TextIOWrapper, io.StringIO], width: int, text: str) -> None:
        self.printWrapped0(pw, width, 0, text)

    def printWrapped0(
        self,
        pw: typing.Union[io.TextIOWrapper, io.StringIO],
        width: int,
        nextLineTabStop: int,
        text: str,
    ) -> None:
        sb = io.StringIO()
        self.__renderWrappedTextBlock(sb, width, nextLineTabStop, text)
        pw.write(sb.getvalue() + "\n")
        pw.flush()

    def printUsage1(
        self,
        pw: typing.Union[io.TextIOWrapper, io.StringIO],
        width: int,
        app: str,
        options: Options,
    ) -> None:
        buff = io.StringIO()
        buff.write(self.getSyntaxPrefix())
        buff.write(app)
        buff.write(" ")

        processedGroups = []

        optList = list(options.getOptions())
        if self.getOptionComparator() is not None:
            import functools

            optList.sort(key=functools.cmp_to_key(self.getOptionComparator()))

        for i, option in enumerate(optList):
            group = options.getOptionGroup(option)

            if group is not None:
                if group not in processedGroups:
                    processedGroups.append(group)
                    self.__appendOptionGroup(buff, group)
            else:
                self.__appendOption(buff, option, option.isRequired())

            if i < len(optList) - 1:
                buff.write(" ")

        buffStr = buff.getvalue()
        self.printWrapped0(pw, width, buffStr.find(" ") + 1, buffStr)

    def printUsage0(
        self,
        pw: typing.Union[io.TextIOWrapper, io.StringIO],
        width: int,
        cmdLineSyntax: str,
    ) -> None:
        argPos = cmdLineSyntax.find(" ") + 1

        self.printWrapped0(
            pw,
            width,
            len(self.getSyntaxPrefix()) + argPos,
            self.getSyntaxPrefix() + cmdLineSyntax,
        )

    def printOptions(
        self,
        pw: typing.Union[io.TextIOWrapper, io.StringIO],
        width: int,
        options: Options,
        leftPad: int,
        descPad: int,
    ) -> None:
        sb = io.StringIO()
        self._renderOptions(sb, width, options, leftPad, descPad)
        pw.write(sb.getvalue() + "\n")

    def printHelp7(
        self,
        cmdLineSyntax: str,
        header: str,
        options: Options,
        footer: str,
        autoUsage: bool,
    ) -> None:

        pass  # LLM could not translate this method

    def printHelp6(self, cmdLineSyntax: str, header: str, options: Options, footer: str) -> None:

        pass  # LLM could not translate this method

    def printHelp5(self, cmdLineSyntax: str, options: Options, autoUsage: bool) -> None:
        self.printHelp1(self.getWidth(), cmdLineSyntax, None, options, None, autoUsage)

    def printHelp4(self, cmdLineSyntax: str, options: Options) -> None:

        pass  # LLM could not translate this method

    def printHelp3(
        self,
        pw: typing.Union[io.TextIOWrapper, io.StringIO],
        width: int,
        cmdLineSyntax: str,
        header: str,
        options: Options,
        leftPad: int,
        descPad: int,
        footer: str,
        autoUsage: bool,
    ) -> None:
        if cmdLineSyntax is None or cmdLineSyntax == "":
            raise ValueError("cmdLineSyntax not provided")

        if autoUsage:
            self.printUsage1(pw, width, cmdLineSyntax, options)
        else:
            self.printUsage0(pw, width, cmdLineSyntax)

        if header is not None and header != "":
            self.printWrapped1(pw, width, header)

        self.printOptions(pw, width, options, leftPad, descPad)

        if footer is not None and footer != "":
            self.printWrapped1(pw, width, footer)

        pw.flush()

    def printHelp2(
        self,
        pw: typing.Union[io.TextIOWrapper, io.StringIO],
        width: int,
        cmdLineSyntax: str,
        header: str,
        options: Options,
        leftPad: int,
        descPad: int,
        footer: str,
    ) -> None:

        pass  # LLM could not translate this method

    def printHelp1(
        self,
        width: int,
        cmdLineSyntax: str,
        header: str,
        options: Options,
        footer: str,
        autoUsage: bool,
    ) -> None:
        import sys

        pw = sys.stdout

        self.printHelp3(
            pw,
            width,
            cmdLineSyntax,
            header,
            options,
            self.getLeftPadding(),
            self.getDescPadding(),
            footer,
            autoUsage,
        )
        pw.flush()

    def printHelp0(self, width: int, cmdLineSyntax: str, header: str, options: Options, footer: str) -> None:
        self.printHelp1(width, cmdLineSyntax, header, options, footer, False)

    def getWidth(self) -> int:
        return self.defaultWidth

    def getSyntaxPrefix(self) -> str:
        return self.defaultSyntaxPrefix

    def getOptPrefix(self) -> str:
        return self.defaultOptPrefix

    def getOptionComparator(self) -> typing.Callable[[Option, Option], int]:
        return self._optionComparator

    def getNewLine(self) -> str:
        return self.defaultNewLine

    def getLongOptSeparator(self) -> str:
        return self.__longOptSeparator

    def getLongOptPrefix(self) -> str:
        return self.defaultLongOptPrefix

    def getLeftPadding(self) -> int:
        return self.defaultLeftPad

    def getDescPadding(self) -> int:
        return self.defaultDescPad

    def getArgName(self) -> str:
        return self.defaultArgName

    def _findWrapPos(self, text: str, width: int, startPos: int) -> int:
        pos = text.find("\n", startPos)
        if pos != -1 and pos <= width:
            return pos + 1

        pos = text.find("\t", startPos)
        if pos != -1 and pos <= width:
            return pos + 1

        if startPos + width >= len(text):
            return -1

        pos = startPos + width
        while pos >= startPos:
            c = text[pos]
            if c == " " or c == "\n" or c == "\r":
                break
            pos -= 1

        if pos > startPos:
            return pos

        pos = startPos + width

        return -1 if pos == len(text) else pos

    def _createPadding(self, len_: int) -> str:
        return " " * len_

    def __renderWrappedTextBlock(
        self, sb: io.StringIO, width: int, nextLineTabStop: int, text: str
    ) -> typing.Union[typing.List, io.TextIOBase]:
        try:
            reader = io.StringIO(text)
            firstLine = True
            while True:
                line = reader.readline()
                if not line:
                    break
                # Remove the trailing newline if present
                if line.endswith("\n"):
                    line = line[:-1]

                if not firstLine:
                    sb.write(self.getNewLine())
                else:
                    firstLine = False

                self._renderWrappedText(sb, width, nextLineTabStop, line)
        except IOError:
            pass

        return sb

    def __appendOptionGroup(self, buff: io.StringIO, group: OptionGroup) -> None:
        if not group.isRequired():
            buff.write("[")

        optList = list(group.getOptions())
        if self.getOptionComparator() is not None:
            optList.sort(key=functools.cmp_to_key(self.getOptionComparator()))

        for i, option in enumerate(optList):
            self.__appendOption(buff, option, True)

            if i < len(optList) - 1:
                buff.write(" | ")

        if not group.isRequired():
            buff.write("]")

    def __appendOption(self, buff: io.StringIO, option: Option, required: bool) -> None:

        pass  # LLM could not translate this method


class OptionComparator:

    __serialVersionUID: int = 5305467873966684014

    def compare(self, opt1: Option, opt2: Option) -> int:
        key1 = opt1.getKey().lower()
        key2 = opt2.getKey().lower()
        if key1 < key2:
            return -1
        elif key1 > key2:
            return 1
        else:
            return 0
