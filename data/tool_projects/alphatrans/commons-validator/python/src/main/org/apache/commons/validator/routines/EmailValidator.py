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

    __IP_DOMAIN_PATTERN: re.Pattern = re.compile(r"^\[(.*)\]$")
    __EMAIL_PATTERN: re.Pattern = re.compile("^(.+)@(\\S+)$")
    __USER_REGEX: str = None  # LLM could not translate this field

    __IP_DOMAIN_REGEX: str = "^\\[(.*)\\]$"
    __EMAIL_REGEX: str = "^(.+)@(\\S+)$"
    __QUOTED_USER: str = '("(\\\\"|[^"])*")'
    __SPECIAL_CHARS: str = None  # LLM could not translate this field

    __serialVersionUID: int = 1705927040799295880

    __WORD: str = None  # LLM could not translate this field

    @staticmethod
    def initialize_fields() -> None:
        EmailValidator.__EMAIL_VALIDATOR_WITH_LOCAL: EmailValidator = EmailValidator(1, True, False, None)

        EmailValidator.__EMAIL_VALIDATOR_WITH_TLD: EmailValidator = EmailValidator(1, False, True, None)

        EmailValidator.__EMAIL_VALIDATOR: EmailValidator = EmailValidator(1, False, False, None)

    def _isValidUser(self, user: str) -> bool:
        if user is None or len(user) > EmailValidator.__MAX_USERNAME_LEN:
            return False

        return EmailValidator.__USER_PATTERN.fullmatch(user) is not None

    def _isValidDomain(self, domain: str) -> bool:
        ipDomainMatcher = self._EmailValidator__IP_DOMAIN_PATTERN.fullmatch(domain)

        if ipDomainMatcher:
            inetAddressValidator = InetAddressValidator.getInstance()
            return inetAddressValidator.isValid(ipDomainMatcher.group(1))

        if self._EmailValidator__allowTld:
            return self._EmailValidator__domainValidator.isValid(domain) or (
                not domain.startswith(".") and self._EmailValidator__domainValidator.isValidTld(domain)
            )
        else:
            return self._EmailValidator__domainValidator.isValid(domain)

    def isValid(self, email: str) -> bool:
        if email is None:
            return False

        if email.endswith("."):  # check this first - it's cheap!
            return False

        emailMatcher = self._EmailValidator__EMAIL_PATTERN.fullmatch(email)
        if not emailMatcher:
            return False

        if not self._isValidUser(emailMatcher.group(1)):
            return False

        if not self._isValidDomain(emailMatcher.group(2)):
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
                    raise ValueError("DomainValidator must agree with allowLocal setting")
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
                if EmailValidator.__EMAIL_VALIDATOR_WITH_LOCAL_WITH_TLD is None:
                    EmailValidator.__EMAIL_VALIDATOR_WITH_LOCAL_WITH_TLD = EmailValidator(1, True, True, None)
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
        return EmailValidator._EmailValidator__EMAIL_VALIDATOR


EmailValidator.initialize_fields()
