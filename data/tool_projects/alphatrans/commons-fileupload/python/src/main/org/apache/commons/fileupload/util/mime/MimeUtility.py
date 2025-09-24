from __future__ import annotations
import re
import os
from io import BytesIO
import io
import typing
from typing import *
from src.main.org.apache.commons.fileupload.util.mime.Base64Decoder import *
from src.main.org.apache.commons.fileupload.util.mime.ParseException import *
from src.main.org.apache.commons.fileupload.util.mime.QuotedPrintableDecoder import *


class MimeUtility:

    __MIME2JAVA: typing.Dict[str, str] = {}

    __LINEAR_WHITESPACE: str = " \t\r\n"
    __ENCODED_TOKEN_FINISHER: str = "?="
    __ENCODED_TOKEN_MARKER: str = "=?"
    __QUOTEDPRINTABLE_ENCODING_MARKER: str = "Q"
    __BASE64_ENCODING_MARKER: str = "B"
    __US_ASCII_CHARSET: str = "US-ASCII"

    @staticmethod
    def run_static_init():
        MimeUtility.__MIME2JAVA["iso-2022-cn"] = "ISO2022CN"
        MimeUtility.__MIME2JAVA["iso-2022-kr"] = "ISO2022KR"
        MimeUtility.__MIME2JAVA["utf-8"] = "UTF8"
        MimeUtility.__MIME2JAVA["utf8"] = "UTF8"
        MimeUtility.__MIME2JAVA["ja_jp.iso2022-7"] = "ISO2022JP"
        MimeUtility.__MIME2JAVA["ja_jp.eucjp"] = "EUCJIS"
        MimeUtility.__MIME2JAVA["euc-kr"] = "KSC5601"
        MimeUtility.__MIME2JAVA["euckr"] = "KSC5601"
        MimeUtility.__MIME2JAVA["us-ascii"] = "ISO-8859-1"
        MimeUtility.__MIME2JAVA["x-us-ascii"] = "ISO-8859-1"

    @staticmethod
    def decodeText(text: str) -> str:
        if MimeUtility.__ENCODED_TOKEN_MARKER not in text:
            return text

        offset = 0
        end_offset = len(text)

        start_white_space = -1
        end_white_space = -1

        decoded_text = []

        previous_token_encoded = False

        while offset < end_offset:
            ch = text[offset]

            if ch in MimeUtility.__LINEAR_WHITESPACE:  # whitespace found
                start_white_space = offset
                while offset < end_offset:
                    ch = text[offset]
                    if ch in MimeUtility.__LINEAR_WHITESPACE:  # whitespace found
                        offset += 1
                    else:
                        end_white_space = offset
                        break
            else:
                word_start = offset

                while offset < end_offset:
                    ch = text[offset]
                    if ch not in MimeUtility.__LINEAR_WHITESPACE:  # not white space
                        offset += 1
                    else:
                        break

                word = text[word_start:offset]
                if word.startswith(MimeUtility.__ENCODED_TOKEN_MARKER):
                    try:
                        decoded_word = MimeUtility.__decodeWord(word)

                        if not previous_token_encoded and start_white_space != -1:
                            decoded_text.append(text[start_white_space:end_white_space])
                            start_white_space = -1

                        previous_token_encoded = True
                        decoded_text.append(decoded_word)
                        continue

                    except ParseException:
                        pass

                if start_white_space != -1:
                    decoded_text.append(text[start_white_space:end_white_space])
                    start_white_space = -1

                previous_token_encoded = False
                decoded_text.append(word)

        return "".join(decoded_text)

    @staticmethod
    def __javaCharset(charset: str) -> str:
        if charset is None:
            return None

        mapped_charset = MimeUtility.__MIME2JAVA.get(charset.lower())
        if mapped_charset is None:
            return charset
        return mapped_charset

    @staticmethod
    def __decodeWord(word: str) -> str:
        if not word.startswith(MimeUtility.__ENCODED_TOKEN_MARKER):
            raise ParseException(f"Invalid RFC 2047 encoded-word: {word}")

        charset_pos = word.find("?", 2)
        if charset_pos == -1:
            raise ParseException(f"Missing charset in RFC 2047 encoded-word: {word}")

        charset = word[2:charset_pos].lower()

        encoding_pos = word.find("?", charset_pos + 1)
        if encoding_pos == -1:
            raise ParseException(f"Missing encoding in RFC 2047 encoded-word: {word}")

        encoding = word[charset_pos + 1 : encoding_pos]

        encoded_text_pos = word.find(
            MimeUtility.__ENCODED_TOKEN_FINISHER, encoding_pos + 1
        )
        if encoded_text_pos == -1:
            raise ParseException(
                f"Missing encoded text in RFC 2047 encoded-word: {word}"
            )

        encoded_text = word[encoding_pos + 1 : encoded_text_pos]

        if len(encoded_text) == 0:
            return ""

        try:
            out = io.BytesIO()

            encoded_data = encoded_text.encode(MimeUtility.__US_ASCII_CHARSET)

            if encoding == MimeUtility.__BASE64_ENCODING_MARKER:
                Base64Decoder.decode(encoded_data, out)
            elif encoding == MimeUtility.__QUOTEDPRINTABLE_ENCODING_MARKER:
                QuotedPrintableDecoder.decode(encoded_data, out)
            else:
                raise ValueError(f"Unknown RFC 2047 encoding: {encoding}")

            decoded_data = out.getvalue()
            return decoded_data.decode(MimeUtility.__javaCharset(charset))
        except Exception as e:
            raise ValueError(f"Invalid RFC 2047 encoding: {str(e)}")

    def __init__(self) -> None:
        raise NotImplementedError("This class cannot be instantiated")


MimeUtility.run_static_init()
