from __future__ import annotations

from typing import Optional

class SmsClient:
    def __init__(self, api_base_url: str = ...) -> None: ...
    async def send_sms(
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
    ) -> int: ...
