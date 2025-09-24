from __future__ import annotations
import time
import re
import enum
import threading
import io
import typing
from typing import *
import logging

# from src.main.org.apache.commons.logging.Log import *
# from src.main.org.apache.commons.logging.LogFactory import *
from src.main.org.apache.commons.validator.Validator import *
from src.main.org.apache.commons.validator.ValidatorException import *


class ValidatorAction:

    __methodParameterList: typing.List[str] = []

    __dependencyList: typing.List[str] = []

    __instance: typing.Any = None
    __javascript: str = None
    __jsFunction: str = None
    __jsFunctionName: str = None
    __msg: str = None
    __depends: str = None
    __parameterClasses: typing.List[typing.Type[typing.Any]] = None
    __methodParams: str = (
        (
            Validator.BEAN_PARAM
            + ","
            + Validator.VALIDATOR_ACTION_PARAM
            + ","
            + Validator.FIELD_PARAM
        )
        if Validator.BEAN_PARAM
        and Validator.VALIDATOR_ACTION_PARAM
        and Validator.FIELD_PARAM
        else None
    )
    __validationMethod: Optional[Callable] = None
    __method: str = None
    __validationClass: typing.Type[typing.Any] = None
    __classname: str = None
    __name: str = None
    __log: logging.Logger = logging.getLogger(__name__)
    __serialVersionUID: int = 1339713700053204597

    def toString(self) -> str:
        results = f"ValidatorAction: {self.__name}\n"
        return results

    def getDependencyList(self) -> typing.List[str]:
        return list(self.__dependencyList)

    def isDependency(self, validatorName: str) -> bool:
        return validatorName in self.__dependencyList

    def _loadJavascriptFunction(self) -> None:
        if self.__javascriptAlreadyLoaded():
            return

        if self.__getLog().isEnabledFor(logging.DEBUG):
            self.__getLog().debug("  Loading function begun")

        if self.__jsFunction is None:
            self.__jsFunction = self.__generateJsFunction()

        javascriptFileName = self.__formatJavascriptFileName()

        if self.__getLog().isEnabledFor(logging.DEBUG):
            self.__getLog().debug(f"  Loading js function '{javascriptFileName}'")

        self.__javascript = self.__readJavascriptFile(javascriptFileName)

        if self.__getLog().isEnabledFor(logging.DEBUG):
            self.__getLog().debug("  Loading javascript function completed")

    def _init(self) -> None:
        self._loadJavascriptFunction()

    def setJavascript(self, javascript: str) -> None:
        if self.__jsFunction is not None:
            raise RuntimeError(
                "Cannot call setJavascript() after calling setJsFunction()"
            )
        self.__javascript = javascript

    def getJavascript(self) -> str:
        return self.__javascript

    def setJsFunction(self, jsFunction: str) -> None:
        if self.__javascript is not None:
            raise RuntimeError(
                "Cannot call setJsFunction() after calling setJavascript()"
            )

        self.__jsFunction = jsFunction

    def setJsFunctionName(self, jsFunctionName: str) -> None:
        self.__jsFunctionName = jsFunctionName

    def getJsFunctionName(self) -> str:
        return self.__jsFunctionName

    def setMsg(self, msg: str) -> None:
        self.__msg = msg

    def getMsg(self) -> str:
        return self.__msg

    def setDepends(self, depends: str) -> None:
        self.__depends = depends

        self.__dependencyList.clear()

        for depend in (
            depend.strip() for depend in depends.split(",") if depend.strip()
        ):
            self.__dependencyList.append(depend)

    def getDepends(self) -> str:
        return self.__depends

    def setMethodParams(self, methodParams: str) -> None:
        self.__methodParams = methodParams

        self.__methodParameterList.clear()

        for value in methodParams.split(","):
            value = value.strip()
            if value:
                self.__methodParameterList.append(value)

    def getMethodParams(self) -> str:
        return self.__methodParams

    def setMethod(self, method: str) -> None:
        self.__method = method

    def getMethod(self) -> str:
        return self.__method

    def setClassname(self, classname: str) -> None:
        self.__classname = classname

    def getClassname(self) -> str:
        return self.__classname

    def setName(self, name: str) -> None:
        self.__name = name

    def getName(self) -> str:
        return self.__name

    def __getLog(self) -> logging.Logger:
        if self.__log is None:
            self.__log = logging.getLogger(self.__class__.__name__)
        return self.__log

    def __onlyReturnErrors(self, params: typing.Dict[str, typing.Any]) -> bool:
        v: Validator = params.get(Validator.VALIDATOR_PARAM)
        return v.getOnlyReturnErrors()

    def __getClassLoader(self, params: typing.Dict[str, typing.Any]) -> typing.Any:
        v: Validator = params.get(Validator.VALIDATOR_PARAM)
        return v.getClassLoader()

    def __isValid(self, result: typing.Any) -> bool:
        if isinstance(result, bool):
            return result
        return result is not None

    def __getValidationClassInstance(self) -> typing.Any:
        if (
            self.__validationMethod
            and hasattr(self.__validationMethod, "__self__")
            and self.__validationMethod.__self__ is None
        ):
            # If the method is static, set instance to None
            self.__instance = None
        else:
            if self.__instance is None:
                try:
                    # Create a new instance of the validation class
                    self.__instance = self.__validationClass()
                except TypeError as e:
                    msg1 = f"Couldn't create instance of {self.__classname}. {str(e)}"
                    raise ValidatorException(msg1)
                except Exception as e:
                    msg1 = f"Couldn't create instance of {self.__classname}. {str(e)}"
                    raise ValidatorException(msg1)
        return self.__instance

    def __getParameterValues(
        self, params: typing.Dict[str, typing.Any]
    ) -> typing.List[typing.Any]:
        param_value = [None] * len(self.__methodParameterList)

        for i, param_class_name in enumerate(self.__methodParameterList):
            param_value[i] = params.get(param_class_name)

        return param_value

    def __loadParameterClasses(self, loader: typing.Any) -> None:
        if self.__parameterClasses is not None:
            return

        parameterClasses = [None] * len(self.__methodParameterList)

        for i, paramClassName in enumerate(self.__methodParameterList):
            try:
                parameterClasses[i] = loader.loadClass(paramClassName)
            except ClassNotFoundException as e:
                raise ValidatorException(e.args[0])

        self.__parameterClasses = parameterClasses

    def __loadValidationClass(self, loader: typing.Any) -> None:
        if self.__validationClass is not None:
            return

        try:
            self.__validationClass = loader.loadClass(self.__classname)
        except ClassNotFoundException as e:
            raise ValidatorException(str(e))

    def __loadValidationMethod(self) -> None:
        if self.__validationMethod is not None:
            return

        try:
            self.__validationMethod = getattr(self.__validationClass, self.__method)
        except AttributeError as e:
            raise ValidatorException(f"No such validation method: {str(e)}")

    def __generateJsFunction(self) -> str:
        js_name = "org.apache.commons.validator.javascript"
        js_name += ".validate"
        js_name += self.__name[0].upper() + self.__name[1:]
        return js_name

    def __javascriptAlreadyLoaded(self) -> bool:
        return self.__javascript is not None

    def __formatJavascriptFileName(self) -> str:
        fname = self.__jsFunction[1:]

        if not self.__jsFunction.startswith("/"):
            fname = self.__jsFunction.replace(".", "/") + ".js"

        return fname

    def __readJavascriptFile(self, javascriptFileName: str) -> str:
        class_loader = threading.current_thread().__class__.__module__
        if class_loader is None:
            class_loader = self.__class__.__module__

        try:
            with open(javascriptFileName, "r", encoding="utf-8") as file:
                buffer = file.read()
        except FileNotFoundError:
            self.__getLog().debug(
                f"  Unable to read javascript name {javascriptFileName}"
            )
            return None
        except IOError as e:
            self.__getLog().error("Error reading javascript file.", exc_info=e)
            return None

        return buffer if buffer.strip() else None
