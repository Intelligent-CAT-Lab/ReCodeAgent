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
        self.__options = options
        self.__requiredOptions = list(options.getRequiredOptions())

    def _processProperties(
        self, properties: typing.Union[configparser.ConfigParser, typing.Dict]
    ) -> None:
        if properties is None:
            return

        for option in properties.keys():
            opt = self.__options.getOption(option)
            if opt is None:
                raise UnrecognizedOptionException(
                    "Default option wasn't defined", option
                )

            group = self.__options.getOptionGroup(opt)
            selected = group is not None and group.getSelected() is not None

            if not self._cmd.hasOption2(option) and not selected:
                value = properties[option]

                if opt.hasArg():
                    if opt.getValues() is None or len(opt.getValues()) == 0:
                        try:
                            opt.addValueForProcessing(value)
                        except RuntimeError:
                            pass
                elif not (value.lower() in ["yes", "true", "1"]):
                    continue

                self._cmd._addOption(opt)
                self.__updateRequiredOptions(opt)

    def _processOption(self, arg: str, iter_: typing.Iterator[str]) -> None:
        has_option = self._getOptions().hasOption(arg)

        if not has_option:
            raise UnrecognizedOptionException(f"Unrecognized option: {arg}", arg)

        opt = self._getOptions().getOption(arg).clone()

        self.__updateRequiredOptions(opt)

        if opt.hasArg():
            self.processArgs(opt, iter_)

        self._cmd._addOption(opt)

    def processArgs(self, opt: Option, iter_: typing.Iterator[str]) -> None:
        while True:
            try:
                str_ = next(iter_)
            except StopIteration:
                break

            if self._getOptions().hasOption(str_) and str_.startswith("-"):
                iter_ = self._reverseIterator(iter_)
                break

            try:
                opt.addValueForProcessing(Util.stripLeadingAndTrailingQuotes(str_))
            except RuntimeError:
                iter_ = self._reverseIterator(iter_)
                break

        if opt.getValues() is None and not opt.hasOptionalArg():
            raise MissingArgumentException.MissingArgumentException1(1, None, opt)

    def _reverseIterator(self, iter_: typing.Iterator[str]) -> typing.Iterator[str]:
        """
        Helper method to reverse the iterator by moving it one step back.
        This simulates the `previous()` method in Java's ListIterator.
        """
        return iter([*iter_][:-1])

    def parse3(
        self,
        options: Options,
        arguments: typing.List[str],
        properties: typing.Union[configparser.ConfigParser, typing.Dict],
        stopAtNonOption: bool,
    ) -> CommandLine:
        # Clear values for all help options
        for opt in options.helpOptions():
            opt.clearValues()

        # Reset selected options for all option groups
        for group in options.getOptionGroups():
            group.setSelected(None)

        # Set the options for the parser
        self._setOptions(options)

        # Initialize the CommandLine object
        self._cmd = CommandLine()

        eat_the_rest = False

        # Handle null arguments
        if arguments is None:
            arguments = []

        # Flatten the arguments
        token_list = self._flatten(self._getOptions(), arguments, stopAtNonOption)
        iterator = iter(token_list)

        # Process each token
        for t in iterator:
            if t == "--":
                eat_the_rest = True
            elif t == "-":
                if stopAtNonOption:
                    eat_the_rest = True
                else:
                    self._cmd._addArg(t)
            elif t.startswith("-"):
                if stopAtNonOption and not self._getOptions().hasOption(t):
                    eat_the_rest = True
                    self._cmd._addArg(t)
                else:
                    self._processOption(t, iterator)
            else:
                self._cmd._addArg(t)
                if stopAtNonOption:
                    eat_the_rest = True

            # If we need to eat the rest of the arguments
            if eat_the_rest:
                for remaining in iterator:
                    if remaining != "--":
                        self._cmd._addArg(remaining)

        # Process properties and check required options
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
        self, options: Options, arguments: typing.List[str], stopAtNonOption: bool
    ) -> CommandLine:
        return self.parse3(options, arguments, None, stopAtNonOption)

    def parse0(self, options: Options, arguments: typing.List[str]) -> CommandLine:
        return self.parse3(options, arguments, None, False)

    def _getRequiredOptions(self) -> typing.List[typing.Any]:
        return self.__requiredOptions

    def _getOptions(self) -> Options:
        return self.__options

    def _checkRequiredOptions(self) -> None:
        if self._getRequiredOptions():
            raise MissingOptionException.MissingOptionException1(
                1, self._getRequiredOptions(), None
            )

    def __updateRequiredOptions(self, opt: Option) -> None:
        if opt.isRequired():
            self._getRequiredOptions().remove(opt.getKey())

        option_group = self._getOptions().getOptionGroup(opt)
        if option_group is not None:
            if option_group.isRequired():
                self._getRequiredOptions().remove(option_group)

            option_group.setSelected(opt)

    def _flatten(
        self, opts: Options, arguments: List[str], stopAtNonOption: bool
    ) -> List[str]:
        raise ParseException("This method must be implemented by a subclass")
