from __future__ import annotations
import time
import copy
import re
from io import IOBase
from io import StringIO
import io
import typing
from typing import *


class IOUtils:

    DEFAULT_BUFFER_SIZE: int = 1024 * 4
    __EOF: int = -1

    @staticmethod
    def rethrow(throwable: BaseException) -> RuntimeError:
        raise throwable

    @staticmethod
    def copyLarge1(
        input_: typing.Union[io.TextIOWrapper, io.BufferedReader, io.TextIOBase],
        output: typing.Union[io.TextIOWrapper, io.BufferedWriter, io.TextIOBase],
        buffer: typing.List[str],
    ) -> int:
        count = 0
        n = input_.readinto(buffer)
        while n is not None and n != IOUtils.__EOF:
            output.write("".join(buffer[:n]))
            count += n
            n = input_.readinto(buffer)
        return count

    @staticmethod
    def copyLarge0(
        input_: typing.Union[io.TextIOWrapper, io.BufferedReader, io.TextIOBase],
        output: typing.Union[io.TextIOWrapper, io.BufferedWriter, io.TextIOBase],
    ) -> int:
        return IOUtils.copyLarge1(input_, output, [""] * IOUtils.DEFAULT_BUFFER_SIZE)

    @staticmethod
    def copy1(
        input_: typing.Union[io.TextIOWrapper, io.BufferedReader, io.TextIOBase],
        output: typing.Union[typing.List, io.TextIOBase],
        buffer: typing.Union[str, typing.List[str], io.StringIO],
    ) -> int:
        count = 0

        # Determine buffer size
        if isinstance(buffer, str):
            buffer_size = len(buffer)
        elif isinstance(buffer, list):
            buffer_size = len(buffer)
        elif isinstance(buffer, io.StringIO):
            buffer_size = buffer.getbuffer().nbytes if hasattr(buffer, "getbuffer") else 8192
        else:
            buffer_size = 8192

        while True:
            chunk = input_.read(buffer_size)
            if not chunk:  # EOF
                break
            n = len(chunk)

            # Append to output
            if isinstance(output, list):
                output.append(chunk)
            else:
                output.write(chunk)

            count += n

        return count

    @staticmethod
    def copy0(
        input_: typing.Union[io.TextIOWrapper, io.BufferedReader, io.TextIOBase],
        output: typing.Union[typing.List, io.TextIOBase],
    ) -> int:
        return IOUtils.copy1(input_, output, io.StringIO(" " * IOUtils.DEFAULT_BUFFER_SIZE))

    def __init__(self) -> None:
        raise AssertionError("IOUtils is a utility class and should not be instantiated")
