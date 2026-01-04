from __future__ import annotations
import re
import os
import io
import numbers
import typing
from typing import *
import configparser
from src.main.org.apache.commons.cli.AlreadySelectedException import *
from src.main.org.apache.commons.cli.AmbiguousOptionException import *
from src.main.org.apache.commons.cli.CommandLine import *
from src.main.org.apache.commons.cli.CommandLineParser import *
from src.main.org.apache.commons.cli.MissingArgumentException import *
from src.main.org.apache.commons.cli.MissingOptionException import *
from src.main.org.apache.commons.cli.Option import *
from src.main.org.apache.commons.cli.OptionGroup import *
from src.main.org.apache.commons.cli.Options import *
from src.main.org.apache.commons.cli.ParseException import *
from src.main.org.apache.commons.cli.UnrecognizedOptionException import *
from src.main.org.apache.commons.cli.Util import *


class DefaultParser(CommandLineParser):

    _expectedOpts: typing.List[typing.Any] = None

    _skipParsing: bool = False

    _currentOption: Option = None

    _currentToken: str = ""

    _stopAtNonOption: bool = False

    _options: Options = None

    _cmd: CommandLine = None

    __stripLeadingAndTrailingQuotes: bool = False

    __allowPartialMatching: bool = False

    def parse3(
        self,
        options: Options,
        arguments: typing.List[typing.List[str]],
        properties: typing.Union[configparser.ConfigParser, typing.Dict],
        stopAtNonOption: bool,
    ) -> CommandLine:
        self._options = options
        self._stopAtNonOption = stopAtNonOption
        self._skipParsing = False
        self._currentOption = None
        self._expectedOpts = list(options.getRequiredOptions())

        for group in options.getOptionGroups():
            group.setSelected(None)

        self._cmd = CommandLine()

        if arguments is not None:
            for argument in arguments:
                self.__handleToken(argument)

        self.__checkRequiredArgs()

        self.__handleProperties(properties)

        self._checkRequiredOptions()

        return self._cmd

    def parse2(
        self,
        options: Options,
        arguments: typing.List[typing.List[str]],
        properties: typing.Union[configparser.ConfigParser, typing.Dict],
    ) -> CommandLine:
        return self.parse3(options, arguments, properties, False)

    def parse1(
        self,
        options: Options,
        arguments: typing.List[typing.List[str]],
        stopAtNonOption: bool,
    ) -> CommandLine:
        return self.parse3(options, arguments, None, stopAtNonOption)

    def parse0(self, options: Options, arguments: typing.List[typing.List[str]]) -> CommandLine:

        pass  # LLM could not translate this method

    def _handleConcatenatedOptions(self, token: str) -> None:
        i = 1
        while i < len(token):
            ch = str(token[i])

            if not self._options.hasOption(ch):
                self.__handleUnknownToken(token[i:] if self._stopAtNonOption and i > 1 else token)
                break

            self.__handleOption(self._options.getOption(ch))

            if self._currentOption is not None and len(token) != i + 1:
                self._currentOption.addValueForProcessing(
                    self.__stripLeadingAndTrailingQuotesDefaultOff(token[i + 1 :])
                )
                break

            i += 1

    def _checkRequiredOptions(self) -> None:
        if self._expectedOpts:
            raise MissingOptionException.MissingOptionException1(1, self._expectedOpts, None)

    def __init__(
        self,
        constructorId: int,
        allowPartialMatching: bool,
        stripLeadingAndTrailingQuotes: bool,
    ) -> None:
        if constructorId == 0:
            self._DefaultParser__allowPartialMatching = allowPartialMatching
            self._DefaultParser__stripLeadingAndTrailingQuotes = None
        elif constructorId == 1:
            self._DefaultParser__allowPartialMatching = allowPartialMatching
            self._DefaultParser__stripLeadingAndTrailingQuotes = stripLeadingAndTrailingQuotes
        else:
            self._DefaultParser__allowPartialMatching = True
            self._DefaultParser__stripLeadingAndTrailingQuotes = None

    @staticmethod
    def builder() -> Builder:
        return Builder()

    def __updateRequiredOptions(self, option: Option) -> None:
        if option.isRequired():
            self._expectedOpts.remove(option.getKey())

        if self._options.getOptionGroup(option) is not None:
            group = self._options.getOptionGroup(option)

            if group.isRequired():
                self._expectedOpts.remove(group)

            group.setSelected(option)

    def __stripLeadingAndTrailingQuotesDefaultOn(self, token: str) -> str:
        if self.__stripLeadingAndTrailingQuotes is None or self.__stripLeadingAndTrailingQuotes:
            return Util.stripLeadingAndTrailingQuotes(token)
        return token

    def __stripLeadingAndTrailingQuotesDefaultOff(self, token: str) -> str:
        if self.__stripLeadingAndTrailingQuotes is not None and self.__stripLeadingAndTrailingQuotes:
            return Util.stripLeadingAndTrailingQuotes(token)
        return token

    def __isShortOption(self, token: str) -> bool:
        if token is None or not token.startswith("-") or len(token) == 1:
            return False

        pos = token.find("=")
        optName = token[1:] if pos == -1 else token[1:pos]
        if self._options.hasShortOption(optName):
            return True
        return len(optName) > 0 and self._options.hasShortOption(optName[0])

    def __isOption(self, token: str) -> bool:
        return self.__isLongOption(token) or self.__isShortOption(token)

    def __isNegativeNumber(self, token: str) -> bool:
        try:
            float(token)
            return True
        except ValueError:
            return False

    def __isLongOption(self, token: str) -> bool:
        if token is None or not token.startswith("-") or len(token) == 1:
            return False

        pos = token.find("=")
        t = token if pos == -1 else token[0:pos]

        if len(self.__getMatchingLongOptions(t)) > 0:
            return True
        if self.__getLongPrefix(token) is not None and not token.startswith("--"):
            return True

        return False

    def __isJavaProperty(self, token: str) -> bool:
        opt = token[0:1]
        option = self._options.getOption(opt)

        return option is not None and (option.getArgs() >= 2 or option.getArgs() == Option.UNLIMITED_VALUES)

    def __isArgument(self, token: str) -> bool:

        pass  # LLM could not translate this method

    def __handleUnknownToken(self, token: str) -> None:
        if token.startswith("-") and len(token) > 1 and not self._stopAtNonOption:
            raise UnrecognizedOptionException("Unrecognized option: " + token, token)

        self._cmd._addArg(token)
        if self._stopAtNonOption:
            self._skipParsing = True

    def __handleToken(self, token: str) -> None:
        self._currentToken = token

        if self._skipParsing:
            self._cmd._addArg(token)
        elif token == "--":
            self._skipParsing = True
        elif self._currentOption is not None and self._currentOption.acceptsArg() and self.__isArgument(token):
            self._currentOption.addValueForProcessing(self.__stripLeadingAndTrailingQuotesDefaultOn(token))
        elif token.startswith("--"):
            self.__handleLongOption(token)
        elif token.startswith("-") and token != "-":
            self.__handleShortAndLongOption(token)
        else:
            self.__handleUnknownToken(token)

        if self._currentOption is not None and not self._currentOption.acceptsArg():
            self._currentOption = None

    def __handleShortAndLongOption(self, token: str) -> None:
        t = Util.stripLeadingHyphens(token)

        pos = t.find("=")

        if len(t) == 1:
            if self._options.hasShortOption(t):
                self.__handleOption(self._options.getOption(t))
            else:
                self.__handleUnknownToken(token)
        elif pos == -1:
            if self._options.hasShortOption(t):
                self.__handleOption(self._options.getOption(t))
            elif len(self.__getMatchingLongOptions(t)) > 0:
                self.__handleLongOptionWithoutEqual(token)
            else:
                opt = self.__getLongPrefix(t)

                if opt is not None and self._options.getOption(opt).acceptsArg():
                    self.__handleOption(self._options.getOption(opt))
                    self._currentOption.addValueForProcessing(
                        self.__stripLeadingAndTrailingQuotesDefaultOff(t[len(opt) :])
                    )
                    self._currentOption = None
                elif self.__isJavaProperty(t):
                    self.__handleOption(self._options.getOption(t[0:1]))
                    self._currentOption.addValueForProcessing(self.__stripLeadingAndTrailingQuotesDefaultOff(t[1:]))
                    self._currentOption = None
                else:
                    self._handleConcatenatedOptions(token)
        else:
            opt = t[0:pos]
            value = t[pos + 1 :]

            if len(opt) == 1:
                option = self._options.getOption(opt)
                if option is not None and option.acceptsArg():
                    self.__handleOption(option)
                    self._currentOption.addValueForProcessing(value)
                    self._currentOption = None
                else:
                    self.__handleUnknownToken(token)
            elif self.__isJavaProperty(opt):
                self.__handleOption(self._options.getOption(opt[0:1]))
                self._currentOption.addValueForProcessing(opt[1:])
                self._currentOption.addValueForProcessing(value)
                self._currentOption = None
            else:
                self.__handleLongOptionWithEqual(token)

    def __handleProperties(self, properties: typing.Union[configparser.ConfigParser, typing.Dict]) -> None:
        if properties is None:
            return

        # Handle both dict and ConfigParser
        if isinstance(properties, configparser.ConfigParser):
            property_items = [
                (key, properties.get(section, key))
                for section in properties.sections()
                for key in properties.options(section)
            ]
        else:
            property_items = list(properties.items())

        for option, value in property_items:
            opt = self._options.getOption(option)
            if opt is None:
                raise UnrecognizedOptionException("Default option wasn't defined", option)

            group = self._options.getOptionGroup(opt)
            selected = group is not None and group.getSelected() is not None

            if not self._cmd.hasOption2(option) and not selected:
                if opt.hasArg():
                    if opt.getValues() is None or len(opt.getValues()) == 0:
                        opt.addValueForProcessing(self.__stripLeadingAndTrailingQuotesDefaultOff(value))
                elif not (value.lower() == "yes" or value.lower() == "true" or value.lower() == "1"):
                    continue

                self.__handleOption(opt)
                self._currentOption = None

    def __handleOption(self, option: Option) -> None:
        self.__checkRequiredArgs()

        option = option.clone()

        self.__updateRequiredOptions(option)

        self._cmd._addOption(option)

        if option.hasArg():
            self._currentOption = option
        else:
            self._currentOption = None

    def __handleLongOptionWithoutEqual(self, token: str) -> None:
        matchingOpts: typing.List[str] = self.__getMatchingLongOptions(token)
        if not matchingOpts:
            self.__handleUnknownToken(self._currentToken)
        elif len(matchingOpts) > 1 and not self._options.hasLongOption(token):
            raise AmbiguousOptionException(token, matchingOpts)
        else:
            key: str = token if self._options.hasLongOption(token) else matchingOpts[0]
            self.__handleOption(self._options.getOption(key))

    def __handleLongOptionWithEqual(self, token: str) -> None:
        pos = token.find("=")

        value = token[pos + 1 :]

        opt = token[0:pos]

        matchingOpts = self.__getMatchingLongOptions(opt)
        if len(matchingOpts) == 0:
            self.__handleUnknownToken(self._currentToken)
        elif len(matchingOpts) > 1 and not self._options.hasLongOption(opt):
            raise AmbiguousOptionException(opt, matchingOpts)
        else:
            key = opt if self._options.hasLongOption(opt) else matchingOpts[0]
            option = self._options.getOption(key)

            if option.acceptsArg():
                self.__handleOption(option)
                self._currentOption.addValueForProcessing(self.__stripLeadingAndTrailingQuotesDefaultOff(value))
                self._currentOption = None
            else:
                self.__handleUnknownToken(self._currentToken)

    def __handleLongOption(self, token: str) -> None:
        if token.find("=") == -1:
            self.__handleLongOptionWithoutEqual(token)
        else:
            self.__handleLongOptionWithEqual(token)

    def __getMatchingLongOptions(self, token: str) -> typing.List[str]:
        if self._DefaultParser__allowPartialMatching:
            return self._options.getMatchingOptions(token)

        matches: typing.List[str] = []
        if self._options.hasLongOption(token):
            option: Option = self._options.getOption(token)
            matches.append(option.getLongOpt())

        return matches

    def __getLongPrefix(self, token: str) -> str:
        t = Util.stripLeadingHyphens(token)

        opt = None
        i = len(t) - 2
        while i > 1:
            prefix = t[0:i]
            if self._options.hasLongOption(prefix):
                opt = prefix
                break
            i -= 1

        return opt

    def __checkRequiredArgs(self) -> None:

        pass  # LLM could not translate this method


class Builder:

    __stripLeadingAndTrailingQuotes: bool = False

    __allowPartialMatching: bool = None  # LLM could not translate this field

    def setStripLeadingAndTrailingQuotes(self, stripLeadingAndTrailingQuotes: bool) -> Builder:
        self.__stripLeadingAndTrailingQuotes = stripLeadingAndTrailingQuotes
        return self

    def setAllowPartialMatching(self, allowPartialMatching: bool) -> Builder:
        self.__allowPartialMatching = allowPartialMatching
        return self

    def build(self) -> DefaultParser:
        return DefaultParser(1, self.__allowPartialMatching, self.__stripLeadingAndTrailingQuotes)

    def __init__(self) -> None:
        pass
