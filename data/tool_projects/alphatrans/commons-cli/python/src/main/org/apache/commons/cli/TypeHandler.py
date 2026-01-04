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
        if clazz is PatternOptionBuilder.STRING_VALUE:
            return str_
        if clazz is PatternOptionBuilder.OBJECT_VALUE:
            return TypeHandler.createObject(str_)
        if clazz is PatternOptionBuilder.NUMBER_VALUE:
            return TypeHandler.createNumber(str_)
        if clazz is PatternOptionBuilder.DATE_VALUE:
            return TypeHandler.createDate(str_)
        if clazz is PatternOptionBuilder.CLASS_VALUE:
            return TypeHandler.createClass(str_)
        if clazz is PatternOptionBuilder.FILE_VALUE:
            return TypeHandler.createFile(str_)
        if clazz is PatternOptionBuilder.EXISTING_FILE_VALUE:
            return TypeHandler.openFile(str_)
        if clazz is PatternOptionBuilder.FILES_VALUE:
            return TypeHandler.createFiles(str_)
        if clazz is PatternOptionBuilder.URL_VALUE:
            return TypeHandler.createURL(str_)
        raise ParseException(f"Unable to handle the class: {clazz}")

    @staticmethod
    def openFile(str_: str) -> typing.Union[io.FileIO, io.BufferedReader]:
        try:
            return open(str_, "rb")
        except FileNotFoundError as e:
            raise ParseException("Unable to find file: " + str_)

    @staticmethod
    def createValue1(str_: str, obj: typing.Any) -> typing.Any:
        return TypeHandler.createValue0(str_, obj)

    @staticmethod
    def createURL(str_: str) -> str:
        try:
            result = urllib.parse.urlparse(str_)
            if not result.scheme or not result.netloc:
                raise ValueError("Invalid URL")
            return str_
        except (ValueError, Exception) as e:
            raise ParseException(f"Unable to parse the URL: {str_}")

    @staticmethod
    def createObject(classname: str) -> typing.Any:
        try:
            cl = eval(classname)
        except (NameError, AttributeError) as e:
            raise ParseException(f"Unable to find the class: {classname}")

        try:
            return cl()
        except Exception as e:
            raise ParseException(f"{type(e).__name__}; Unable to create an instance of: {classname}")

    @staticmethod
    def createNumber(str_: str) -> typing.Union[int, float, numbers.Number]:
        try:
            if str_.find(".") != -1:
                return float(str_)
            return int(str_)
        except ValueError as e:
            raise ParseException(str(e))

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
            # Try to import as a module path (e.g., "package.module.ClassName")
            parts = classname.rsplit(".", 1)
            if len(parts) == 2:
                module_name, class_name = parts
                import importlib

                module = importlib.import_module(module_name)
                return getattr(module, class_name)
            else:
                # Try builtins first
                import builtins

                if hasattr(builtins, classname):
                    return getattr(builtins, classname)
                # If it's a simple name, it might be in the caller's scope
                # We'll try to get it from the calling frame's globals
                import inspect

                frame = inspect.currentframe()
                if frame and frame.f_back and frame.f_back.f_back:
                    caller_globals = frame.f_back.f_back.f_globals
                    if classname in caller_globals:
                        return caller_globals[classname]
                raise ImportError(f"Class not found: {classname}")
        except (ImportError, AttributeError, ValueError) as e:
            raise ParseException(f"Unable to find the class: {classname}")
