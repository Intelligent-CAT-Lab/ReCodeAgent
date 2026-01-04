from __future__ import annotations
import time
import re
from abc import ABC
import io
import typing
from typing import *
import configparser
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


class Parser(CommandLineParser, ABC):

    _cmd: CommandLine = None

    __requiredOptions: typing.List[typing.Any] = None

    __options: Options = None

    def _setOptions(self, options: Options) -> None:

        pass  # LLM could not translate this method

    def _processProperties(self, properties: typing.Union[configparser.ConfigParser, typing.Dict]) -> None:
        if properties is None:
            return

        # Handle both dict and ConfigParser
        if isinstance(properties, configparser.ConfigParser):
            # Convert ConfigParser to dict-like iteration
            property_items = []
            for section in properties.sections():
                for key in properties.options(section):
                    property_items.append((key, properties.get(section, key)))
        else:
            property_items = properties.items()

        for option, value in property_items:
            opt = self.__options.getOption(option)
            if opt is None:
                raise UnrecognizedOptionException("Default option wasn't defined", option)

            group = self.__options.getOptionGroup(opt)
            selected = group is not None and group.getSelected() is not None

            if not self._cmd.hasOption2(option) and not selected:
                if opt.hasArg():
                    if opt.getValues() is None or len(opt.getValues()) == 0:
                        try:
                            opt.addValueForProcessing(value)
                        except RuntimeError:
                            pass
                elif not (value.lower() == "yes" or value.lower() == "true" or value == "1"):
                    continue

                self._cmd._addOption(opt)
                self.__updateRequiredOptions(opt)

    def _processOption(self, arg: str, iter_: typing.Iterator[str]) -> None:
        hasOption = self._getOptions().hasOption(arg)

        if not hasOption:
            raise UnrecognizedOptionException("Unrecognized option: " + arg, arg)

        opt = self._getOptions().getOption(arg).clone()

        self.__updateRequiredOptions(opt)

        if opt.hasArg():
            self.processArgs(opt, iter_)

        self._cmd.addOption(opt)

    def processArgs(self, opt: Option, iter_: typing.Iterator[str]) -> None:
        while True:
            try:
                str_ = next(iter_)
            except StopIteration:
                break

            if self._getOptions().hasOption(str_) and str_.startswith("-"):
                # Move back one element (simulate iter.previous())
                # Note: Python iterators don't have previous(), so we break here
                # The caller would need to handle this differently or use a different approach
                break

            try:
                opt.addValueForProcessing(Util.stripLeadingAndTrailingQuotes(str_))
            except RuntimeError as exp:
                # iter.previous() - can't go back in Python iterator
                break

        if opt.getValues() is None and not opt.hasOptionalArg():
            raise MissingArgumentException.MissingArgumentException1(1, None, opt)

    def parse3(
        self,
        options: Options,
        arguments: typing.List[str],
        properties: typing.Union[configparser.ConfigParser, typing.Dict],
        stopAtNonOption: bool,
    ) -> CommandLine:
        for opt in options.helpOptions():
            opt.clearValues()

        for group in options.getOptionGroups():
            group.setSelected(None)

        self._setOptions(options)

        self._cmd = CommandLine()

        eatTheRest = False

        if arguments is None:
            arguments = []

        tokenList = list(self._flatten(self._getOptions(), arguments, stopAtNonOption))

        iterator = iter(tokenList)

        while True:
            try:
                t = next(iterator)
            except StopIteration:
                break

            if t == "--":
                eatTheRest = True
            elif t == "-":
                if stopAtNonOption:
                    eatTheRest = True
                else:
                    self._cmd._addArg(t)
            elif t.startswith("-"):
                if stopAtNonOption and not self._getOptions().hasOption(t):
                    eatTheRest = True
                    self._cmd._addArg(t)
                else:
                    self._processOption(t, iterator)
            else:
                self._cmd._addArg(t)

                if stopAtNonOption:
                    eatTheRest = True

            if eatTheRest:
                while True:
                    try:
                        str_ = next(iterator)
                        if str_ != "--":
                            self._cmd._addArg(str_)
                    except StopIteration:
                        break

        self._processProperties(properties)
        self._checkRequiredOptions()

        return self._cmd

    def parse2(
        self,
        options: Options,
        arguments: typing.List[str],
        properties: typing.Union[configparser.ConfigParser, typing.Dict],
    ) -> CommandLine:
        return self.parse3(options, arguments, properties, False)

    def parse1(
        self,
        options: Options,
        arguments: typing.List[typing.List[str]],
        stopAtNonOption: bool,
    ) -> CommandLine:

        pass  # LLM could not translate this method

    def parse0(self, options: Options, arguments: typing.List[typing.List[str]]) -> CommandLine:

        pass  # LLM could not translate this method

    def _getRequiredOptions(self) -> typing.List[typing.Any]:
        return self.__requiredOptions

    def _getOptions(self) -> Options:
        return self.__options

    def _checkRequiredOptions(self) -> None:
        if len(self._getRequiredOptions()) > 0:
            raise MissingOptionException.MissingOptionException1(1, self._getRequiredOptions(), None)

    def __updateRequiredOptions(self, opt: Option) -> None:
        if opt.isRequired():
            self._getRequiredOptions().remove(opt.getKey())

        if self._getOptions().getOptionGroup(opt) is not None:
            group = self._getOptions().getOptionGroup(opt)

            if group.isRequired():
                self._getRequiredOptions().remove(group)

            group.setSelected(opt)

    def _flatten(
        self,
        opts: Options,
        arguments: typing.List[typing.List[str]],
        stopAtNonOption: bool,
    ) -> typing.List[typing.List[str]]:
        raise NotImplementedError("Subclasses must implement _flatten method")
