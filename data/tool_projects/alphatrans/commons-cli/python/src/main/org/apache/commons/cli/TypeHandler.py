from __future__ import annotations
import time
import re
import pathlib
import io
import numbers
import typing
from typing import *
import datetime
import urllib
from src.main.org.apache.commons.cli.ParseException import *
from src.main.org.apache.commons.cli.PatternOptionBuilder import *


class TypeHandler:

    @staticmethod
    def createValue0(str_: str, clazz: typing.Type[typing.Any]) -> typing.Any:
        if clazz == PatternOptionBuilder.STRING_VALUE:
            return str_
        if clazz == PatternOptionBuilder.OBJECT_VALUE:
            return TypeHandler.createObject(str_)
        if clazz == PatternOptionBuilder.NUMBER_VALUE:
            return TypeHandler.createNumber(str_)
        if clazz == PatternOptionBuilder.DATE_VALUE:
            return TypeHandler.createDate(str_)
        if clazz == PatternOptionBuilder.CLASS_VALUE:
            return TypeHandler.createClass(str_)
        if clazz == PatternOptionBuilder.FILE_VALUE:
            return TypeHandler.createFile(str_)
        if clazz == PatternOptionBuilder.EXISTING_FILE_VALUE:
            return TypeHandler.openFile(str_)
        if clazz == PatternOptionBuilder.FILES_VALUE:
            return TypeHandler.createFiles(str_)
        if clazz == PatternOptionBuilder.URL_VALUE:
            return TypeHandler.createURL(str_)
        raise ParseException(f"Unable to handle the class: {clazz}")

    @staticmethod
    def openFile(str_: str) -> typing.Union[io.FileIO, io.BufferedReader]:
        try:
            return open(str_, "rb")
        except FileNotFoundError as e:
            raise ParseException(f"Unable to find file: {str_}")

    @staticmethod
    def createValue1(str_: str, obj: typing.Any) -> typing.Any:
        return TypeHandler.createValue0(str_, typing.cast(typing.Type[typing.Any], obj))

    @staticmethod
    def createURL(str_: str) -> urllib.parse.ParseResult:
        try:
            return urllib.parse.urlparse(str_)
        except ValueError:
            raise ParseException(f"Unable to parse the URL: {str_}")

    @staticmethod
    def createObject(classname: str) -> typing.Any:
        try:
            # Dynamically import the module and class
            components = classname.split(".")
            module_name = ".".join(components[:-1])
            class_name = components[-1]
            module = __import__(module_name, fromlist=[class_name])
            cls = getattr(module, class_name)
        except (ImportError, AttributeError):
            raise ParseException(f"Unable to find the class: {classname}")

        try:
            # Create an instance of the class
            return cls()
        except Exception as e:
            raise ParseException(
                f"{type(e).__name__}; Unable to create an instance of: {classname}"
            )

    @staticmethod
    def createNumber(str_: str) -> typing.Union[int, float, numbers.Number]:
        try:
            if "." in str_:
                return float(str_)
            return int(str_)
        except ValueError as e:
            raise ParseException(e.args[0])

    @staticmethod
    def createFiles(str_: str) -> typing.List[pathlib.Path]:
        raise NotImplementedError("Not yet implemented")

    @staticmethod
    def createFile(str_: str) -> pathlib.Path:
        return pathlib.Path(str_)

    @staticmethod
    def createDate(str_: str) -> typing.Union[datetime.datetime, datetime.date]:
        raise NotImplementedError("Not yet implemented")

    @staticmethod
    def createClass(classname: str) -> typing.Type[typing.Any]:
        try:
            return __import__(classname)
        except ModuleNotFoundError as e:
            raise ParseException(f"Unable to find the class: {classname}") from e
