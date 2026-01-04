from __future__ import annotations
import copy
import re
from io import StringIO
import io
from io import BytesIO
import typing
from typing import *
import os
from src.main.org.apache.commons.fileupload.FileUploadBase import *
from src.main.org.apache.commons.fileupload.FileItemStream import *
from src.main.org.apache.commons.fileupload.util.Closeable import *
from src.main.org.apache.commons.fileupload.ProgressListener import *


class MultipartStream:

    _BOUNDARY_PREFIX: typing.List[int] = None  # LLM could not translate this field

    _STREAM_TERMINATOR: typing.List[int] = None  # LLM could not translate this field

    _HEADER_SEPARATOR: typing.List[int] = None  # LLM could not translate this field

    _DEFAULT_BUFSIZE: int = 4096
    HEADER_PART_SIZE_MAX: int = 10240
    DASH: int = 0x2D
    LF: int = 0x0A
    CR: int = 0x0D
    __notifier: ProgressNotifier = None

    __headerEncoding: str = ""

    __tail: int = 0

    __head: int = 0

    __buffer: typing.List[int] = None

    __bufSize: int = 0

    __boundaryTable: typing.List[int] = None

    __boundary: typing.List[int] = None

    __keepRegion: int = 0

    __boundaryLength: int = 0

    __input: typing.Union[io.BytesIO, io.StringIO, io.BufferedReader] = None

    _FIELD_SEPARATOR: typing.List[int] = [CR, LF]

    @staticmethod
    def MultipartStream3(
        input_: typing.Union[io.BytesIO, io.StringIO, io.BufferedReader],
        boundary: typing.List[int],
    ) -> MultipartStream:
        return MultipartStream(input_, boundary, MultipartStream._DEFAULT_BUFSIZE, None)

    @staticmethod
    def MultipartStream1(
        input_: typing.Union[io.BytesIO, io.StringIO, io.BufferedReader],
        boundary: typing.List[int],
        bufSize: int,
    ) -> MultipartStream:
        return MultipartStream(input_, boundary, bufSize, None)

    @staticmethod
    def MultipartStream0() -> MultipartStream:
        return MultipartStream.MultipartStream2(None, None, None)

    def _findSeparator(self) -> int:
        buffer_pos = self.__head
        table_pos = 0

        while buffer_pos < self.__tail:
            while table_pos >= 0 and self.__buffer[buffer_pos] != self.__boundary[table_pos]:
                table_pos = self.__boundaryTable[table_pos]
            buffer_pos += 1
            table_pos += 1
            if table_pos == self.__boundaryLength:
                return buffer_pos - self.__boundaryLength
        return -1

    def _findByte(self, value: int, pos: int) -> int:
        for i in range(pos, self.__tail):
            if self.__buffer[i] == value:
                return i

        return -1

    @staticmethod
    def arrayequals(a: typing.List[int], b: typing.List[int], count: int) -> bool:
        for i in range(count):
            if a[i] != b[i]:
                return False
        return True

    def readHeaders(self) -> str:
        i = 0
        baos = BytesIO()
        size = 0
        while i < len(self._HEADER_SEPARATOR):
            try:
                b = self.readByte()
            except FileUploadOSError as e:
                raise e
            except IOError as e:
                raise MalformedStreamException("Stream ended unexpectedly")
            size += 1
            if size > self.HEADER_PART_SIZE_MAX:
                raise MalformedStreamException(
                    f"Header section has more than {self.HEADER_PART_SIZE_MAX} bytes (maybe it is not properly terminated)"
                )
            if b == self._HEADER_SEPARATOR[i]:
                i += 1
            else:
                i = 0
            baos.write(bytes([b]))

        headers = None
        if self.__headerEncoding is not None:
            try:
                headers = baos.getvalue().decode(self.__headerEncoding)
            except (UnicodeDecodeError, LookupError):
                headers = baos.getvalue().decode()
        else:
            headers = baos.getvalue().decode()

        return headers

    def setBoundary(self, boundary: typing.List[int]) -> None:
        if len(boundary) != self.__boundaryLength - len(MultipartStream._BOUNDARY_PREFIX):
            raise IllegalBoundaryException("The length of a boundary token cannot be changed")
        self.__boundary[
            len(MultipartStream._BOUNDARY_PREFIX) : len(MultipartStream._BOUNDARY_PREFIX) + len(boundary)
        ] = boundary
        self.__computeBoundaryTable()

    def readBoundary(self) -> bool:
        marker = [0, 0]
        nextChunk = False

        self.__head += self.__boundaryLength
        try:
            marker[0] = self.readByte()
            if marker[0] == MultipartStream.LF:
                return True

            marker[1] = self.readByte()
            if MultipartStream.arrayequals(marker, MultipartStream._STREAM_TERMINATOR, 2):
                nextChunk = False
            elif MultipartStream.arrayequals(marker, MultipartStream._FIELD_SEPARATOR, 2):
                nextChunk = True
            else:
                raise MalformedStreamException("Unexpected characters follow a boundary")
        except FileUploadOSError as e:
            raise e
        except IOError as e:
            raise MalformedStreamException("Stream ended unexpectedly")
        return nextChunk

    def readByte(self) -> int:
        if self.__head == self.__tail:
            self.__head = 0
            data = self.__input.read(self.__bufSize)
            if not data:
                raise IOError("No more data is available")
            self.__tail = len(data)
            self.__buffer[self.__head : self.__tail] = data
            if self.__notifier is not None:
                self.__notifier.noteBytesRead(self.__tail)
        result = self.__buffer[self.__head]
        self.__head += 1
        return result

    def setHeaderEncoding(self, encoding: str) -> None:
        self.__headerEncoding = encoding

    def getHeaderEncoding(self) -> str:
        return self.__headerEncoding

    @staticmethod
    def MultipartStream2(
        input_: typing.Union[io.BytesIO, io.StringIO, io.BufferedReader],
        boundary: typing.List[int],
        pNotifier: ProgressNotifier,
    ) -> MultipartStream:
        return MultipartStream(input_, boundary, MultipartStream._DEFAULT_BUFSIZE, pNotifier)

    def __init__(
        self,
        input_: typing.Union[io.BytesIO, io.StringIO, io.BufferedReader],
        boundary: typing.List[int],
        bufSize: int,
        pNotifier: ProgressNotifier,
    ) -> None:
        if boundary is None:
            raise ValueError("boundary may not be null")

        self.__boundaryLength = len(boundary) + len(MultipartStream._BOUNDARY_PREFIX)
        if bufSize < self.__boundaryLength + 1:
            raise ValueError("The buffer size specified for the MultipartStream is too small")

        self.__input = input_
        self.__bufSize = max(bufSize, self.__boundaryLength * 2)
        self.__buffer = [0] * self.__bufSize
        self.__notifier = pNotifier

        self.__boundary = [0] * self.__boundaryLength
        self.__boundaryTable = [0] * (self.__boundaryLength + 1)
        self.__keepRegion = len(self.__boundary)

        # System.arraycopy equivalent
        self.__boundary[0 : len(MultipartStream._BOUNDARY_PREFIX)] = MultipartStream._BOUNDARY_PREFIX
        self.__boundary[
            len(MultipartStream._BOUNDARY_PREFIX) : len(MultipartStream._BOUNDARY_PREFIX) + len(boundary)
        ] = boundary

        self.__computeBoundaryTable()

        self.__head = 0
        self.__tail = 0

    def __computeBoundaryTable(self) -> None:
        position = 2
        candidate = 0

        self.__boundaryTable[0] = -1
        self.__boundaryTable[1] = 0

        while position <= self.__boundaryLength:
            if self.__boundary[position - 1] == self.__boundary[candidate]:
                self.__boundaryTable[position] = candidate + 1
                candidate += 1
                position += 1
            elif candidate > 0:
                candidate = self.__boundaryTable[candidate]
            else:
                self.__boundaryTable[position] = 0
                position += 1

    def newInputStream(self) -> ItemInputStream:
        return ItemInputStream()


