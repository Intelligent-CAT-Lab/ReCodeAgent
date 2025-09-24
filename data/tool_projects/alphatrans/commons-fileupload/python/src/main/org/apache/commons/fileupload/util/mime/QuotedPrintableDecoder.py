from __future__ import annotations
import re
from io import StringIO
import io
from io import BytesIO
import typing
from typing import *


class QuotedPrintableDecoder:

    __UPPER_NIBBLE_SHIFT: int = 8 // 2

    @staticmethod
    def decode(
        data: typing.List[int],
        out: typing.Union[io.BytesIO, io.StringIO, io.BufferedWriter],
    ) -> int:
        off = 0
        length = len(data)
        end_offset = off + length
        bytes_written = 0

        while off < end_offset:
            ch = data[off]
            off += 1

            if ch == ord("_"):
                out.write(b" ")
            elif ch == ord("="):
                if off + 1 >= end_offset:
                    raise IOError(
                        "Invalid quoted printable encoding; truncated escape sequence"
                    )

                b1 = data[off]
                off += 1
                b2 = data[off]
                off += 1

                if b1 == ord("\r"):
                    if b2 != ord("\n"):
                        raise IOError(
                            "Invalid quoted printable encoding; CR must be followed by LF"
                        )
                else:
                    c1 = QuotedPrintableDecoder.__hexToBinary(b1)
                    c2 = QuotedPrintableDecoder.__hexToBinary(b2)
                    out.write(
                        bytes(
                            [(c1 << QuotedPrintableDecoder.__UPPER_NIBBLE_SHIFT) | c2]
                        )
                    )
                    bytes_written += 1
            else:
                out.write(bytes([ch]))
                bytes_written += 1

        return bytes_written

    @staticmethod
    def __hexToBinary(b: int) -> int:
        i = int(chr(b), 16) if chr(b).isdigit() or chr(b).lower() in "abcdef" else -1
        if i == -1:
            raise IOError(
                f"Invalid quoted printable encoding: not a valid hex digit: {b}"
            )
        return i

    def __init__(self) -> None:
        pass
