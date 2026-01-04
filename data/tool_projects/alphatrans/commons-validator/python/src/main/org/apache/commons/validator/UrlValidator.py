from __future__ import annotations
import re
import os
import pathlib
import io
import typing
from typing import *
from src.main.org.apache.commons.validator.GenericValidator import *
from src.main.org.apache.commons.validator.routines.InetAddressValidator import *
from src.main.org.apache.commons.validator.util.Flags import *
from src.main.org.apache.commons.validator.routines.DomainValidator import *
from src.main.org.apache.commons.validator.routines.RegexValidator import *


class UrlValidator:

    _defaultSchemes: typing.List[str] = ["http", "https", "ftp"]
    NO_FRAGMENTS: int = 1 << 2
    ALLOW_2_SLASHES: int = 1 << 1
    ALLOW_ALL_SCHEMES: int = 1 << 0
    __allowedSchemes: typing.Set[str] = set()
    __options: Flags = None

    __PORT_PATTERN: re.Pattern = re.compile(r"^:(\d{1,5})$")
    __DOMAIN_PATTERN: re.Pattern = None  # LLM could not translate this field

    __LEGAL_ASCII_PATTERN: re.Pattern = re.compile(r"^[\x00-\x7F]+$")
    __QUERY_PATTERN: re.Pattern = re.compile(r"^(.*)$")
    __PATH_PATTERN: re.Pattern = re.compile(r"^(/[-\w:@&?=+,.!/~*'%$_;]*)?$")
    __PARSE_AUTHORITY_EXTRA: int = 3
    __PARSE_AUTHORITY_PORT: int = 2
    __PARSE_AUTHORITY_HOST_IP: int = 1
    __AUTHORITY_REGEX: str = None  # LLM could not translate this field

    __SCHEME_PATTERN: re.Pattern = re.compile(r"^[a-zA-Z][a-zA-Z0-9\+\-\.]*")
    __PARSE_URL_FRAGMENT: int = 9
    __PARSE_URL_QUERY: int = 7
    __PARSE_URL_PATH: int = 5
    __PARSE_URL_AUTHORITY: int = 4
    __PARSE_URL_SCHEME: int = 2
    __URL_PATTERN: re.Pattern = re.compile("^(([^:/?#]+):)?(//([^/?#]*))?([^?#]*)(\\?([^#]*))?(#(.*))?")
    __URL_REGEX: str = "^(([^:/?#]+):)?(//([^/?#]*))?([^?#]*)(\\?([^#]*))?(#(.*))?"
    __AUTHORITY_CHARS_REGEX: str = "a-zA-Z0-9\\-\\."
    __SPECIAL_CHARS: str = ";/@&=,.?:+$"
    __ALPHA_CHARS: str = "a-zA-Z"
    __serialVersionUID: int = 24137157400029593
    __ALPHA_PATTERN: re.Pattern = re.compile(f"^[{__ALPHA_CHARS}]")
    __AUTHORITY_PATTERN: re.Pattern = re.compile(__AUTHORITY_REGEX) if __AUTHORITY_REGEX is not None else None
    __VALID_CHARS: str = "[^\\s" + (";/@&=,.?:+$" if __SPECIAL_CHARS is None else __SPECIAL_CHARS) + "]"

    __ATOM: str = None  # LLM could not translate this field

    __ATOM_PATTERN: re.Pattern = re.compile(f"^({__ATOM}).*?$")

    def _countToken(self, token: str, target: str) -> int:
        token_index = 0
        count = 0
        while token_index != -1:
            token_index = target.find(token, token_index)
            if token_index > -1:
                token_index += 1
                count += 1
        return count

    def _isValidFragment(self, fragment: str) -> bool:
        if fragment is None:
            return True

        return self.__options.isOff(UrlValidator.NO_FRAGMENTS)

    def _isValidQuery(self, query: str) -> bool:
        if query is None:
            return True

        return self.__QUERY_PATTERN.match(query) is not None

    def _isValidPath(self, path: str) -> bool:
        if path is None:
            return False

        if not self.__PATH_PATTERN.match(path):
            return False

        slash2Count = self._countToken("//", path)
        if self.__options.isOff(UrlValidator.ALLOW_2_SLASHES) and (slash2Count > 0):
            return False

        slashCount = self._countToken("/", path)
        dot2Count = self._countToken("..", path)
        if dot2Count > 0 and (slashCount - slash2Count - 1) <= dot2Count:
            return False

        return True

    def _isValidAuthority(self, authority: str) -> bool:
        if authority is None:
            return False

        inetAddressValidator = InetAddressValidator.getInstance()

        authorityMatcher = self.__AUTHORITY_PATTERN.fullmatch(authority)
        if not authorityMatcher:
            return False

        hostname = False
        hostIP = authorityMatcher.group(self.__PARSE_AUTHORITY_HOST_IP)
        ipV4Address = inetAddressValidator.isValid(hostIP)

        if not ipV4Address:
            hostname = self.__DOMAIN_PATTERN.fullmatch(hostIP) is not None

        if hostname:
            chars = list(hostIP)
            size = 1
            for i in range(len(chars)):
                if chars[i] == ".":
                    size += 1
            domainSegment = [None] * size
            match = True
            segmentCount = 0
            segmentLength = 0

            while match:
                atomMatcher = self.__ATOM_PATTERN.fullmatch(hostIP)
                match = atomMatcher is not None
                if match:
                    domainSegment[segmentCount] = atomMatcher.group(1)
                    segmentLength = len(domainSegment[segmentCount]) + 1
                    hostIP = "" if segmentLength >= len(hostIP) else hostIP[segmentLength:]

                    segmentCount += 1

            topLevel = domainSegment[segmentCount - 1]
            if len(topLevel) < 2 or len(topLevel) > 4:
                return False

            if not self.__ALPHA_PATTERN.fullmatch(topLevel[0:1]):
                return False

            if segmentCount < 2:
                return False

        if not hostname and not ipV4Address:
            return False

        port = authorityMatcher.group(self.__PARSE_AUTHORITY_PORT)
        if port is not None and not self.__PORT_PATTERN.fullmatch(port):
            return False

        extra = authorityMatcher.group(self.__PARSE_AUTHORITY_EXTRA)
        if not GenericValidator.isBlankOrNull(extra):
            return False

        return True

    def _isValidScheme(self, scheme: str) -> bool:
        if scheme is None:
            return False

        if not self.__SCHEME_PATTERN.fullmatch(scheme):
            return False

        if self.__options.isOff(UrlValidator.ALLOW_ALL_SCHEMES) and scheme not in self.__allowedSchemes:
            return False

        return True

    def isValid(self, value: str) -> bool:
        if value is None:
            return False

        if not self.__LEGAL_ASCII_PATTERN.fullmatch(value):
            return False

        urlMatcher = self.__URL_PATTERN.match(value)
        if not urlMatcher:
            return False

        if not self._isValidScheme(urlMatcher.group(self.__PARSE_URL_SCHEME)):
            return False

        if not self._isValidAuthority(urlMatcher.group(self.__PARSE_URL_AUTHORITY)):
            return False

        if not self._isValidPath(urlMatcher.group(self.__PARSE_URL_PATH)):
            return False

        if not self._isValidQuery(urlMatcher.group(self.__PARSE_URL_QUERY)):
            return False

        if not self._isValidFragment(urlMatcher.group(self.__PARSE_URL_FRAGMENT)):
            return False

        return True

    @staticmethod
    def UrlValidator3() -> UrlValidator:
        return UrlValidator.UrlValidator2(None)

    @staticmethod
    def UrlValidator2(schemes: typing.List[typing.List[str]]) -> UrlValidator:
        return UrlValidator(schemes, 0)

    @staticmethod
    def UrlValidator1(options: int) -> UrlValidator:
        return UrlValidator(None, options)

    def __init__(self, schemes: typing.List[typing.List[str]], options: int) -> None:
        self.__options = Flags(1, options)

        if self.__options.isOn(UrlValidator.ALLOW_ALL_SCHEMES):
            return

        if schemes is None:
            schemes = self._defaultSchemes

        self.__allowedSchemes.update(schemes)
