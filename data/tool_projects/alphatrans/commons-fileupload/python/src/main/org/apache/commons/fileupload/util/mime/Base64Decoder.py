from __future__ import annotations
import re
import enum
from io import StringIO
import io
from io import BytesIO
import typing
from typing import *


class Base64Decoder:

    __DECODING_TABLE: typing.List[int] = [0] * (127 - (-128) + 1)
    __PADDING: int = ord("=")
    __ENCODING_TABLE: typing.List[int] = [
        ord("A"),
        ord("B"),
        ord("C"),
        ord("D"),
        ord("E"),
        ord("F"),
        ord("G"),
        ord("H"),
        ord("I"),
        ord("J"),
        ord("K"),
        ord("L"),
        ord("M"),
        ord("N"),
        ord("O"),
        ord("P"),
        ord("Q"),
        ord("R"),
        ord("S"),
        ord("T"),
        ord("U"),
        ord("V"),
        ord("W"),
        ord("X"),
        ord("Y"),
        ord("Z"),
        ord("a"),
        ord("b"),
        ord("c"),
        ord("d"),
        ord("e"),
        ord("f"),
        ord("g"),
        ord("h"),
        ord("i"),
        ord("j"),
        ord("k"),
        ord("l"),
        ord("m"),
        ord("n"),
        ord("o"),
        ord("p"),
        ord("q"),
        ord("r"),
        ord("s"),
        ord("t"),
        ord("u"),
        ord("v"),
        ord("w"),
        ord("x"),
        ord("y"),
        ord("z"),
        ord("0"),
        ord("1"),
        ord("2"),
        ord("3"),
        ord("4"),
        ord("5"),
        ord("6"),
        ord("7"),
        ord("8"),
        ord("9"),
        ord("+"),
        ord("/"),
    ]
    __INPUT_BYTES_PER_CHUNK: int = 4
    __MASK_BYTE_UNSIGNED: int = 0xFF
    __PAD_BYTE: int = -2
    __INVALID_BYTE: int = -1  # must be outside range 0-63

    @staticmethod
    def run_static_init():
        # Initialize the DECODING_TABLE with INVALID_BYTE
        for i in range(len(Base64Decoder.__DECODING_TABLE)):
            Base64Decoder.__DECODING_TABLE[i] = Base64Decoder.__INVALID_BYTE

        # Map ENCODING_TABLE values to their indices in DECODING_TABLE
        for i, value in enumerate(Base64Decoder.__ENCODING_TABLE):
            Base64Decoder.__DECODING_TABLE[value] = i

        # Set the PADDING character in DECODING_TABLE to PAD_BYTE
        Base64Decoder.__DECODING_TABLE[Base64Decoder.__PADDING] = (
            Base64Decoder.__PAD_BYTE
        )

    @staticmethod
    def decode(
        data: typing.List[int],
        out: typing.Union[io.BytesIO, io.StringIO, io.BufferedWriter],
    ) -> int:
        out_len = 0
        cache = [0] * Base64Decoder.__INPUT_BYTES_PER_CHUNK
        cached_bytes = 0

        for b in data:
            d = Base64Decoder.__DECODING_TABLE[Base64Decoder.__MASK_BYTE_UNSIGNED & b]
            if d == Base64Decoder.__INVALID_BYTE:
                continue  # Ignore invalid bytes
            cache[cached_bytes] = d
            cached_bytes += 1
            if cached_bytes == Base64Decoder.__INPUT_BYTES_PER_CHUNK:
                b1, b2, b3, b4 = cache
                if b1 == Base64Decoder.__PAD_BYTE or b2 == Base64Decoder.__PAD_BYTE:
                    raise IOError(
                        "Invalid Base64 input: incorrect padding, first two bytes cannot be padding"
                    )
                out.write(
                    bytes([(b1 << 2) | (b2 >> 4)])
                )  # 6 bits of b1 plus 2 bits of b2
                out_len += 1
                if b3 != Base64Decoder.__PAD_BYTE:
                    out.write(
                        bytes(
                            [
                                ((b2 << 4) | (b3 >> 2))
                                & Base64Decoder.__MASK_BYTE_UNSIGNED
                            ]
                        )
                    )  # 4 bits of b2 plus 4 bits of b3
                    out_len += 1
                    if b4 != Base64Decoder.__PAD_BYTE:
                        out.write(
                            bytes(
                                [((b3 << 6) | b4) & Base64Decoder.__MASK_BYTE_UNSIGNED]
                            )
                        )  # 2 bits of b3 plus 6 bits of b4
                        out_len += 1
                elif (
                    b4 != Base64Decoder.__PAD_BYTE
                ):  # if byte 3 is pad, byte 4 must be pad too
                    raise IOError(
                        "Invalid Base64 input: incorrect padding, 4th byte must be padding if 3rd byte is"
                    )
                cached_bytes = 0

        if cached_bytes != 0:
            raise IOError("Invalid Base64 input: truncated")

        return out_len

    def __init__(self) -> None:
        raise NotImplementedError("This class cannot be instantiated")


Base64Decoder.run_static_init()
