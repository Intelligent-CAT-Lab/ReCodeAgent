from __future__ import annotations
import time
import re
import os
import io
import typing
from typing import *


class Flags:

    __flags: int = 0
    __serialVersionUID: int = 8481587558770237995

    def toString(self) -> str:
        bin_str = bin(self.__flags)[
            2:
        ]  # Convert to binary string and remove '0b' prefix
        padded_bin_str = bin_str.zfill(64)  # Pad with leading zeros to make it 64 bits
        return padded_bin_str

    def hashCode(self) -> int:
        return int(self.__flags)

    def equals(self, obj: typing.Any) -> bool:
        if not isinstance(obj, Flags):
            return False

        if obj is self:
            return True

        return self.__flags == obj.__flags

    def clone(self) -> typing.Any:
        try:
            import copy

            return copy.deepcopy(self)
        except Exception as e:
            raise RuntimeError("Couldn't clone Flags object.") from e

    def turnOnAll(self) -> None:
        self.__flags = 0xFFFFFFFFFFFFFFFF

    def clear(self) -> None:
        self.__flags = 0

    def turnOffAll(self) -> None:
        self.__flags = 0

    def turnOff(self, flag: int) -> None:
        self.__flags &= ~flag

    def turnOn(self, flag: int) -> None:
        self.__flags |= flag

    def isOff(self, flag: int) -> bool:
        return (self.__flags & flag) == 0

    def isOn(self, flag: int) -> bool:
        return (self.__flags & flag) == flag

    def getFlags(self) -> int:
        return self.__flags

    def __init__(self, constructorId: int, flags: int) -> None:
        if constructorId == 1:
            self.__flags = flags
