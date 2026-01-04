from __future__ import annotations
import io
from src.main.org.apache.commons.validator.routines.InetAddressValidator import *
from src.main.org.apache.commons.validator.routines.DomainValidator import *


class EmailValidator:

    __EMAIL_VALIDATOR: EmailValidator = None
    __USER_PATTERN: re.Pattern = None  # LLM could not translate this field

    __TLD_PATTERN: re.Pattern = re.compile(r"^([a-zA-Z]+)$")
    __IP_DOMAIN_PATTERN: re.Pattern = re.compile(r"^\[(.*)\]$")
    __WORD: str = None  # LLM could not translate this field

    __ATOM: str = None  # LLM could not translate this field

    __QUOTED_USER: str = '("[^"]*")'
    __SPECIAL_CHARS: str = None  # LLM could not translate this field

    __ATOM_PATTERN: re.Pattern = None  # LLM could not translate this field

    __DOMAIN_PATTERN: re.Pattern = (
        re.compile(r"^" + (__ATOM or "") + r"(\." + (__ATOM or "") + r")*\s*$") if __ATOM else None
    )

    @staticmethod
    def initialize_fields() -> None:
        EmailValidator.__EMAIL_VALIDATOR: EmailValidator = EmailValidator()

    def _stripComments(self, emailStr: str) -> str:
        import re

        result = emailStr
        commentPat = r'^((?:[^"\\]|\\.)*(?:"(?:[^"\\]|\\.)*"(?:[^"\\]|\\.)*)*)\\((?:[^()\\]|\\.)*\\)'

        while re.match(commentPat, result):
            result = re.sub(commentPat, r"\1 ", result, count=1)

        return result

    def _isValidSymbolicDomain(self, domain: str) -> bool:
        domainSegment = [None] * 10
        match = True
        i = 0

        while match:
            atomMatcher = self.__ATOM_PATTERN.match(domain)
            match = atomMatcher is not None
            if match:
                domainSegment[i] = atomMatcher.group(1)
                l = len(domainSegment[i]) + 1
                domain = "" if l >= len(domain) else domain[l:]
                i += 1

        length = i

        if length < 2:
            return False

        tld = domainSegment[length - 1]
        if len(tld) > 1:
            if not self.__TLD_PATTERN.match(tld):
                return False
        else:
            return False

        return True

    def _isValidIpAddress(self, ipAddress: str) -> bool:
        ipAddressMatcher = self.__IP_DOMAIN_PATTERN.match(ipAddress)

        if not ipAddressMatcher:
            return False

        for i in range(1, 5):  # 1 to 4 inclusive
            ipSegment = ipAddressMatcher.group(i)
            if ipSegment is None or len(ipSegment) <= 0:
                return False

            iIpSegment = 0

            try:
                iIpSegment = int(ipSegment)
            except ValueError:
                return False

            if iIpSegment > 255:
                return False

        return True

    def _isValidUser(self, user: str) -> bool:
        return self.__USER_PATTERN.match(user) is not None

    def _isValidDomain(self, domain: str) -> bool:
        symbolic = False

        ipDomainMatcher = self.__IP_DOMAIN_PATTERN.match(domain)

        if ipDomainMatcher:
            inetAddressValidator = InetAddressValidator.getInstance()
            if inetAddressValidator.isValid(ipDomainMatcher.group(1)):
                return True
        else:
            symbolic = self.__DOMAIN_PATTERN.match(domain) is not None

        if symbolic:
            if not self._isValidSymbolicDomain(domain):
                return False
        else:
            return False

        return True

    def isValid(self, email: str) -> bool:

        pass  # LLM could not translate this method

    def __init__(self) -> None:
        super().__init__()

    @staticmethod
    def getInstance() -> EmailValidator:
        return EmailValidator.__EMAIL_VALIDATOR


EmailValidator.initialize_fields()
