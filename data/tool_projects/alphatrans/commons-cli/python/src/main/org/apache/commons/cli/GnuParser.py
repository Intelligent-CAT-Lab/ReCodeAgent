from __future__ import annotations
import re
import io
import typing
from typing import *
from src.main.org.apache.commons.cli.Options import *
from src.main.org.apache.commons.cli.Parser import *
from src.main.org.apache.commons.cli.Util import *


class GnuParser(Parser):

    def _flatten(
        self, options: Options, arguments: List[str], stopAtNonOption: bool
    ) -> List[str]:
        tokens: List[str] = []
        eatTheRest = False

        i = 0
        while i < len(arguments):
            arg = arguments[i]

            if arg == "--":
                eatTheRest = True
                tokens.append("--")
            elif arg == "-":
                tokens.append("-")
            elif arg.startswith("-"):
                opt = Util.stripLeadingHyphens(arg)

                if options.hasOption(opt):
                    tokens.append(arg)
                elif "=" in opt and options.hasOption(opt.split("=")[0]):
                    tokens.append(arg[: arg.index("=")])  # --foo
                    tokens.append(arg[arg.index("=") + 1 :])  # value
                elif options.hasOption(arg[:2]):
                    tokens.append(arg[:2])  # -D
                    tokens.append(arg[2:])  # property=value
                else:
                    eatTheRest = stopAtNonOption
                    tokens.append(arg)
            else:
                tokens.append(arg)

            if eatTheRest:
                i += 1
                while i < len(arguments):
                    tokens.append(arguments[i])
                    i += 1
                break

            i += 1

        return tokens
