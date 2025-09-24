from __future__ import annotations
import time
import copy
import re
import enum
import numbers
import io
import typing
from typing import *
import os
from src.main.org.apache.commons.validator.routines.RegexValidator import *


class DomainValidator:

    mylocalTLDsMinus: typing.List[typing.List[str]] = None

    mylocalTLDsPlus: typing.List[typing.List[str]] = None

    mygenericTLDsMinus: typing.List[typing.List[str]] = None

    mygenericTLDsPlus: typing.List[typing.List[str]] = None

    mycountryCodeTLDsPlus: typing.List[typing.List[str]] = None

    mycountryCodeTLDsMinus: typing.List[typing.List[str]] = None

    __inUse: bool = False
    __LOCAL_TLDS: List[str] = [
        "localdomain",  # Also widely used as localhost.localdomain
        "localhost",  # RFC2606 defined
    ]
    __COUNTRY_CODE_TLDS: List[str] = [
        "ac",  # Ascension Island
        "ad",  # Andorra
        "ae",  # United Arab Emirates
        "af",  # Afghanistan
        "ag",  # Antigua and Barbuda
        "ai",  # Anguilla
        "al",  # Albania
        "am",  # Armenia
        "ao",  # Angola
        "aq",  # Antarctica
        "ar",  # Argentina
        "as",  # American Samoa
        "at",  # Austria
        "au",  # Australia (includes Ashmore and Cartier Islands and Coral Sea Islands)
        "aw",  # Aruba
        "ax",  # Åland
        "az",  # Azerbaijan
        "ba",  # Bosnia and Herzegovina
        "bb",  # Barbados
        "bd",  # Bangladesh
        "be",  # Belgium
        "bf",  # Burkina Faso
        "bg",  # Bulgaria
        "bh",  # Bahrain
        "bi",  # Burundi
        "bj",  # Benin
        "bm",  # Bermuda
        "bn",  # Brunei Darussalam
        "bo",  # Bolivia
        "br",  # Brazil
        "bs",  # Bahamas
        "bt",  # Bhutan
        "bv",  # Bouvet Island
        "bw",  # Botswana
        "by",  # Belarus
        "bz",  # Belize
        "ca",  # Canada
        "cc",  # Cocos (Keeling) Islands
        "cd",  # Democratic Republic of the Congo (formerly Zaire)
        "cf",  # Central African Republic
        "cg",  # Republic of the Congo
        "ch",  # Switzerland
        "ci",  # Côte d'Ivoire
        "ck",  # Cook Islands
        "cl",  # Chile
        "cm",  # Cameroon
        "cn",  # China, mainland
        "co",  # Colombia
        "cr",  # Costa Rica
        "cu",  # Cuba
        "cv",  # Cape Verde
        "cw",  # Curaçao
        "cx",  # Christmas Island
        "cy",  # Cyprus
        "cz",  # Czech Republic
        "de",  # Germany
        "dj",  # Djibouti
        "dk",  # Denmark
        "dm",  # Dominica
        "do",  # Dominican Republic
        "dz",  # Algeria
        "ec",  # Ecuador
        "ee",  # Estonia
        "eg",  # Egypt
        "er",  # Eritrea
        "es",  # Spain
        "et",  # Ethiopia
        "eu",  # European Union
        "fi",  # Finland
        "fj",  # Fiji
        "fk",  # Falkland Islands
        "fm",  # Federated States of Micronesia
        "fo",  # Faroe Islands
        "fr",  # France
        "ga",  # Gabon
        "gb",  # Great Britain (United Kingdom)
        "gd",  # Grenada
        "ge",  # Georgia
        "gf",  # French Guiana
        "gg",  # Guernsey
        "gh",  # Ghana
        "gi",  # Gibraltar
        "gl",  # Greenland
        "gm",  # The Gambia
        "gn",  # Guinea
        "gp",  # Guadeloupe
        "gq",  # Equatorial Guinea
        "gr",  # Greece
        "gs",  # South Georgia and the South Sandwich Islands
        "gt",  # Guatemala
        "gu",  # Guam
        "gw",  # Guinea-Bissau
        "gy",  # Guyana
        "hk",  # Hong Kong
        "hm",  # Heard Island and McDonald Islands
        "hn",  # Honduras
        "hr",  # Croatia (Hrvatska)
        "ht",  # Haiti
        "hu",  # Hungary
        "id",  # Indonesia
        "ie",  # Ireland (Éire)
        "il",  # Israel
        "im",  # Isle of Man
        "in",  # India
        "io",  # British Indian Ocean Territory
        "iq",  # Iraq
        "ir",  # Iran
        "is",  # Iceland
        "it",  # Italy
        "je",  # Jersey
        "jm",  # Jamaica
        "jo",  # Jordan
        "jp",  # Japan
        "ke",  # Kenya
        "kg",  # Kyrgyzstan
        "kh",  # Cambodia (Khmer)
        "ki",  # Kiribati
        "km",  # Comoros
        "kn",  # Saint Kitts and Nevis
        "kp",  # North Korea
        "kr",  # South Korea
        "kw",  # Kuwait
        "ky",  # Cayman Islands
        "kz",  # Kazakhstan
        "la",  # Laos (currently being marketed as the official domain for Los Angeles)
        "lb",  # Lebanon
        "lc",  # Saint Lucia
        "li",  # Liechtenstein
        "lk",  # Sri Lanka
        "lr",  # Liberia
        "ls",  # Lesotho
        "lt",  # Lithuania
        "lu",  # Luxembourg
        "lv",  # Latvia
        "ly",  # Libya
        "ma",  # Morocco
        "mc",  # Monaco
        "md",  # Moldova
        "me",  # Montenegro
        "mg",  # Madagascar
        "mh",  # Marshall Islands
        "mk",  # Republic of Macedonia
        "ml",  # Mali
        "mm",  # Myanmar
        "mn",  # Mongolia
        "mo",  # Macau
        "mp",  # Northern Mariana Islands
        "mq",  # Martinique
        "mr",  # Mauritania
        "ms",  # Montserrat
        "mt",  # Malta
        "mu",  # Mauritius
        "mv",  # Maldives
        "mw",  # Malawi
        "mx",  # Mexico
        "my",  # Malaysia
        "mz",  # Mozambique
        "na",  # Namibia
        "nc",  # New Caledonia
        "ne",  # Niger
        "nf",  # Norfolk Island
        "ng",  # Nigeria
        "ni",  # Nicaragua
        "nl",  # Netherlands
        "no",  # Norway
        "np",  # Nepal
        "nr",  # Nauru
        "nu",  # Niue
        "nz",  # New Zealand
        "om",  # Oman
        "pa",  # Panama
        "pe",  # Peru
        "pf",  # French Polynesia With Clipperton Island
        "pg",  # Papua New Guinea
        "ph",  # Philippines
        "pk",  # Pakistan
        "pl",  # Poland
        "pm",  # Saint-Pierre and Miquelon
        "pn",  # Pitcairn Islands
        "pr",  # Puerto Rico
        "ps",  # Palestinian territories (PA-controlled West Bank and Gaza Strip)
        "pt",  # Portugal
        "pw",  # Palau
        "py",  # Paraguay
        "qa",  # Qatar
        "re",  # Réunion
        "ro",  # Romania
        "rs",  # Serbia
        "ru",  # Russia
        "rw",  # Rwanda
        "sa",  # Saudi Arabia
        "sb",  # Solomon Islands
        "sc",  # Seychelles
        "sd",  # Sudan
        "se",  # Sweden
        "sg",  # Singapore
        "sh",  # Saint Helena
        "si",  # Slovenia
        "sj",  # Svalbard and Jan Mayen Islands Not in use (Norwegian dependencies; see .no)
        "sk",  # Slovakia
        "sl",  # Sierra Leone
        "sm",  # San Marino
        "sn",  # Senegal
        "so",  # Somalia
        "sr",  # Suriname
        "ss",  # ss National Communication Authority (NCA)
        "st",  # São Tomé and Príncipe
        "su",  # Soviet Union (deprecated)
        "sv",  # El Salvador
        "sx",  # Sint Maarten
        "sy",  # Syria
        "sz",  # Swaziland
        "tc",  # Turks and Caicos Islands
        "td",  # Chad
        "tf",  # French Southern and Antarctic Lands
        "tg",  # Togo
        "th",  # Thailand
        "tj",  # Tajikistan
        "tk",  # Tokelau
        "tl",  # East Timor (deprecated old code)
        "tm",  # Turkmenistan
        "tn",  # Tunisia
        "to",  # Tonga
        "tr",  # Turkey
        "tt",  # Trinidad and Tobago
        "tv",  # Tuvalu
        "tw",  # Taiwan, Republic of China
        "tz",  # Tanzania
        "ua",  # Ukraine
        "ug",  # Uganda
        "uk",  # United Kingdom
        "us",  # United States of America
        "uy",  # Uruguay
        "uz",  # Uzbekistan
        "va",  # Vatican City State
        "vc",  # Saint Vincent and the Grenadines
        "ve",  # Venezuela
        "vg",  # British Virgin Islands
        "vi",  # U.S. Virgin Islands
        "vn",  # Vietnam
        "vu",  # Vanuatu
        "wf",  # Wallis and Futuna
        "ws",  # Samoa (formerly Western Samoa)
        "xn--2scrj9c",  # ಭಾರತ National Internet eXchange of India
        "xn--3e0b707e",  # 한국 KISA (Korea Internet &amp; Security Agency)
        "xn--3hcrj9c",  # ଭାରତ National Internet eXchange of India
        "xn--45br5cyl",  # ভাৰত National Internet eXchange of India
        "xn--45brj9c",  # ভারত National Internet Exchange of India
        "xn--54b7fta0cc",  # বাংলা Posts and Telecommunications Division
        "xn--80ao21a",  # қаз Association of IT Companies of Kazakhstan
        "xn--90a3ac",  # срб Serbian National Internet Domain Registry (RNIDS)
        "xn--90ais",  # ??? Reliable Software Inc.
        "xn--clchc0ea0b2g2a9gcd",  # சிங்கப்பூர் Singapore Network Information Centre
        "xn--d1alf",  # мкд Macedonian Academic Research Network Skopje
        "xn--e1a4c",  # ею EURid vzw/asbl
        "xn--fiqs8s",  # 中国 China Internet Network Information Center
        "xn--fiqz9s",  # 中國 China Internet Network Information Center
        "xn--fpcrj9c3d",  # భారత్ National Internet Exchange of India
        "xn--fzc2c9e2c",  # ලංකා LK Domain Registry
        "xn--gecrj9c",  # ભારત National Internet Exchange of India
        "xn--h2breg3eve",  # भारतम् National Internet eXchange of India
        "xn--h2brj9c",  # भारत National Internet Exchange of India
        "xn--h2brj9c8c",  # भारोत National Internet eXchange of India
        "xn--j1amh",  # укр Ukrainian Network Information Centre (UANIC), Inc.
        "xn--j6w193g",  # 香港 Hong Kong Internet Registration Corporation Ltd.
        "xn--kprw13d",  # 台湾 Taiwan Network Information Center (TWNIC)
        "xn--kpry57d",  # 台灣 Taiwan Network Information Center (TWNIC)
        "xn--l1acc",  # мон Datacom Co.,Ltd
        "xn--lgbbat1ad8j",  # الجزائر CERIST
        "xn--mgb9awbf",  # عمان Telecommunications Regulatory Authority (TRA)
        "xn--mgba3a4f16a",  # ایران Institute for Research in Fundamental Sciences (IPM)
        "xn--mgbaam7a8h",  # امارات Telecommunications Regulatory Authority (TRA)
        "xn--mgbah1a3hjkrd",  # موريتانيا Université de Nouakchott Al Aasriya
        "xn--mgbai9azgqp6j",  # پاکستان National Telecommunication Corporation
        "xn--mgbayh7gpa",  # الاردن National Information Technology Center (NITC)
        "xn--mgbbh1a",  # بارت National Internet eXchange of India
        "xn--mgbbh1a71e",  # بھارت National Internet Exchange of India
        "xn--mgbc0a9azcg",  # المغرب Agence Nationale de Réglementation des
        "xn--mgbcpq6gpa1a",  # البحرين Telecommunications Regulatory Authority (TRA)
        "xn--mgberp4a5d4ar",  # السعودية Communications and Information Technology
        "xn--mgbgu82a",  # ڀارت National Internet eXchange of India
        "xn--mgbpl2fh",  # ????? Sudan Internet Society
        "xn--mgbtx2b",  # عراق Communications and Media Commission (CMC)
        "xn--mgbx4cd0ab",  # مليسيا MYNIC Berhad
        "xn--mix891f",  # 澳門 Bureau of Telecommunications Regulation (DSRT)
        "xn--node",  # გე Information Technologies Development Center (ITDC)
        "xn--o3cw4h",  # ไทย Thai Network Information Center Foundation
        "xn--ogbpf8fl",  # سورية National Agency for Network Services (NANS)
        "xn--p1ai",  # рф Coordination Center for TLD RU
        "xn--pgbs0dh",  # تونس Agence Tunisienne d&#39;Internet
        "xn--q7ce6a",  # ລາວ Lao National Internet Center (LANIC)
        "xn--qxa6a",  # ευ EURid vzw/asbl
        "xn--qxam",  # ελ ICS-FORTH GR
        "xn--rvc1e0am3e",  # ഭാരതം National Internet eXchange of India
        "xn--s9brj9c",  # ਭਾਰਤ National Internet Exchange of India
        "xn--wgbh1c",  # مصر National Telecommunication Regulatory Authority - NTRA
        "xn--wgbl6a",  # قطر Communications Regulatory Authority
        "xn--xkc2al3hye2a",  # இலங்கை LK Domain Registry
        "xn--xkc2dl3a5ee0h",  # இந்தியா National Internet Exchange of India
        "xn--y9a3aq",  # ??? Internet Society
        "xn--yfro4i67o",  # 新加坡 Singapore Network Information Centre (SGNIC) Pte Ltd
        "xn--ygbi2ammx",  # فلسطين Ministry of Telecom &amp; Information Technology (MTIT)
        "ye",  # Yemen
        "yt",  # Mayotte
        "za",  # South Africa
        "zm",  # Zambia
        "zw",  # Zimbabwe
    ]
    __GENERIC_TLDS: typing.List[typing.List[str]] = (
        None  # LLM could not translate this field
    )

    __INFRASTRUCTURE_TLDS: List[str] = [
        "arpa",  # internet infrastructure
    ]
    __DOMAIN_LABEL_REGEX: str = r"\p{Alnum}(?>[\p{Alnum}-]{0,61}\p{Alnum})?"
    __allowLocal: bool = False

    __UNEXPECTED_ENUM_VALUE: str = "Unexpected enum value: "
    __TOP_LABEL_REGEX: str = r"\p{Alpha}(?>[\p{Alnum}-]{0,61}\p{Alnum})?"
    __serialVersionUID: int = -4407125112880174009
    __EMPTY_STRING_ARRAY: List[str] = []
    __MAX_DOMAIN_LENGTH: int = 253
    __localTLDsPlus: typing.List[str] = __EMPTY_STRING_ARRAY
    __localTLDsMinus: typing.List[str] = __EMPTY_STRING_ARRAY
    __genericTLDsMinus: typing.List[str] = __EMPTY_STRING_ARRAY
    __countryCodeTLDsMinus: List[str] = __EMPTY_STRING_ARRAY
    __genericTLDsPlus: typing.List[str] = __EMPTY_STRING_ARRAY
    __countryCodeTLDsPlus: typing.List[str] = __EMPTY_STRING_ARRAY
    __DOMAIN_NAME_REGEX: str = None
    __domainRegex: RegexValidator = RegexValidator(
        __DOMAIN_NAME_REGEX, caseSensitive=True
    )

    @staticmethod
    def initialize_fields() -> None:
        DomainValidator.__DOMAIN_NAME_REGEX: str = (
            f"^(?:{DomainValidator.__DOMAIN_LABEL_REGEX}.)+({DomainValidator.__TOP_LABEL_REGEX}).?$"
        )

    @staticmethod
    def unicodeToASCII(input_: str) -> str:
        if DomainValidator.__isOnlyASCII(input_):  # skip possibly expensive processing
            return input_
        try:
            ascii_ = IDN.toASCII(input_)
            if IDNBUGHOLDER._IDNBUGHOLDER__IDN_TOASCII_PRESERVES_TRAILING_DOTS:
                return ascii_
            length = len(input_)
            if length == 0:  # check there is a last character
                return input_
            last_char = input_[-1]  # fetch original last char
            if last_char in [
                "\u002e",
                "\u3002",
                "\uff0e",
                "\uff61",
            ]:  # check for specific characters
                return ascii_ + "."  # restore the missing stop
            else:
                return ascii_
        except ValueError:  # input is not valid
            return input_

    def getOverrides(self, table: ArrayType) -> typing.List[str]:
        if table == ArrayType.COUNTRY_CODE_MINUS:
            array = self.mycountryCodeTLDsMinus
        elif table == ArrayType.COUNTRY_CODE_PLUS:
            array = self.mycountryCodeTLDsPlus
        elif table == ArrayType.GENERIC_MINUS:
            array = self.mygenericTLDsMinus
        elif table == ArrayType.GENERIC_PLUS:
            array = self.mygenericTLDsPlus
        elif table == ArrayType.LOCAL_MINUS:
            array = self.mylocalTLDsMinus
        elif table == ArrayType.LOCAL_PLUS:
            array = self.mylocalTLDsPlus
        else:
            raise ValueError(f"{self.__UNEXPECTED_ENUM_VALUE}{table}")

        # Return a copy of the array
        return array.copy()

    @staticmethod
    def getTLDEntries(table: ArrayType) -> List[str]:
        if table == ArrayType.COUNTRY_CODE_MINUS:
            array = DomainValidator.__countryCodeTLDsMinus
        elif table == ArrayType.COUNTRY_CODE_PLUS:
            array = DomainValidator.__countryCodeTLDsPlus
        elif table == ArrayType.GENERIC_MINUS:
            array = DomainValidator.__genericTLDsMinus
        elif table == ArrayType.GENERIC_PLUS:
            array = DomainValidator.__genericTLDsPlus
        elif table == ArrayType.LOCAL_MINUS:
            array = DomainValidator.__localTLDsMinus
        elif table == ArrayType.LOCAL_PLUS:
            array = DomainValidator.__localTLDsPlus
        elif table == ArrayType.GENERIC_RO:
            array = DomainValidator.__GENERIC_TLDS
        elif table == ArrayType.COUNTRY_CODE_RO:
            array = DomainValidator.__COUNTRY_CODE_TLDS
        elif table == ArrayType.INFRASTRUCTURE_RO:
            array = DomainValidator.__INFRASTRUCTURE_TLDS
        elif table == ArrayType.LOCAL_RO:
            array = DomainValidator.__LOCAL_TLDS
        else:
            raise ValueError(DomainValidator.__UNEXPECTED_ENUM_VALUE + str(table))

        return array.copy()  # Return a copy of the array

    @staticmethod
    def updateTLDOverride(table: ArrayType, tlds: List[str]) -> None:
        if DomainValidator.__inUse:
            raise RuntimeError("Can only invoke this method before calling getInstance")

        # Create a lowercase copy of the TLDs
        copy = [tld.lower() for tld in tlds]
        copy.sort()

        # Update the appropriate TLD list based on the table value
        if table == ArrayType.COUNTRY_CODE_MINUS:
            DomainValidator.__countryCodeTLDsMinus = copy
        elif table == ArrayType.COUNTRY_CODE_PLUS:
            DomainValidator.__countryCodeTLDsPlus = copy
        elif table == ArrayType.GENERIC_MINUS:
            DomainValidator.__genericTLDsMinus = copy
        elif table == ArrayType.GENERIC_PLUS:
            DomainValidator.__genericTLDsPlus = copy
        elif table == ArrayType.LOCAL_MINUS:
            DomainValidator.__localTLDsMinus = copy
        elif table == ArrayType.LOCAL_PLUS:
            DomainValidator.__localTLDsPlus = copy
        elif table in (
            ArrayType.COUNTRY_CODE_RO,
            ArrayType.GENERIC_RO,
            ArrayType.INFRASTRUCTURE_RO,
            ArrayType.LOCAL_RO,
        ):
            raise ValueError(f"Cannot update the table: {table}")
        else:
            raise ValueError(f"{DomainValidator.__UNEXPECTED_ENUM_VALUE}{table}")

    def isAllowLocal(self) -> bool:
        return self.__allowLocal

    def isValidLocalTld(self, lTld: str) -> bool:
        key = self.__chompLeadingDot(self.unicodeToASCII(lTld).lower())
        return (
            self.__arrayContains(self.__LOCAL_TLDS, key)
            or (
                self.mylocalTLDsPlus is not None
                and self.__arrayContains(self.mylocalTLDsPlus, key)
            )
        ) and not (
            self.mylocalTLDsMinus is not None
            and self.__arrayContains(self.mylocalTLDsMinus, key)
        )

    def isValidCountryCodeTld(self, ccTld: str) -> bool:
        key = self.__chompLeadingDot(self.unicodeToASCII(ccTld).lower())
        return (
            self.__arrayContains(self.__COUNTRY_CODE_TLDS, key)
            or self.__arrayContains(self.mycountryCodeTLDsPlus, key)
        ) and not self.__arrayContains(self.mycountryCodeTLDsMinus, key)

    def isValidGenericTld(self, gTld: str) -> bool:
        key = self.__chompLeadingDot(self.unicodeToASCII(gTld).lower())
        return (
            self.__arrayContains(self.__GENERIC_TLDS, key)
            or self.__arrayContains(self.__genericTLDsPlus, key)
        ) and not self.__arrayContains(self.__genericTLDsMinus, key)

    def isValidInfrastructureTld(self, iTld: str) -> bool:
        key = self.__chompLeadingDot(self.unicodeToASCII(iTld).lower())
        return self.__arrayContains(self.__INFRASTRUCTURE_TLDS, key)

    def isValidTld(self, tld: str) -> bool:
        if self.__allowLocal and self.isValidLocalTld(tld):
            return True
        return (
            self.isValidInfrastructureTld(tld)
            or self.isValidGenericTld(tld)
            or self.isValidCountryCodeTld(tld)
        )

    def isValidDomainSyntax(self, domain: str) -> bool:
        if domain is None:
            return False
        domain = self.unicodeToASCII(domain)
        if len(domain) > self.__MAX_DOMAIN_LENGTH:
            return False
        groups = self.__domainRegex.match(domain)
        return (groups is not None and len(groups) > 0) or self.__hostnameRegex.isValid(
            domain
        )

    def isValid(self, domain: str) -> bool:
        if domain is None:
            return False
        domain = self.unicodeToASCII(domain)
        if len(domain) > self.__MAX_DOMAIN_LENGTH:
            return False
        groups = self.__domainRegex.match(domain)
        if groups is not None and len(groups) > 0:
            return self.isValidTld(groups[0])
        return self.__allowLocal and self.__hostnameRegex.isValid(domain)

    def __init__(
        self, constructorId: int, items: typing.List[Item], allowLocal: bool
    ) -> None:
        if constructorId == 0:
            self.__allowLocal = allowLocal

            cc_minus = self.__countryCodeTLDsMinus
            cc_plus = self.__countryCodeTLDsPlus
            gen_minus = self.__genericTLDsMinus
            gen_plus = self.__genericTLDsPlus
            local_minus = self.__localTLDsMinus
            local_plus = self.__localTLDsPlus

            for item in items:
                copy = [value.lower() for value in item.values]
                copy.sort()

                if item.type == ArrayType.COUNTRY_CODE_MINUS:
                    cc_minus = copy
                elif item.type == ArrayType.COUNTRY_CODE_PLUS:
                    cc_plus = copy
                elif item.type == ArrayType.GENERIC_MINUS:
                    gen_minus = copy
                elif item.type == ArrayType.GENERIC_PLUS:
                    gen_plus = copy
                elif item.type == ArrayType.LOCAL_MINUS:
                    local_minus = copy
                elif item.type == ArrayType.LOCAL_PLUS:
                    local_plus = copy

            self.mycountryCodeTLDsMinus = cc_minus
            self.mycountryCodeTLDsPlus = cc_plus
            self.mygenericTLDsMinus = gen_minus
            self.mygenericTLDsPlus = gen_plus
            self.mylocalTLDsMinus = local_minus
            self.mylocalTLDsPlus = local_plus
        else:
            self.__allowLocal = allowLocal
            self.mycountryCodeTLDsMinus = self.__countryCodeTLDsMinus
            self.mycountryCodeTLDsPlus = self.__countryCodeTLDsPlus
            self.mygenericTLDsPlus = self.__genericTLDsPlus
            self.mygenericTLDsMinus = self.__genericTLDsMinus
            self.mylocalTLDsPlus = self.__localTLDsPlus
            self.mylocalTLDsMinus = self.__localTLDsMinus

    @staticmethod
    def getInstance2(allowLocal: bool, items: typing.List[Item]) -> DomainValidator:
        DomainValidator._DomainValidator__inUse = True
        return DomainValidator(0, items, allowLocal)

    @staticmethod
    def getInstance1(allowLocal: bool) -> DomainValidator:
        DomainValidator._DomainValidator__inUse = True
        if allowLocal:
            return LazyHolder._LazyHolder__DOMAIN_VALIDATOR_WITH_LOCAL
        return LazyHolder._LazyHolder__DOMAIN_VALIDATOR

    @staticmethod
    def getInstance0() -> DomainValidator:
        DomainValidator._DomainValidator__inUse = True
        return LazyHolder._LazyHolder__DOMAIN_VALIDATOR

    @staticmethod
    def __arrayContains(sortedArray: typing.List[str], key: str) -> bool:
        import bisect

        index = bisect.bisect_left(sortedArray, key)
        return index < len(sortedArray) and sortedArray[index] == key

    @staticmethod
    def __isOnlyASCII(input_: str) -> bool:
        if input_ is None:
            return True
        for char in input_:
            if ord(char) > 0x7F:  # CHECKSTYLE IGNORE MagicNumber
                return False
        return True

    def __chompLeadingDot(self, str_: str) -> str:
        if str_.startswith("."):
            return str_[1:]
        return str_


