from __future__ import annotations

from typing import Any, Optional

class SmsClient:
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

class Encoding:
    DEFAULT: Encoding
    EIGHT_BIT: Encoding
    UDH: Encoding

class SmsFlags:
    def __init__(self) -> None: ...
    @staticmethod
    def debug() -> SmsFlags: ...
    @staticmethod
    def nolog() -> SmsFlags: ...
    @staticmethod
    def flash() -> SmsFlags: ...
    @staticmethod
    def test() -> SmsFlags: ...
    @staticmethod
    def nobl() -> SmsFlags: ...
    @staticmethod
    def convert() -> SmsFlags: ...
    def __or__(self, other: SmsFlags) -> SmsFlags: ...
    def __and__(self, other: SmsFlags) -> SmsFlags: ...
