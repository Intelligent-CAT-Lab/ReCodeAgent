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
            if current in (
                Constants.END_OF_STREAM,
                ord(Constants.LF),
                ord(Constants.CR),
            ):
                break
            buffer.append(chr(current))
        return "".join(buffer)

    def close(self) -> None:
        self.__closed = True
        self.__lastChar = Constants.END_OF_STREAM
        super().close()

    def read1(self, buf: typing.List[str], offset: int, length: int) -> int:
        if length == 0:
            return 0

        len_read = super().readinto(memoryview(buf)[offset : offset + length])

        if len_read > 0:
            for i in range(offset, offset + len_read):
                ch = buf[i]
                if ch == Constants.LF:
                    if Constants.CR != (buf[i - 1] if i > offset else self.__lastChar):
                        self.__eolCounter += 1
                elif ch == Constants.CR:
                    self.__eolCounter += 1

            self.__lastChar = buf[offset + len_read - 1]

        elif len_read == Constants.END_OF_STREAM:
            self.__lastChar = Constants.END_OF_STREAM

        self.__position += len_read
        return len_read

    def read0(self) -> int:
        current = self.read(1)  # Read a single character
        if not current:  # If no character is read, end of stream
            current = Constants.END_OF_STREAM
        else:
            current = ord(current)  # Convert character to its ASCII value

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
        buf = [""] * n  # Create a list of empty strings with size n
        return self.lookAhead1(buf)

    def lookAhead1(self, buf: typing.List[str]) -> typing.List[str]:
        n = len(buf)
        self.mark(n)
        self.readinto(buf)
        self.reset()
        return buf

    def lookAhead0(self) -> int:
        self.mark(1)
        c = self.read(1)
        self.reset()
        return ord(c) if c else -1

    def getPosition(self) -> int:
        return self.__position

    def getLastChar(self) -> int:
        return self.__lastChar

    def getCurrentLineNumber(self) -> int:
        if (
            self.__lastChar == Constants.CR
            or self.__lastChar == Constants.LF
            or self.__lastChar == Constants.UNDEFINED
            or self.__lastChar == Constants.END_OF_STREAM
        ):
            return self.__eolCounter  # counter is accurate
        return self.__eolCounter + 1  # Allow for counter being incremented only at EOL

    super().__init__(reader)