class ArrayType:

    LOCAL_MINUS: ArrayType = None

    LOCAL_PLUS: ArrayType = None

    LOCAL_RO: ArrayType = None

    INFRASTRUCTURE_RO: ArrayType = None

    COUNTRY_CODE_RO: ArrayType = None

    GENERIC_RO: ArrayType = None

    COUNTRY_CODE_MINUS: ArrayType = None

    COUNTRY_CODE_PLUS: ArrayType = None

    GENERIC_MINUS: ArrayType = None

    GENERIC_PLUS: ArrayType = None


class Item:

    values: typing.List[typing.List[str]] = None

    type: ArrayType = None

    def __init__(self, type_: ArrayType, values: List[str]) -> None:
        self.type = type_
        self.values = values  # no need to copy here


class LazyHolder:

    __DOMAIN_VALIDATOR_WITH_LOCAL: DomainValidator = None
    __DOMAIN_VALIDATOR: DomainValidator = None

    @staticmethod
    def initialize_fields() -> None:
        LazyHolder.__DOMAIN_VALIDATOR_WITH_LOCAL: DomainValidator = DomainValidator(
            1, None, True
        )

        LazyHolder.__DOMAIN_VALIDATOR: DomainValidator = DomainValidator(1, None, False)


class IDNBUGHOLDER:

    __IDN_TOASCII_PRESERVES_TRAILING_DOTS: bool = None

    @staticmethod
    def initialize_fields() -> None:
        IDNBUGHOLDER.__IDN_TOASCII_PRESERVES_TRAILING_DOTS: bool = (
            IDNBUGHOLDER.__keepsTrailingDot()
        )

    @staticmethod
    def __keepsTrailingDot() -> bool:
        input = "a."  # must be a valid name
        return input == IDN.toASCII(input)


DomainValidator.initialize_fields()

LazyHolder.initialize_fields()

IDNBUGHOLDER.initialize_fields()
