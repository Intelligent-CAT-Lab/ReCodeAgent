from __future__ import annotations
import re
import io
from src.main.org.apache.commons.validator.routines.CodeValidator import *
from src.main.org.apache.commons.validator.routines.checkdigit.CheckDigit import *
from src.main.org.apache.commons.validator.routines.checkdigit.CheckDigitException import *
from src.main.org.apache.commons.validator.routines.checkdigit.EAN13CheckDigit import *
from src.main.org.apache.commons.validator.routines.checkdigit.ISBN10CheckDigit import *


class ISBNValidator:

    def isValid(self, isbn: str) -> bool:
        return ISBNValidator.getInstance0().isValidISBN10(isbn)

    def __init__(self) -> None:
        super().__init__()
