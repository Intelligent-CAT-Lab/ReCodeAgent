from __future__ import annotations
import re
from abc import ABC
import io
import typing
from typing import *
from src.main.org.apache.commons.cli.CommandLine import *
from src.main.org.apache.commons.cli.Options import *
from src.main.org.apache.commons.cli.ParseException import *


class CommandLineParser(ABC):

    def parse1(
        self,
        options: Options,
        arguments: typing.List[typing.List[str]],
        stopAtNonOption: bool,
    ) -> CommandLine:
        raise NotImplementedError

    def parse0(self, options: Options, arguments: typing.List[typing.List[str]]) -> CommandLine:
        pass
