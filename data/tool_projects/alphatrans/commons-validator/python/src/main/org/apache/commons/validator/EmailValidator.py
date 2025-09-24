from __future__ import annotations
import io
from src.main.org.apache.commons.validator.routines.InetAddressValidator import *
from src.main.org.apache.commons.validator.routines.DomainValidator import *


class EmailValidator:

    __EMAIL_VALIDATOR: EmailValidator = None
    __TLD_PATTERN: re.Pattern = re.compile(r"^([a-zA-Z]+)$")
    __IP_DOMAIN_PATTERN: re.Pattern = re.compile(r"^\[(.*)\]$")
    __QUOTED_USER: str = r'("[^"]*")'
    __VALID_CHARS: str = r"[^\\s" + r"\\p{Cntrl}\\(\\)<>@,;:'\\\\\\\"\\.\\[\\]" + r"]"
    __SPECIAL_CHARS: str = r"\p{Cntrl}\(\)<>@,;:'\\\"\.\\[\]"
    __WORD: str = None

    __ATOM: str = None  # LLM could not translate this field

    __ATOM_PATTERN: re.Pattern = None
    __DOMAIN_PATTERN: re.Pattern = None
    __USER_PATTERN: re.Pattern = re.compile(rf"^\s*{__WORD}(\.{__WORD})*$")

    @staticmethod
    def initialize_fields() -> None:
        EmailValidator.__EMAIL_VALIDATOR: EmailValidator = EmailValidator()

        EmailValidator.__WORD: str = (
            f"(({EmailValidator.__VALID_CHARS}|'')+|{EmailValidator.__QUOTED_USER})"
        )

        EmailValidator.__ATOM_PATTERN: re.Pattern = re.compile(
            f"({EmailValidator.__ATOM})"
        )

        EmailValidator.__DOMAIN_PATTERN: re.Pattern = re.compile(
            rf"^{EmailValidator.__ATOM}(.{EmailValidator.__ATOM})*s*$"
        )

    def _stripComments(self, emailStr: str) -> str:
        import re

        result = emailStr
        commentPat = r'^((?:[^"\\\\]|\\\\.)*(?:"(?:[^"\\\\]|\\\\.)*"(?:[^"\\\\]|\\\\.)*)*)\((?:[^()\\\\]|\\\\.)*\)'
        commentMatcher = re.compile(commentPat)

        while commentMatcher.match(result):
            result = re.sub(commentPat, r"\1 ", result)

        return result

    def _isValidSymbolicDomain(self, domain: str) -> bool:
        domain_segment = [None] * 10  # Equivalent to `new String[10]` in Java
        match = True
        i = 0
        atom_matcher = self.__ATOM_PATTERN.match(domain)

        while match:
            if atom_matcher:
                domain_segment[i] = atom_matcher.group(1)
                l = len(domain_segment[i]) + 1
                domain = "" if l >= len(domain) else domain[l:]
                i += 1
                atom_matcher = self.__ATOM_PATTERN.match(domain)
            else:
                match = False

        length = i

        if length < 2:
            return False

        tld = domain_segment[length - 1]
        if len(tld) > 1:
            if not self.__TLD_PATTERN.match(tld):
                return False
        else:
            return False

        return True

    def _isValidIpAddress(self, ipAddress: str) -> bool:
        ip_address_matcher = self.__IP_DOMAIN_PATTERN.match(ipAddress)
        if not ip_address_matcher:
            return False

        ip_segments = ip_address_matcher.group(1).split(".")
        if len(ip_segments) != 4:
            return False

        for ip_segment in ip_segments:
            if not ip_segment or len(ip_segment) == 0:
                return False

            try:
                i_ip_segment = int(ip_segment)
            except ValueError:
                return False

            if i_ip_segment < 0 or i_ip_segment > 255:
                return False

        return True

    def _isValidUser(self, user: str) -> bool:
        return bool(self.__USER_PATTERN.match(user))

    def _isValidDomain(self, domain: str) -> bool:
        symbolic = False

        ip_domain_matcher = self.__IP_DOMAIN_PATTERN.match(domain)

        if ip_domain_matcher:
            inet_address_validator = InetAddressValidator.getInstance()
            if inet_address_validator.isValid(ip_domain_matcher.group(1)):
                return True
        else:
            symbolic = bool(self.__DOMAIN_PATTERN.match(domain))

        if symbolic:
            if not self._isValidSymbolicDomain(domain):
                return False
        else:
            return False

        return True

    def isValid(self, email: str) -> bool:
        return EmailValidator.getInstance0().isValid(email)

    def __init__(self) -> None:
        super().__init__()

    @staticmethod
    def getInstance() -> EmailValidator:
        return EmailValidator.__EMAIL_VALIDATOR


EmailValidator.initialize_fields()
