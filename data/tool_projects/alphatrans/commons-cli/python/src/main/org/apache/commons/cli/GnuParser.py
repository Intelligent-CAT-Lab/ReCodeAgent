from __future__ import annotations
import re
import io
import typing
from typing import *
from src.main.org.apache.commons.cli.Options import *
from src.main.org.apache.commons.cli.Parser import *
from src.main.org.apache.commons.cli.Util import *


class GnuParser(Parser):

    def _flatten(self, options: Options, arguments: typing.List[str], stopAtNonOption: bool) -> typing.List[str]:
        tokens: typing.List[str] = []

        eat_the_rest = False

        i = 0
        while i < len(arguments):
            arg = arguments[i]

            if arg == "--":
                eat_the_rest = True
                tokens.append("--")
            elif arg == "-":
                tokens.append("-")
            elif arg.startswith("-"):
                opt = Util.stripLeadingHyphens(arg)

                if options.hasOption(opt):
                    tokens.append(arg)
                elif "=" in opt and options.hasOption(opt[: opt.index("=")]):
                    tokens.append(arg[: arg.index("=")])  # --foo
                    tokens.append(arg[arg.index("=") + 1 :])  # value
                elif len(arg) >= 2 and options.hasOption(arg[:2]):
                    tokens.append(arg[:2])  # -D
                    tokens.append(arg[2:])  # property=value
                else:
                    eat_the_rest = stopAtNonOption
                    tokens.append(arg)
            else:
                tokens.append(arg)

            if eat_the_rest:
                i += 1
                while i < len(arguments):
                    tokens.append(arguments[i])
                    i += 1
            else:
                i += 1

        return tokens
