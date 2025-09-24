from __future__ import annotations
import time
import re
import io
import typing
from typing import *
import os


class Arg:

    _resource: bool = True
    _position: int = -1
    _name: str = None
    _key: str = None
    _bundle: str = None
    __serialVersionUID: int = -8922606779669839294

    def toString(self) -> str:
        results = []
        results.append("Arg: name=")
        results.append(self._name)
        results.append("  key=")
        results.append(self._key)
        results.append("  position=")
        results.append(str(self._position))
        results.append("  bundle=")
        results.append(self._bundle)
        results.append("  resource=")
        results.append(str(self._resource))
        results.append("\n")
        return "".join(results)

    def clone(self) -> typing.Any:
        try:
            return super().clone()
        except AttributeError as e:  # Python does not have NotImplementedError
            raise RuntimeError(str(e))

    def setResource(self, resource: bool) -> None:
        self._resource = resource

    def setPosition(self, position: int) -> None:
        self._position = position

    def setName(self, name: str) -> None:
        self._name = name

    def setKey(self, key: str) -> None:
        self._key = key

    def setBundle(self, bundle: str) -> None:
        self._bundle = bundle

    def isResource(self) -> bool:
        return self._resource

    def getPosition(self) -> int:
        return self._position

    def getName(self) -> str:
        return self._name

    def getKey(self) -> str:
        return self._key

    def getBundle(self) -> str:
        return self._bundle