class ProgressNotifier:

    __items: int = 0

    __bytesRead: int = 0

    __contentLength: int = 0

    __listener: ProgressListener = None

    def __notifyListener(self) -> None:
        if self.__listener is not None:
            self.__listener.update(self.__bytesRead, self.__contentLength, self.__items)

    def noteItem(self) -> None:
        self.__items += 1
        self.__notifyListener()

    def noteBytesRead(self, pBytes: int) -> None:
        """Indicates that the given number of bytes have been read from
        the input stream.
        """
        self.__bytesRead += pBytes
        self.__notifyListener()

    def __init__(self, pListener: ProgressListener, pContentLength: int) -> None:
        self.__listener = pListener
        self.__contentLength = pContentLength


class ItemInputStream:

    __BYTE_POSITIVE_OFFSET: int = 256
    __closed: bool = False

    __pos: int = 0

    __pad: int = 0

    __total: int = 0

    def isClosed(self) -> bool:
        return self.__closed

    def skip(self, bytes_: int) -> int:
        if self.__closed:
            raise ItemSkippedException()
        av = self.available()
        if av == 0:
            av = self.__makeAvailable()
            if av == 0:
                return 0
        res = min(av, bytes_)
        self.__outer._MultipartStream__head += res
        return res

    def read(self) -> int:
        return self.read0()

    def available(self) -> int:
        if self.__pos == -1:
            return self.__tail - self.__head - self.__pad
        return self.__pos - self.__head

    def close1(self, pCloseUnderlying: bool) -> None:
        if self.__closed:
            return
        if pCloseUnderlying:
            self.__closed = True
            self.__outer._MultipartStream__input.close()
        else:
            while True:
                av = self.available()
                if av == 0:
                    av = self.__makeAvailable()
                    if av == 0:
                        break
                self.skip(av)
        self.__closed = True

    def close0(self) -> None:
        self.close1(False)

    def read1(self, b: typing.List[int], off: int, len_: int) -> int:
        if self.__closed:
            raise ItemSkippedException()
        if len_ == 0:
            return 0
        res = self.available()
        if res == 0:
            res = self.__makeAvailable()
            if res == 0:
                return -1
        res = min(res, len_)
        b[off : off + res] = self.__outer._MultipartStream__buffer[
            self.__outer._MultipartStream__head : self.__outer._MultipartStream__head + res
        ]
        self.__outer._MultipartStream__head += res
        self.__total += res
        return res

    def read0(self) -> int:
        if self.__closed:
            raise ItemSkippedException()
        if self.available() == 0 and self.__makeAvailable() == 0:
            return -1
        self.__total += 1
        b = self.__outer._MultipartStream__buffer[self.__outer._MultipartStream__head]
        self.__outer._MultipartStream__head += 1
        if b >= 0:
            return b
        return b + self.__BYTE_POSITIVE_OFFSET

    def getBytesRead(self) -> int:
        return self.__total

    def __makeAvailable(self) -> int:
        if self.__pos != -1:
            return 0

        self.__total += self.__outer._MultipartStream__tail - self.__outer._MultipartStream__head - self.__pad
        self.__outer._MultipartStream__buffer[0 : self.__pad] = self.__outer._MultipartStream__buffer[
            self.__outer._MultipartStream__tail - self.__pad : self.__outer._MultipartStream__tail
        ]

        self.__outer._MultipartStream__head = 0
        self.__outer._MultipartStream__tail = self.__pad

        while True:
            bytesRead = self.__outer._MultipartStream__input.read(
                self.__outer._MultipartStream__bufSize - self.__outer._MultipartStream__tail
            )
            if bytesRead is None or len(bytesRead) == 0:
                msg = "Stream ended unexpectedly"
                raise MalformedStreamException(msg)

            bytesReadLen = len(bytesRead)
            if self.__outer._MultipartStream__notifier is not None:
                self.__outer._MultipartStream__notifier.noteBytesRead(bytesReadLen)

            self.__outer._MultipartStream__buffer[
                self.__outer._MultipartStream__tail : self.__outer._MultipartStream__tail + bytesReadLen
            ] = bytesRead
            self.__outer._MultipartStream__tail += bytesReadLen

            self.__findSeparator()
            av = self.available()

            if av > 0 or self.__pos != -1:
                return av

    def __findSeparator(self) -> None:
        self.__pos = self.__outer._findSeparator()
        if self.__pos == -1:
            if (
                self.__outer._MultipartStream__tail - self.__outer._MultipartStream__head
                > self.__outer._MultipartStream__keepRegion
            ):
                self.__pad = self.__outer._MultipartStream__keepRegion
            else:
                self.__pad = self.__outer._MultipartStream__head - self.__outer._MultipartStream__head

    def __init__(self) -> None:
        self.__findSeparator()


class MalformedStreamException:

    __serialVersionUID: int = 6466926458059796677

    def __init__(self, message: str) -> None:
        super().__init__(message)


class IllegalBoundaryException:

    __serialVersionUID: int = -161533165102632918

    def __init__(self, message: str) -> None:
        super().__init__(message)
