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

    __DEFAULT_URL_VALIDATOR: UrlValidator = None  # LLM could not translate this field

    __DEFAULT_SCHEMES: typing.List[str] = ["http", "https", "ftp"]
    __authorityValidator: RegexValidator = None

    __allowedSchemes: typing.Set[str] = None

    __options: int = 0

    __QUERY_PATTERN: re.Pattern = re.compile("^(\\S*)$")
    __QUERY_REGEX: str = "^(\\S*)$"
    __PATH_PATTERN: re.Pattern = None  # LLM could not translate this field

    __PATH_REGEX: str = "^(/[-\\w:@&?=+,.!/~*'%$_;\\(\\)]*)?$"
    __PARSE_AUTHORITY_EXTRA: int = 4
    __PARSE_AUTHORITY_PORT: int = 3
    __PARSE_AUTHORITY_HOST_IP: int = 2
    __PARSE_AUTHORITY_IPV6: int = 1
    __AUTHORITY_PATTERN: re.Pattern = None  # LLM could not translate this field

    __USERINFO_FIELD_REGEX: str = None  # LLM could not translate this field

    __USERINFO_CHARS_REGEX: str = "[a-zA-Z0-9%-._~!$&'()*+,;=]"
    __IPV6_REGEX: str = "::FFFF:(?:\\d{1,3}\\.){3}\\d{1,3}|[0-9a-fA-F:]+"
    __AUTHORITY_CHARS_REGEX: str = r"\p{Alnum}\-\."
    __SCHEME_PATTERN: re.Pattern = None  # LLM could not translate this field

    __SCHEME_REGEX: str = "^[a-zA-Z][a-zA-Z0-9+\\-.]*"
    __MAX_UNSIGNED_16_BIT_INT: int = 0xFFFF
    __serialVersionUID: int = 7557161713937335013

    __AUTHORITY_REGEX: str = None  # LLM could not translate this field

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

        return self.__isOff(UrlValidator.NO_FRAGMENTS)

    def _isValidQuery(self, query: str) -> bool:
        if query is None:
            return True

        return self.__QUERY_PATTERN.fullmatch(query) is not None

    def _isValidPath(self, path: str) -> bool:
        if path is None:
            return False

        if not self.__PATH_PATTERN.match(path):
            return False

        try:
            from urllib.parse import urljoin

            # Simulate Java's URI normalization behavior
            # Use urljoin with a base URL to normalize the path
            base_url = "http://localhost"
            full_url = urljoin(base_url + "/", path.lstrip("/"))

            # Extract the normalized path
            from urllib.parse import urlparse

            norm = urlparse(full_url).path

            # Check if trying to go via or to parent dir
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

        if self._UrlValidator__authorityValidator is not None and self._UrlValidator__authorityValidator.isValid(
            authority
        ):
            return True

        authorityASCII = DomainValidator.unicodeToASCII(authority)

        authorityMatcher = self._UrlValidator__AUTHORITY_PATTERN.fullmatch(authorityASCII)
        if authorityMatcher is None:
            return False

        ipv6 = authorityMatcher.group(self._UrlValidator__PARSE_AUTHORITY_IPV6)
        if ipv6 is not None:
            inetAddressValidator = InetAddressValidator.getInstance()
            if not inetAddressValidator.isValidInet6Address(ipv6):
                return False
        else:
            hostLocation = authorityMatcher.group(self._UrlValidator__PARSE_AUTHORITY_HOST_IP)
            if not self._UrlValidator__domainValidator.isValid(hostLocation):
                inetAddressValidator = InetAddressValidator.getInstance()
                if not inetAddressValidator.isValidInet4Address(hostLocation):
                    return False

            port = authorityMatcher.group(self._UrlValidator__PARSE_AUTHORITY_PORT)
            if port is not None and len(port) > 0:
                try:
                    iPort = int(port)
                    if iPort < 0 or iPort > self._UrlValidator__MAX_UNSIGNED_16_BIT_INT:
                        return False
                except ValueError:
                    return False

        extra = authorityMatcher.group(self._UrlValidator__PARSE_AUTHORITY_EXTRA)
        if extra is not None and len(extra.strip()) > 0:
            return False

        return True

    def _isValidScheme(self, scheme: str) -> bool:
        if scheme is None:
            return False

        if not self.__SCHEME_PATTERN.fullmatch(scheme):
            return False

        if self.__isOff(UrlValidator.ALLOW_ALL_SCHEMES) and scheme.lower() not in self.__allowedSchemes:
            return False

        return True

    def isValid(self, value: str) -> bool:
        if value is None:
            return False

        # ensure value is a valid URI
        try:
            from urllib.parse import urlparse

            uri = urlparse(value)

            # Java's URI constructor validates syntax strictly
            # Check for basic URI validity that urlparse doesn't catch
            if not uri.scheme:
                return False

        except Exception:
            return False

        scheme = uri.scheme
        if not self._isValidScheme(scheme):
            return False

        # Use netloc for authority (this is the "raw" authority in Python)
        authority = uri.netloc

        # For file scheme special handling
        if scheme == "file" and (authority is None or authority == ""):
            return True  # this is a local file - nothing more to do here
        elif scheme == "file" and authority is not None and ":" in authority:
            return False
        else:
            if not self._isValidAuthority(authority):
                return False

        # Use path directly (urlparse doesn't decode it by default)
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
    def UrlValidator5(schemes: typing.List[typing.List[str]]) -> UrlValidator:

        pass  # LLM could not translate this method

    @staticmethod
    def UrlValidator4(options: int) -> UrlValidator:
        return UrlValidator.UrlValidator1(None, None, options)

    @staticmethod
    def UrlValidator3(schemes: typing.List[str], options: int) -> UrlValidator:
        return UrlValidator.UrlValidator1(schemes, None, options)

    @staticmethod
    def UrlValidator2(authorityValidator: RegexValidator, options: int) -> UrlValidator:
        return UrlValidator.UrlValidator1(None, authorityValidator, options)

    @staticmethod
    def UrlValidator1(
        schemes: typing.List[typing.List[str]],
        authorityValidator: RegexValidator,
        options: int,
    ) -> UrlValidator:

        pass  # LLM could not translate this method

    def __init__(
        self,
        schemes: typing.List[str],
        authorityValidator: RegexValidator,
        options: int,
        domainValidator: DomainValidator,
    ) -> None:
        self.__options = options
        if domainValidator is None:
            raise ValueError("DomainValidator must not be null")
        if domainValidator.isAllowLocal() != ((options & UrlValidator.ALLOW_LOCAL_URLS) > 0):
            raise ValueError("DomainValidator disagrees with ALLOW_LOCAL_URLS setting")
        self.__domainValidator = domainValidator

        if self.__isOn0(UrlValidator.ALLOW_ALL_SCHEMES):
            self.__allowedSchemes = set()
        else:
            if schemes is None:
                schemes = UrlValidator.__DEFAULT_SCHEMES
            self.__allowedSchemes = set()
            for i in range(len(schemes)):
                self.__allowedSchemes.add(schemes[i].lower())

        self.__authorityValidator = authorityValidator

    @staticmethod
    def getInstance() -> UrlValidator:
        return UrlValidator.__DEFAULT_URL_VALIDATOR

    def __isOff(self, flag: int) -> bool:
        return (self._UrlValidator__options & flag) == 0

    @staticmethod
    def __isOn1(flag: int, options: int) -> bool:
        return (options & flag) > 0

    def __isOn0(self, flag: int) -> bool:

        pass  # LLM could not translate this method
