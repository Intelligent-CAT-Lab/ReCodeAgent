from __future__ import annotations
import re
import io
import numbers
import enum
import typing
from typing import *
import os
from src.main.org.apache.commons.csv.CSVParser import *
from src.main.org.apache.commons.csv.Constants import *


class CSVRecord:

    __parser: CSVParser = None

    __values: typing.List[typing.List[str]] = None

    __recordNumber: int = 0

    __comment: str = ""

    __characterPosition: int = 0

    __serialVersionUID: int = 1

    def toString(self) -> str:
        return (
            f"CSVRecord [comment='{self.__comment}', "
            f"recordNumber={self.__recordNumber}, "
            f"values={self.__values}]"
        )

    def iterator(self) -> typing.Iterator[str]:
        return iter(self.toList())

    def values(self) -> typing.List[str]:
        return self.__values

    def toMap(self) -> typing.Dict[str, str]:
        return self.putIn(dict())

    def toList(self) -> typing.List[str]:
        return list(self.stream())

    def stream(self) -> typing.Iterable[str]:
        return iter(self.__values)

    def size(self) -> int:
        return len(self._CSVRecord__values)

    def putIn(self, map_: typing.Any) -> typing.Any:
        header_map = self.__getHeaderMapRaw()
        if header_map is None:
            return map_

        for key, value in header_map.items():
            if value < len(self.__values):
                map_[key] = self.__values[value]

        return map_

    def isSet1(self, name: str) -> bool:

        pass  # LLM could not translate this method

    def isSet0(self, index: int) -> bool:

        pass  # LLM could not translate this method

    def isMapped(self, name: str) -> bool:
        header_map = self.__getHeaderMapRaw()
        return header_map is not None and name in header_map

    def isConsistent(self) -> bool:
        header_map = self.__getHeaderMapRaw()
        return header_map is None or (self.__values is not None and len(header_map) == len(self.__values))

    def hasComment(self) -> bool:
        return self.__comment is not None

    def getRecordNumber(self) -> int:
        return self.__recordNumber

    def getParser(self) -> CSVParser:
        return self._CSVRecord__parser

    def getComment(self) -> str:
        return self._CSVRecord__comment

    def getCharacterPosition(self) -> int:
        return self._CSVRecord__characterPosition

    def get2(self, name: str) -> str:
        header_map = self.__getHeaderMapRaw()
        if header_map is None:
            raise ValueError("No header mapping was specified, the record values can't be accessed by name")

        index = header_map.get(name)
        if index is None:
            raise ValueError(f"Mapping for {name} not found, expected one of {list(header_map.keys())}")

        if self.__values is None:
            raise ValueError("Record values are not initialized")

        try:
            return self.__values[index]
        except IndexError as e:
            raise ValueError(
                f"Index for header '{name}' is {index} but CSVRecord only has {len(self.__values)} values!"
            )

    def get1(self, i: int) -> str:
        return self.__values[i]

    def get0(self, e: enum.Enum) -> str:

        pass  # LLM could not translate this method

    def __getHeaderMapRaw(self) -> typing.Dict[str, int]:

        pass  # LLM could not translate this method

    def __init__(
        self,
        parser: CSVParser,
        values: typing.List[str],
        comment: str,
        recordNumber: int,
        characterPosition: int,
    ) -> None:
        self.__recordNumber = recordNumber
        self.__values = values if values is not None else Constants.EMPTY_STRING_ARRAY
        self.__parser = parser
        self.__comment = comment
        self.__characterPosition = characterPosition
