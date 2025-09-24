from __future__ import annotations
import re
from abc import ABC
import io
import typing
from typing import *
from src.main.org.apache.commons.fileupload.FileItemStream import *
from src.main.org.apache.commons.fileupload.FileUploadException import *


class FileItemIterator(ABC):

    def next_(self) -> FileItemStream:
        raise FileUploadException("Method not implemented")

    def hasNext(self) -> bool:
        raise NotImplementedError("This method should be implemented by subclasses")
