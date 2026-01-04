from __future__ import annotations
import re
import sys
import io
import typing
from typing import *
import configparser
from src.main.org.apache.commons.cli.Option import *
from src.main.org.apache.commons.cli.ParseException import *
from src.main.org.apache.commons.cli.TypeHandler import *
from src.main.org.apache.commons.cli.Util import *


class CommandLine:

    __options: typing.List[Option] = []

    __args: typing.List[str] = []

    __serialVersionUID: int = 1

    def getOptionObject1(self, opt: str) -> typing.Any:
        try:
            return self.getParsedOptionValue2(opt)
        except ParseException as pe:
            print(
                f"Exception found converting {opt} to desired type: {pe}",
                file=io.sys.stderr,
            )
            return None

    def getOptionObject0(self, opt: str) -> typing.Any:

        pass  # LLM could not translate this method

    def iterator(self) -> typing.Iterator[Option]:
        return iter(self.__options)

    def hasOption2(self, opt: str) -> bool:

        pass  # LLM could not translate this method

    def hasOption1(self, opt: Option) -> bool:

        pass  # LLM could not translate this method

    def hasOption0(self, opt: str) -> bool:
        return self.hasOption2(opt)

    def getParsedOptionValue2(self, opt: str) -> typing.Any:

        pass  # LLM could not translate this method

    def getParsedOptionValue1(self, option: Option) -> typing.Any:
        if option is None:
            return None
        res: str = self.getOptionValue2(option)
        if res is None:
            return None
        return TypeHandler.createValue1(res, option.getType())

    def getParsedOptionValue0(self, opt: str) -> typing.Any:
        return self.getParsedOptionValue2(opt)

    def getOptionValues2(self, opt: str) -> typing.List[typing.List[str]]:

        pass  # LLM could not translate this method

    def getOptionValues1(self, option: Option) -> typing.Optional[typing.List[str]]:
        values: typing.List[str] = []

        for processedOption in self.__options:
            if processedOption.equals(option):
                values.extend(processedOption.getValuesList())

        return None if len(values) == 0 else values

    def getOptionValues0(self, opt: str) -> typing.List[typing.List[str]]:
        return self.getOptionValues2(opt)

    def getOptionValue5(self, opt: str, defaultValue: str) -> str:

        pass  # LLM could not translate this method

    def getOptionValue4(self, opt: str) -> str:

        pass  # LLM could not translate this method

    def getOptionValue3(self, option: Option, defaultValue: str) -> str:

        pass  # LLM could not translate this method

    def getOptionValue2(self, option: Option) -> str:
        if option is None:
            return None
        values: typing.List[str] = self.getOptionValues1(option)
        return None if values is None else values[0]

    def getOptionValue1(self, opt: str, defaultValue: str) -> str:

        pass  # LLM could not translate this method

    def getOptionValue0(self, opt: str) -> str:
        return self.getOptionValue4(opt)

    def getOptions(self) -> typing.List[Option]:
        processed = self.__options
        return list(processed)

    def getOptionProperties1(self, opt: str) -> typing.Union[configparser.ConfigParser, typing.Dict]:
        props = {}

        for option in self.__options:
            if opt == option.getOpt() or opt == option.getLongOpt():
                values = option.getValuesList()
                if len(values) >= 2:
                    props[values[0]] = values[1]
                elif len(values) == 1:
                    props[values[0]] = "true"

        return props

    def getOptionProperties0(self, option: Option) -> typing.Union[configparser.ConfigParser, typing.Dict]:
        props = {}

        for processedOption in self.__options:
            if processedOption.equals(option):
                values = processedOption.getValuesList()
                if len(values) >= 2:
                    props[values[0]] = values[1]
                elif len(values) == 1:
                    props[values[0]] = "true"

        return props

    def getArgs(self) -> typing.List[str]:
        return list(self.__args)

    def getArgList(self) -> typing.List[str]:
        return self.__args

    def _addOption(self, opt: Option) -> None:
        self.__options.append(opt)

    def _addArg(self, arg: str) -> None:
        self.__args.append(arg)

    def __init__(self) -> None:
        if type(self) is CommandLine:
            raise TypeError("Cannot instantiate CommandLine directly")

    def __resolveOption(self, opt: str) -> Option:
        opt = Util.stripLeadingHyphens(opt)
        for option in self.__options:
            if opt == option.getOpt() or opt == option.getLongOpt():
                return option
        return None


class Builder:

    __commandLine: CommandLine = None

    @staticmethod
    def initialize_fields() -> None:
        Builder.__commandLine: CommandLine = CommandLine()

    def build(self) -> CommandLine:
        return self.__commandLine

    def addOption(self, opt: Option) -> Builder:
        self.__commandLine._addOption(opt)
        return self

    def addArg(self, arg: str) -> Builder:
        self.__commandLine._addArg(arg)
        return self


Builder.initialize_fields()
