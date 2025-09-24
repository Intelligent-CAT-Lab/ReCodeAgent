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

    _defaultSchemes: List[str] = ["http", "https", "ftp"]
    NO_FRAGMENTS: int = 1 << 2
    ALLOW_2_SLASHES: int = 1 << 1
    ALLOW_ALL_SCHEMES: int = 1 << 0
    __allowedSchemes: typing.Set[str] = set()
    __options: Flags = None

    __ALPHA_CHARS: str = "a-zA-Z"
    __PORT_PATTERN: re.Pattern = re.compile(r"^:(\d{1,5})$")
    __LEGAL_ASCII_PATTERN: re.Pattern = re.compile(r"^[\x00-\x7F]+$")
    __QUERY_PATTERN: re.Pattern = re.compile(r"^(.*)$")
    __PATH_PATTERN: re.Pattern = re.compile(r"^(/[-\w:@&?=+,.!/~*'%$_;]*)?$")
    __PARSE_AUTHORITY_EXTRA: int = 3
    __PARSE_AUTHORITY_PORT: int = 2
    __PARSE_AUTHORITY_HOST_IP: int = 1
    __AUTHORITY_REGEX: str = r"^([\w\-.]*)(:\d*)?(.*)?"
    __SCHEME_PATTERN: re.Pattern = re.compile(r"^[A-Za-z][A-Za-z0-9+\-\.]*")
    __PARSE_URL_FRAGMENT: int = 9
    __PARSE_URL_QUERY: int = 7
    __PARSE_URL_PATH: int = 5
    __PARSE_URL_AUTHORITY: int = 4
    __PARSE_URL_SCHEME: int = 2
    __URL_PATTERN: re.Pattern = re.compile(
        r"^(([^:/?#]+):)?(//([^/?#]*))?([^?#]*)(\?([^#]*))?(#(.*))?"
    )
    __URL_REGEX: str = r"^(([^:/?#]+):)?(//([^/?#]*))?([^?#]*)(\\?([^#]*))?(#(.*))?"
    __AUTHORITY_CHARS_REGEX: str = r"\p{Alnum}\-\."
    __VALID_CHARS: str = r"[^\\s;/@&=,.?:+$]"
    __SPECIAL_CHARS: str = ";/@&=,.?:+$"
    __serialVersionUID: int = 24137157400029593

    __ATOM: str = None  # LLM could not translate this field

    __DOMAIN_PATTERN: re.Pattern = re.compile(rf"^{__ATOM}(\.{__ATOM})*$")

    __AUTHORITY_PATTERN: re.Pattern = None  # LLM could not translate this field

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

        return self.__options.isOff(self.NO_FRAGMENTS)

    def _isValidQuery(self, query: str) -> bool:
        if query is None:
            return True

        return self.__QUERY_PATTERN.match(query) is not None

    def _isValidPath(self, path: str) -> bool:
        if path is None:
            return False

        if not self.__PATH_PATTERN.match(path):
            return False

        slash2_count = self._countToken("//", path)
        if self.__options.isOff(self.ALLOW_2_SLASHES) and (slash2_count > 0):
            return False

        slash_count = self._countToken("/", path)
        dot2_count = self._countToken("..", path)
        if dot2_count > 0 and (slash_count - slash2_count - 1) <= dot2_count:
            return False

        return True

    def _isValidAuthority(self, authority: str) -> bool:
        if authority is None:
            return False

        inet_address_validator = InetAddressValidator.getInstance()

        authority_matcher = self.__AUTHORITY_PATTERN.match(authority)
        if not authority_matcher:
            return False

        hostname = False
        host_ip = authority_matcher.group(self.__PARSE_AUTHORITY_HOST_IP)
        ipv4_address = inet_address_validator.isValid(host_ip)

        if not ipv4_address:
            hostname = bool(self.__DOMAIN_PATTERN.match(host_ip))

        if hostname:
            chars = list(host_ip)
            size = 1
            for char in chars:
                if char == ".":
                    size += 1

            domain_segment = [None] * size
            match = True
            segment_count = 0
            segment_length = 0

            while match:
                atom_matcher = self.__ATOM_PATTERN.match(host_ip)
                match = bool(atom_matcher)
                if match:
                    domain_segment[segment_count] = atom_matcher.group(1)
                    segment_length = len(domain_segment[segment_count]) + 1
                    host_ip = (
                        ""
                        if segment_length >= len(host_ip)
                        else host_ip[segment_length:]
                    )
                    segment_count += 1

            top_level = domain_segment[segment_count - 1]
            if len(top_level) < 2 or len(top_level) > 4:
                return False

            if not self.__ALPHA_PATTERN.match(top_level[0]):
                return False

            if segment_count < 2:
                return False

        if not hostname and not ipv4_address:
            return False

        port = authority_matcher.group(self.__PARSE_AUTHORITY_PORT)
        if port is not None and not self.__PORT_PATTERN.match(port):
            return False

        extra = authority_matcher.group(self.__PARSE_AUTHORITY_EXTRA)
        if not GenericValidator.isBlankOrNull(extra):
            return False

        return True

    def _isValidScheme(self, scheme: str) -> bool:
        if scheme is None:
            return False

        if not self.__SCHEME_PATTERN.match(scheme):
            return False

        if (
            self.__options.isOff(self.ALLOW_ALL_SCHEMES)
            and scheme not in self.__allowedSchemes
        ):
            return False

        return True

    def isValid(self, value: str) -> bool:
        if value is None:
            return False

        if not self.__LEGAL_ASCII_PATTERN.match(value):
            return False

        url_matcher = self.__URL_PATTERN.match(value)
        if not url_matcher:
            return False

        if not self._isValidScheme(url_matcher.group(self.__PARSE_URL_SCHEME)):
            return False

        if not self._isValidAuthority(url_matcher.group(self.__PARSE_URL_AUTHORITY)):
            return False

        if not self._isValidPath(url_matcher.group(self.__PARSE_URL_PATH)):
            return False

        if not self._isValidQuery(url_matcher.group(self.__PARSE_URL_QUERY)):
            return False

        if not self._isValidFragment(url_matcher.group(self.__PARSE_URL_FRAGMENT)):
            return False

        return True

    @staticmethod
    def UrlValidator3() -> UrlValidator:
        return UrlValidator.UrlValidator2(None)

    @staticmethod
    def UrlValidator2(schemes: Optional[List[str]]) -> UrlValidator:
        return UrlValidator(schemes, 0)

    @staticmethod
    def UrlValidator1(options: int) -> UrlValidator:
        return UrlValidator(None, options)

    def __init__(self, schemes: Optional[List[str]], options: int) -> None:
        self.__options = Flags(1, options)

        if self.__options.isOn(self.ALLOW_ALL_SCHEMES):
            return

        if schemes is None:
            schemes = self._defaultSchemes

        self.__allowedSchemes.update(schemes)
