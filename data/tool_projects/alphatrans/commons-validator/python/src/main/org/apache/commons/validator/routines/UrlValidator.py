from __future__ import annotations
import re
import urllib
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

    ALLOW_LOCAL_URLS: int = 1 << 3
    NO_FRAGMENTS: int = 1 << 2
    ALLOW_2_SLASHES: int = 1 << 1
    ALLOW_ALL_SCHEMES: int = 1 << 0
    __domainValidator: DomainValidator = None

    __DEFAULT_URL_VALIDATOR: UrlValidator = None
    __DEFAULT_SCHEMES: List[str] = ["http", "https", "ftp"]
    __authorityValidator: RegexValidator = None

    __allowedSchemes: typing.Set[str] = None

    __options: int = 0

    __QUERY_PATTERN: re.Pattern = re.compile(r"^(\\S*)$")
    __QUERY_REGEX: str = r"^(\\S*)$"
    __PATH_REGEX: str = r"^(/[-\w:@&?=+,.!/~*'%$_;\(\)]*)?$"
    __PARSE_AUTHORITY_EXTRA: int = 4
    __PARSE_AUTHORITY_PORT: int = 3
    __PARSE_AUTHORITY_HOST_IP: int = 2
    __PARSE_AUTHORITY_IPV6: int = 1
    __AUTHORITY_REGEX: str = (
        r"(?:\[(::FFFF:(?:\d{1,3}\.){3}\d{1,3}|[0-9a-fA-F:]+)\]|(?:(?:USERINFO_CHARS_REGEX\+)+(?::USERINFO_CHARS_REGEX\*)?@)?([\p{Alnum}\-\.]*))(?::(\d*))?(.*)?"
    )
    __USERINFO_CHARS_REGEX: str = "[a-zA-Z0-9%-._~!$&'()*+,;=]"
    __IPV6_REGEX: str = r"::FFFF:(?:\d{1,3}\.){3}\d{1,3}|[0-9a-fA-F:]+"
    __AUTHORITY_CHARS_REGEX: str = r"\p{Alnum}\-\."
    __SCHEME_PATTERN: re.Pattern = re.compile(r"^[A-Za-z][A-Za-z0-9+\-\.]*")
    __SCHEME_REGEX: str = r"^\p{Alpha}[\p{Alnum}\+\-\.\]*"
    __MAX_UNSIGNED_16_BIT_INT: int = 0xFFFF
    __serialVersionUID: int = 7557161713937335013

    __AUTHORITY_PATTERN: re.Pattern = None  # LLM could not translate this field

    @staticmethod
    def initialize_fields() -> None:
        UrlValidator.__DEFAULT_URL_VALIDATOR: UrlValidator = (
            UrlValidator.UrlValidator6()
        )

    def _countToken(self, token: str, target: str) -> int:
        token_index = 0
        count = 0
        while token_index != -1:
            token_index = target.find(token, token_index)
            if token_index != -1:
                token_index += len(token)
                count += 1
        return count

    def _isValidFragment(self, fragment: str) -> bool:
        if fragment is None:
            return True

        return self.__isOff(self.NO_FRAGMENTS)

    def _isValidQuery(self, query: str) -> bool:
        if query is None:
            return True

        return self.__QUERY_PATTERN.match(query) is not None

    def _isValidPath(self, path: str) -> bool:
        if path is None:
            return False

        if not self.__PATH_PATTERN.match(path):
            return False

        try:
            norm = pathlib.PurePosixPath(path).as_posix()
            if norm.startswith("/../") or norm == "/..":
                return False
        except Exception:
            return False

        slash2_count = self._countToken("//", path)
        if self.__isOff(self.ALLOW_2_SLASHES) and (slash2_count > 0):
            return False

        return True

    def _isValidAuthority(self, authority: str) -> bool:
        if authority is None:
            return False

        if self.__authorityValidator is not None and self.__authorityValidator.isValid(
            authority
        ):
            return True

        authority_ascii = DomainValidator.unicodeToASCII(authority)

        authority_matcher = self.__AUTHORITY_PATTERN.match(authority_ascii)
        if not authority_matcher:
            return False

        ipv6 = authority_matcher.group(self.__PARSE_AUTHORITY_IPV6)
        if ipv6 is not None:
            inet_address_validator = InetAddressValidator.getInstance()
            if not inet_address_validator.isValidInet6Address(ipv6):
                return False
        else:
            host_location = authority_matcher.group(self.__PARSE_AUTHORITY_HOST_IP)
            if not self.__domainValidator.isValid(host_location):
                inet_address_validator = InetAddressValidator.getInstance()
                if not inet_address_validator.isValidInet4Address(host_location):
                    return False

            port = authority_matcher.group(self.__PARSE_AUTHORITY_PORT)
            if port is not None and len(port) > 0:
                try:
                    i_port = int(port)
                    if i_port < 0 or i_port > self.__MAX_UNSIGNED_16_BIT_INT:
                        return False
                except ValueError:
                    return False  # this can happen for big numbers

        extra = authority_matcher.group(self.__PARSE_AUTHORITY_EXTRA)
        if extra is not None and extra.strip():
            return False

        return True

    def _isValidScheme(self, scheme: str) -> bool:
        if scheme is None:
            return False

        if not self.__SCHEME_PATTERN.match(scheme):
            return False

        if (
            self.__isOff(self.ALLOW_ALL_SCHEMES)
            and scheme.lower() not in self.__allowedSchemes
        ):
            return False

        return True

    def isValid(self, value: str) -> bool:
        if value is None:
            return False

        # Ensure value is a valid URI
        try:
            uri = urllib.parse.urlparse(value)
        except Exception:
            return False

        scheme = uri.scheme
        if not self._isValidScheme(scheme):
            return False

        authority = uri.netloc
        if scheme == "file" and (authority is None or authority == ""):
            # Special case - file: allows an empty authority
            return True  # This is a local file - nothing more to do here
        elif scheme == "file" and authority is not None and ":" in authority:
            return False
        else:
            if not self._isValidAuthority(authority):
                return False

        if not self._isValidPath(uri.path):
            return False

        if not self._isValidQuery(uri.query):
            return False

        if not self._isValidFragment(uri.fragment):
            return False

        return True

    @staticmethod
    def UrlValidator6() -> UrlValidator:
        return UrlValidator.UrlValidator5(None)

    @staticmethod
    def UrlValidator5(schemes: Optional[List[str]]) -> UrlValidator:
        return UrlValidator.UrlValidator3(schemes, 0)

    @staticmethod
    def UrlValidator4(options: int) -> UrlValidator:
        return UrlValidator.UrlValidator1(None, None, options)

    @staticmethod
    def UrlValidator3(schemes: Optional[List[str]], options: int) -> UrlValidator:
        return UrlValidator.UrlValidator1(schemes, None, options)

    @staticmethod
    def UrlValidator2(authorityValidator: RegexValidator, options: int) -> UrlValidator:
        return UrlValidator.UrlValidator1(None, authorityValidator, options)

    @staticmethod
    def UrlValidator1(
        schemes: Optional[List[str]],
        authorityValidator: Optional[RegexValidator],
        options: int,
    ) -> UrlValidator:
        return UrlValidator(
            schemes,
            authorityValidator,
            options,
            DomainValidator.getInstance1(
                UrlValidator.__isOn1(UrlValidator.ALLOW_LOCAL_URLS, options)
            ),
        )

    def __init__(
        self,
        schemes: Optional[List[str]],
        authorityValidator: Optional[RegexValidator],
        options: int,
        domainValidator: DomainValidator,
    ) -> None:
        self.__options = options

        if domainValidator is None:
            raise ValueError("DomainValidator must not be null")

        if domainValidator.isAllowLocal() != ((options & self.ALLOW_LOCAL_URLS) > 0):
            raise ValueError("DomainValidator disagrees with ALLOW_LOCAL_URLS setting")

        self.__domainValidator = domainValidator

        if self.__isOn0(self.ALLOW_ALL_SCHEMES):
            self.__allowedSchemes = set()
        else:
            if schemes is None:
                schemes = self.__DEFAULT_SCHEMES
            self.__allowedSchemes = {scheme.lower() for scheme in schemes}

        self.__authorityValidator = authorityValidator

    @staticmethod
    def getInstance() -> UrlValidator:
        return UrlValidator.__DEFAULT_URL_VALIDATOR

    def __isOff(self, flag: int) -> bool:
        return (self.__options & flag) == 0

    @staticmethod
    def __isOn1(flag: int, options: int) -> bool:
        return (options & flag) > 0

    def __isOn0(self, flag: int) -> bool:
        return (self.__options & flag) > 0


UrlValidator.initialize_fields()
