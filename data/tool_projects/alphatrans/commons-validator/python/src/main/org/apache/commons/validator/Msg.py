from __future__ import annotations
import time
import re
import io
import typing
from typing import *


class Msg:

    _resource: bool = True
    _name: str = None
    _key: str = None
    _bundle: str = None
    __serialVersionUID: int = 5690015734364127124

    def toString(self) -> str:
        results = []
        results.append("Msg: name=")
        results.append(str(self._name))
        results.append("  key=")
        results.append(str(self._key))
        results.append("  resource=")
        results.append(str(self._resource))
        results.append("  bundle=")
        results.append(str(self._bundle))
        results.append("\n")
        return "".join(results)

    def clone(self) -> typing.Any:
        try:
            return self.__class__.__new__(self.__class__)
        except Exception as e:
            raise RuntimeError(str(e))

    def setResource(self, resource: bool) -> None:
        self._resource = resource

    def isResource(self) -> bool:
        return self._resource

    def setKey(self, key: str) -> None:
        self._key = key

    def getKey(self) -> str:
        return self._key

    def setName(self, name: str) -> None:
        self._name = name

    def getName(self) -> str:
        return self._name

    def setBundle(self, bundle: str) -> None:
        self._bundle = bundle

    def getBundle(self) -> str:
        return self._bundle
