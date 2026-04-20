from __future__ import annotations

from typing import Any, Optional

class PySmsClient:
    def __init__(self, api_base_url: str) -> None: ...
    def send_sms(
        self,
        api_key: str,
        sender: str,
        number: str,
        text: str,
        time: Optional[int] = ...,
        dlr_url: Optional[str] = ...,
        expired: Optional[int] = ...,
        flag_debug: bool = ...,
        flag_nolog: bool = ...,
        flag_flash: bool = ...,
        flag_test: bool = ...,
        flag_nobl: bool = ...,
        flag_convert: bool = ...,
        user_key: Optional[str] = ...,
        use_8bit: bool = ...,
        udh: bool = ...,
    ) -> Any: ...

class PyEncoding:
    DEFAULT: PyEncoding
    EIGHT_BIT: PyEncoding
    UDH: PyEncoding

class PySmsFlags:
    def __init__(self) -> None: ...
    @staticmethod
    def debug() -> PySmsFlags: ...
    @staticmethod
    def nolog() -> PySmsFlags: ...
    @staticmethod
    def flash() -> PySmsFlags: ...
    @staticmethod
    def test() -> PySmsFlags: ...
    @staticmethod
    def nobl() -> PySmsFlags: ...
    @staticmethod
    def convert() -> PySmsFlags: ...
    def __or__(self, other: PySmsFlags) -> PySmsFlags: ...
    def __and__(self, other: PySmsFlags) -> PySmsFlags: ...
