from __future__ import annotations
import time
import re
import io
import typing
from typing import *


class Var:

    JSTYPE_REGEXP: str = "regexp"
    JSTYPE_STRING: str = "string"
    JSTYPE_INT: str = "int"
    __bundle: str = None
    __resource: bool = False
    __jsType: str = None
    __value: str = None
    __name: str = None
    __serialVersionUID: int = -684185211548420224

    def toString(self) -> str:
        results = []

        results.append("Var: name=")
        results.append(self.__name)
        results.append("  value=")
        results.append(self.__value)
        results.append("  resource=")
        results.append(str(self.__resource))
        if self.__resource:
            results.append("  bundle=")
            results.append(self.__bundle)
        results.append("  jsType=")
        results.append(self.__jsType)
        results.append("\n")

        return "".join(results)

    def clone(self) -> typing.Any:
        try:
            return super().clone()
        except AttributeError as e:  # Python does not have NotImplementedError
            raise RuntimeError(str(e))

    def setJsType(self, jsType: str) -> None:
        self.__jsType = jsType

    def getJsType(self) -> str:
        return self.__jsType

    def setBundle(self, bundle: str) -> None:
        self.__bundle = bundle

    def getBundle(self) -> str:
        return self.__bundle

    def setResource(self, resource: bool) -> None:
        self.__resource = resource

    def isResource(self) -> bool:
        return self.__resource

    def setValue(self, value: str) -> None:
        self.__value = value

    def getValue(self) -> str:
        return self.__value

    def setName(self, name: str) -> None:
        self.__name = name

    def getName(self) -> str:
        return self.__name

    def __init__(self, constructorId: int, name: str, value: str, jsType: str) -> None:
        if constructorId == 1:
            self.__name = name
            self.__value = value
            self.__jsType = jsType
