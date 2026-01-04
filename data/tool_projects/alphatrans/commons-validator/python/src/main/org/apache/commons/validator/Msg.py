from __future__ import annotations
import re
from io import StringIO
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
        results = io.StringIO()

        results.write("Msg: name=")
        results.write(str(self._name))
        results.write("  key=")
        results.write(str(self._key))
        results.write("  resource=")
        results.write(str(self._resource))
        results.write("  bundle=")
        results.write(str(self._bundle))
        results.write("\n")

        return results.getvalue()

    def clone(self) -> typing.Any:
        import copy

        return copy.copy(self)

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
