from __future__ import annotations
import re
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
    DEFAULT_OPT_PREFIX: str = "-"
    defaultNewLine: str = os.getenv("line.separator", "\n")
    DEFAULT_SYNTAX_PREFIX: str = "usage: "
    DEFAULT_DESC_PAD: int = 3
    DEFAULT_LEFT_PAD: int = 1
    DEFAULT_WIDTH: int = 74
    DEFAULT_ARG_NAME: str = "arg"
    DEFAULT_LONG_OPT_SEPARATOR: str = " "
    DEFAULT_LONG_OPT_PREFIX: str = "--"
    __longOptSeparator: str = DEFAULT_LONG_OPT_SEPARATOR
    defaultArgName: str = None
    defaultLongOptPrefix: str = None

    @staticmethod
    def initialize_fields() -> None:
        HelpFormatter._optionComparator: typing.Callable[[Option, Option], int] = (
            OptionComparator()
        )

        HelpFormatter.defaultArgName: str = HelpFormatter.DEFAULT_ARG_NAME

        HelpFormatter.defaultLongOptPrefix: str = HelpFormatter.DEFAULT_LONG_OPT_PREFIX

    def setWidth(self, width: int) -> None:
        self.defaultWidth = width

    def setSyntaxPrefix(self, prefix: str) -> None:
        self.defaultSyntaxPrefix = prefix

    def setOptPrefix(self, prefix: str) -> None:
        self.defaultOptPrefix = prefix

    def setOptionComparator(
        self, comparator: typing.Callable[[Option, Option], int]
    ) -> None:
        self._optionComparator = comparator

    def setNewLine(self, newline: str) -> None:
        self.defaultNewLine = newline

    def setLongOptSeparator(self, longOptSeparator: str) -> None:
        self.__longOptSeparator = longOptSeparator

    def setLongOptPrefix(self, prefix: str) -> None:
        self.defaultLongOptPrefix = prefix

    def setLeftPadding(self, padding: int) -> None:
        self.defaultLeftPad = padding

    def setDescPadding(self, padding: int) -> None:
        self.defaultDescPad = padding

    def setArgName(self, name: str) -> None:
        self.defaultArgName = name

    def _rtrim(self, s: str) -> str:
        if s is None or s == "":
            return s

        pos = len(s)

        while pos > 0 and s[pos - 1].isspace():
            pos -= 1

        return s[:pos]

    def _renderWrappedText(
        self, sb: io.StringIO, width: int, nextLineTabStop: int, text: str
    ) -> io.StringIO:
        pos = self._findWrapPos(text, width, 0)

        if pos == -1:
            sb.write(self._rtrim(text))
            return sb

        sb.write(self._rtrim(text[:pos]) + self.getNewLine())

        if nextLineTabStop >= width:
            nextLineTabStop = 1

        padding = self._createPadding(nextLineTabStop)

        while True:
            text = padding + text[pos:].strip()
            pos = self._findWrapPos(text, width, 0)

            if pos == -1:
                sb.write(text)
                return sb

            if len(text) > width and pos >= width:
                pos = width

            sb.write(self._rtrim(text[:pos]) + self.getNewLine())

    def _renderOptions(
        self, sb: io.StringIO, width: int, options: Options, leftPad: int, descPad: int
    ) -> io.StringIO:
        lpad = self._createPadding(leftPad)
        dpad = self._createPadding(descPad)

        max_len = 0
        prefix_list = []

        opt_list = options.helpOptions()

        if self.getOptionComparator() is not None:
            opt_list.sort(key=self.getOptionComparator())

        for option in opt_list:
            opt_buf = io.StringIO()

            if option.getOpt() is None:
                opt_buf.write(lpad)
                opt_buf.write("   ")
                opt_buf.write(self.getLongOptPrefix())
                opt_buf.write(option.getLongOpt())
            else:
                opt_buf.write(lpad)
                opt_buf.write(self.getOptPrefix())
                opt_buf.write(option.getOpt())

                if option.hasLongOpt():
                    opt_buf.write(",")
                    opt_buf.write(self.getLongOptPrefix())
                    opt_buf.write(option.getLongOpt())

            if option.hasArg():
                arg_name = option.getArgName()
                if arg_name is not None and arg_name == "":
                    opt_buf.write(" ")
                else:
                    opt_buf.write(
                        self.__longOptSeparator if option.hasLongOpt() else " "
                    )
                    opt_buf.write("<")
                    opt_buf.write(
                        arg_name if arg_name is not None else self.getArgName()
                    )
                    opt_buf.write(">")

            prefix_list.append(opt_buf.getvalue())
            max_len = max(max_len, len(opt_buf.getvalue()))

        for idx, option in enumerate(opt_list):
            opt_buf = io.StringIO(prefix_list[idx])

            if len(opt_buf.getvalue()) < max_len:
                opt_buf.write(self._createPadding(max_len - len(opt_buf.getvalue())))

            opt_buf.write(dpad)

            next_line_tab_stop = max_len + descPad

            if option.getDescription() is not None:
                opt_buf.write(option.getDescription())

            self._renderWrappedText(sb, width, next_line_tab_stop, opt_buf.getvalue())

            if idx < len(opt_list) - 1:
                sb.write(self.getNewLine())

        return sb

    def printWrapped1(
        self, pw: typing.Union[io.TextIOWrapper, io.StringIO], width: int, text: str
    ) -> None:
        self.printWrapped0(pw, width, 0, text)

    def printWrapped0(
        self,
        pw: typing.Union[io.TextIOWrapper, io.StringIO],
        width: int,
        nextLineTabStop: int,
        text: str,
    ) -> None:
        sb = io.StringIO()  # Create a StringIO object to mimic StringBuffer
        self.__renderWrappedTextBlock(
            sb, width, nextLineTabStop, text
        )  # Call the helper method to render the text
        pw.write(
            sb.getvalue() + "\n"
        )  # Write the content of sb to the PrintWriter (pw) and add a newline

    def printUsage1(
        self,
        pw: typing.Union[io.TextIOWrapper, io.StringIO],
        width: int,
        app: str,
        options: Options,
    ) -> None:
        buff = io.StringIO()
        buff.write(self.getSyntaxPrefix() + app + " ")

        processed_groups = set()

        opt_list = list(options.getOptions())
        if self.getOptionComparator() is not None:
            opt_list.sort(key=self.getOptionComparator())

        for i, option in enumerate(opt_list):
            group = options.getOptionGroup(option)

            if group is not None:
                if group not in processed_groups:
                    processed_groups.add(group)
                    self.__appendOptionGroup(buff, group)
            else:
                self.__appendOption(buff, option, option.isRequired())

            if i < len(opt_list) - 1:
                buff.write(" ")

        self.printWrapped0(pw, width, buff.getvalue().index(" ") + 1, buff.getvalue())

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
        pw.write(sb.getvalue())

    def printHelp7(
        self,
        cmdLineSyntax: str,
        header: str,
        options: Options,
        footer: str,
        autoUsage: bool,
    ) -> None:
        self.printHelp1(
            self.getWidth(), cmdLineSyntax, header, options, footer, autoUsage
        )

    def printHelp6(
        self, cmdLineSyntax: str, header: str, options: Options, footer: str
    ) -> None:
        self.printHelp7(cmdLineSyntax, header, options, footer, False)

    def printHelp5(self, cmdLineSyntax: str, options: Options, autoUsage: bool) -> None:
        self.printHelp1(self.getWidth(), cmdLineSyntax, None, options, None, autoUsage)

    def printHelp4(self, cmdLineSyntax: str, options: Options) -> None:
        self.printHelp1(self.getWidth(), cmdLineSyntax, None, options, None, False)

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
        if not cmdLineSyntax:
            raise ValueError("cmdLineSyntax not provided")

        if autoUsage:
            self.printUsage1(pw, width, cmdLineSyntax, options)
        else:
            self.printUsage0(pw, width, cmdLineSyntax)

        if header:
            self.printWrapped1(pw, width, header)

        self.printOptions(pw, width, options, leftPad, descPad)

        if footer:
            self.printWrapped1(pw, width, footer)

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
        self.printHelp3(
            pw, width, cmdLineSyntax, header, options, leftPad, descPad, footer, False
        )

    def printHelp1(
        self,
        width: int,
        cmdLineSyntax: str,
        header: str,
        options: Options,
        footer: str,
        autoUsage: bool,
    ) -> None:
        pw = io.StringIO()

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
        print(pw.getvalue())
        pw.close()

    def printHelp0(
        self, width: int, cmdLineSyntax: str, header: str, options: Options, footer: str
    ) -> None:
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
        if pos != -1 and pos <= startPos + width:
            return pos + 1

        pos = text.find("\t", startPos)
        if pos != -1 and pos <= startPos + width:
            return pos + 1

        if startPos + width >= len(text):
            return -1

        for pos in range(startPos + width, startPos - 1, -1):
            c = text[pos]
            if c == " " or c == "\n" or c == "\r":
                break

        if pos > startPos:
            return pos

        pos = startPos + width

        return -1 if pos == len(text) else pos

    def _createPadding(self, len_: int) -> str:
        padding = [" "] * len_
        return "".join(padding)

    def __renderWrappedTextBlock(
        self, sb: io.StringIO, width: int, nextLineTabStop: int, text: str
    ) -> io.StringIO:
        try:
            in_buffer = io.StringIO(text)
            first_line = True
            for line in in_buffer:
                line = line.rstrip("\n")  # Remove trailing newline
                if not first_line:
                    sb.write(self.getNewLine())
                else:
                    first_line = False
                self._renderWrappedText(sb, width, nextLineTabStop, line)
        except Exception as e:
            # In Python, we generally avoid empty exception handling.
            # However, to mimic the Java code's behavior, we silently pass here.
            pass

        return sb

    def __appendOptionGroup(self, buff: io.StringIO, group: OptionGroup) -> None:
        if not group.isRequired():
            buff.write("[")

        opt_list = list(group.getOptions())
        if self.getOptionComparator() is not None:
            opt_list.sort(key=self.getOptionComparator())

        for i, option in enumerate(opt_list):
            self.__appendOption(buff, option, True)

            if i < len(opt_list) - 1:
                buff.write(" | ")

        if not group.isRequired():
            buff.write("]")

    def __appendOption(self, buff: io.StringIO, option: Option, required: bool) -> None:
        if not required:
            buff.write("[")

        if option.getOpt() is not None:
            buff.write("-" + option.getOpt())
        else:
            buff.write("--" + option.getLongOpt())

        if option.hasArg() and (option.getArgName() is None or option.getArgName()):
            buff.write(self.__longOptSeparator if option.getOpt() is None else " ")
            buff.write(
                "<"
                + (
                    option.getArgName()
                    if option.getArgName() is not None
                    else self.getArgName()
                )
                + ">"
            )

        if not required:
            buff.write("]")


class OptionComparator:

    __serialVersionUID: int = 5305467873966684014

    def compare(self, opt1: Option, opt2: Option) -> int:
        return (opt1.getKey().casefold() > opt2.getKey().casefold()) - (
            opt1.getKey().casefold() < opt2.getKey().casefold()
        )


HelpFormatter.initialize_fields()
