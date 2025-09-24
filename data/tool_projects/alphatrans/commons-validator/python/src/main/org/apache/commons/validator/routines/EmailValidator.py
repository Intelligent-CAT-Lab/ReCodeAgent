from __future__ import annotations
import re
import io
from src.main.org.apache.commons.validator.routines.InetAddressValidator import *
from src.main.org.apache.commons.validator.routines.DomainValidator import *


class EmailValidator:

    __domainValidator: DomainValidator = None

    __EMAIL_VALIDATOR_WITH_LOCAL_WITH_TLD: EmailValidator = None
    __EMAIL_VALIDATOR_WITH_LOCAL: EmailValidator = None
    __EMAIL_VALIDATOR_WITH_TLD: EmailValidator = None
    __EMAIL_VALIDATOR: EmailValidator = None
    __allowTld: bool = False

    __MAX_USERNAME_LEN: int = 64

    __USER_PATTERN: re.Pattern = None  # LLM could not translate this field

    __IP_DOMAIN_REGEX: str = r"^\[(.*)\]$"
    __EMAIL_PATTERN: re.Pattern = re.compile(r"^(.+)@(\S+)$")
    __EMAIL_REGEX: str = r"^(.+)@(\S+)$"
    __QUOTED_USER: str = r'("(\\\\"|[^"])*")'
    __SPECIAL_CHARS: str = r"\p{Cntrl}\(\)<>@,;:'\\\"\.\\[\]"
    __serialVersionUID: int = 1705927040799295880
    __VALID_CHARS: str = (
        r"(\\.)|[^\s" + (__SPECIAL_CHARS if __SPECIAL_CHARS is not None else "") + "]"
    )
    __WORD: str = None
    __USER_REGEX: str = None

    @staticmethod
    def initialize_fields() -> None:
        EmailValidator.__EMAIL_VALIDATOR_WITH_LOCAL_WITH_TLD: EmailValidator = (
            EmailValidator(1, True, True, None)
        )

        EmailValidator.__EMAIL_VALIDATOR_WITH_LOCAL: EmailValidator = EmailValidator(
            1, True, False, None
        )

        EmailValidator.__EMAIL_VALIDATOR_WITH_TLD: EmailValidator = EmailValidator(
            1, False, True, None
        )

        EmailValidator.__EMAIL_VALIDATOR: EmailValidator = EmailValidator(
            1, False, False, None
        )

        EmailValidator.__WORD: str = (
            f"(({EmailValidator.__VALID_CHARS}|'')+|{EmailValidator.__QUOTED_USER})"
        )

        EmailValidator.__USER_REGEX: str = (
            f"^{EmailValidator.__WORD}(.{EmailValidator.__WORD})*$"
        )

    def _isValidUser(self, user: str) -> bool:
        if user is None or len(user) > self.__MAX_USERNAME_LEN:
            return False

        return bool(self.__USER_PATTERN.match(user))

    def _isValidDomain(self, domain: str) -> bool:
        ip_domain_matcher = self.__IP_DOMAIN_PATTERN.match(domain)

        if ip_domain_matcher:
            inet_address_validator = InetAddressValidator.getInstance()
            return inet_address_validator.isValid(ip_domain_matcher.group(1))

        if self.__allowTld:
            return self.__domainValidator.isValid(domain) or (
                not domain.startswith(".") and self.__domainValidator.isValidTld(domain)
            )
        else:
            return self.__domainValidator.isValid(domain)

    def isValid(self, email: str) -> bool:
        if email is None:
            return False

        if email.endswith("."):  # check this first - it's cheap!
            return False

        email_matcher = self.__EMAIL_PATTERN.match(email)
        if not email_matcher:
            return False

        user, domain = email_matcher.groups()

        if not self._isValidUser(user):
            return False

        if not self._isValidDomain(domain):
            return False

        return True

    @staticmethod
    def EmailValidator0(allowLocal: bool) -> EmailValidator:
        return EmailValidator(1, allowLocal, False, None)

    def __init__(
        self,
        constructorId: int,
        allowLocal: bool,
        allowTld: bool,
        domainValidator: DomainValidator,
    ) -> None:
        if constructorId == 0:
            self.__allowTld = allowTld
            if domainValidator is None:
                raise ValueError("DomainValidator cannot be null")
            else:
                if domainValidator.isAllowLocal() != allowLocal:
                    raise ValueError(
                        "DomainValidator must agree with allowLocal setting"
                    )
                self.__domainValidator = domainValidator
        else:
            self.__allowTld = allowTld
            self.__domainValidator = DomainValidator.getInstance1(allowLocal)

    @staticmethod
    def getInstance2(allowLocal: bool) -> EmailValidator:
        return EmailValidator.getInstance1(allowLocal, False)

    @staticmethod
    def getInstance1(allowLocal: bool, allowTld: bool) -> EmailValidator:
        if allowLocal:
            if allowTld:
                return EmailValidator.__EMAIL_VALIDATOR_WITH_LOCAL_WITH_TLD
            else:
                return EmailValidator.__EMAIL_VALIDATOR_WITH_LOCAL
        else:
            if allowTld:
                return EmailValidator.__EMAIL_VALIDATOR_WITH_TLD
            else:
                return EmailValidator.__EMAIL_VALIDATOR

    @staticmethod
    def getInstance0() -> EmailValidator:
        return EmailValidator.__EMAIL_VALIDATOR


EmailValidator.initialize_fields()
