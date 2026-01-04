from __future__ import annotations
import re
from io import IOBase
import io
import numbers
import typing
from typing import *
import os
from src.main.org.apache.commons.csv.Constants import *


class ExtendedBufferedReader(io.BufferedReader):

    __closed: bool = False

    __position: int = 0

    __eolCounter: int = 0

    __lastChar: int = Constants.UNDEFINED

    def readLine(self) -> str:
        if self.lookAhead0() == Constants.END_OF_STREAM:
            return None
        buffer = []
        while True:
            current = self.read0()
            if current == ord(Constants.CR):
                next_char = self.lookAhead0()
                if next_char == ord(Constants.LF):
                    self.read0()
            if current == Constants.END_OF_STREAM or current == ord(Constants.LF) or current == ord(Constants.CR):
                break
            buffer.append(chr(current))
        return "".join(buffer)

    def close(self) -> None:
        self.__closed = True
        self.__lastChar = Constants.END_OF_STREAM
        if hasattr(super(), "close"):
            super().close()

    def read1(self, buf: typing.List[str], offset: int, length: int) -> int:
        if length == 0:
            return 0

        # Read from the underlying buffer
        data = super().read(length)

        if data is None or len(data) == 0:
            len_read = -1
            self.__lastChar = Constants.END_OF_STREAM
        else:
            len_read = len(data)

            # Decode bytes to string
            text = data.decode("utf-8", errors="replace")

            # Copy characters into buf at the specified offset
            for i in range(len_read):
                buf[offset + i] = text[i]

            # Process each character for EOL counting
            for i in range(offset, offset + len_read):
                ch = buf[i]
                if ch == Constants.LF:
                    prev_char = (
                        buf[i - 1]
                        if i > offset
                        else (chr(self.__lastChar) if self.__lastChar != Constants.UNDEFINED else None)
                    )
                    if Constants.CR != prev_char:
                        self.__eolCounter += 1
                elif ch == Constants.CR:
                    self.__eolCounter += 1

            self.__lastChar = ord(buf[offset + len_read - 1])
            self.__position += len_read

        return len_read

    def read0(self) -> int:
        data = super().read(1)
        if data:
            current = data[0] if isinstance(data, bytes) else ord(data)
        else:
            current = Constants.END_OF_STREAM

        if (
            current == ord(Constants.CR)
            or (current == ord(Constants.LF) and self.__lastChar != ord(Constants.CR))
            or (
                current == Constants.END_OF_STREAM
                and self.__lastChar != ord(Constants.CR)
                and self.__lastChar != ord(Constants.LF)
                and self.__lastChar != Constants.END_OF_STREAM
            )
        ):
            self.__eolCounter += 1

        self.__lastChar = current
        self.__position += 1
        return self.__lastChar

    def isClosed(self) -> bool:
        return self.__closed

    def lookAhead2(self, n: int) -> typing.List[str]:
        buf = [""] * n
        return self.lookAhead1(buf)

    def lookAhead1(self, buf: typing.List[str]) -> typing.List[str]:
        n = len(buf)
        current_pos = self.tell()

        # Read n characters
        data = self.read(n)

        # Reset to original position
        self.seek(current_pos)

        # Fill buf with the characters read
        for i in range(len(data)):
            buf[i] = data[i]

        return buf

    def lookAhead0(self) -> int:
        data = self.peek(1)
        if not data or len(data) == 0:
            return -1
        return data[0]

    def getPosition(self) -> int:
        return self.__position

    def getLastChar(self) -> int:
        return self.__lastChar

    def getCurrentLineNumber(self) -> int:
        if (
            self.__lastChar == ord(Constants.CR)
            or self.__lastChar == ord(Constants.LF)
            or self.__lastChar == Constants.UNDEFINED
            or self.__lastChar == Constants.END_OF_STREAM
        ):
            return self.__eolCounter
        return self.__eolCounter + 1

    def __init__(self, reader: typing.Union[io.TextIOWrapper, io.BufferedReader, io.TextIOBase]) -> None:
        if isinstance(reader, io.BufferedReader):
            super().__init__(reader.raw)
        elif hasattr(reader, "buffer"):
            super().__init__(reader.buffer)
        else:
            super().__init__(reader)
