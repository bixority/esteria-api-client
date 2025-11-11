use chrono::{DateTime, Utc};
use reqwest::Client;
use std::collections::HashMap;
use thiserror::Error;

/// Error types for SMS operations
#[derive(Error, Debug)]
pub enum SmsError {
    #[error("SMS sending failed to: {number}, {message}")]
    SendFailed { number: String, message: String },
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
}

bitflags::bitflags! {
    /// Flags for SMS sending options
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    pub struct SmsFlags: u32 {
        /// Enable debug mode
        const DEBUG   = 0b0000_0001;
        /// Disable logging
        const NOLOG   = 0b0000_0010;
        /// Send as flash SMS
        const FLASH   = 0b0000_0100;
        /// Test mode (don't actually send)
        const TEST    = 0b0000_1000;
        /// No blacklist check
        const NOBL    = 0b0001_0000;
        /// Convert characters
        const CONVERT = 0b0010_0000;
    }
}

/// SMS encoding options
#[derive(Debug, Clone, Copy)]
pub enum Encoding {
    /// Default encoding
    Default,
    /// 8-bit encoding
    EightBit,
    /// User Data Header encoding
    Udh,
}

/// SMS API client for Esteria
pub struct SmsClient {
    api_base_url: String,
    client: Client,
}

/// Request structure for sending SMS
pub struct SmsRequest<'a> {
    pub api_key: &'a str,
    pub sender: &'a str,
    pub number: &'a str,
    pub text: &'a str,
    pub time: Option<DateTime<Utc>>,
    pub dlr_url: Option<&'a str>,
    pub expired: Option<i32>,
    pub flags: SmsFlags,
    pub user_key: Option<&'a str>,
    pub encoding: Encoding,
}

impl<'a> SmsRequest<'a> {
    /// Create a new SMS request with required parameters
    #[must_use]
    pub fn new(api_key: &'a str, sender: &'a str, number: &'a str, text: &'a str) -> Self {
        Self {
            api_key,
            sender,
            number,
            text,
            time: None,
            dlr_url: None,
            expired: None,
            flags: SmsFlags::empty(),
            user_key: None,
            encoding: Encoding::EightBit,
        }
    }

    /// Set scheduled delivery time
    #[must_use]
    pub fn with_time(mut self, time: DateTime<Utc>) -> Self {
        self.time = Some(time);
        self
    }

    /// Set delivery report URL
    #[must_use]
    pub fn with_dlr_url(mut self, dlr_url: &'a str) -> Self {
        self.dlr_url = Some(dlr_url);
        self
    }

    /// Set expiration time in minutes
    #[must_use]
    pub fn with_expired(mut self, expired: i32) -> Self {
        self.expired = Some(expired);
        self
    }

    /// Set SMS flags
    #[must_use]
    pub fn with_flags(mut self, flags: SmsFlags) -> Self {
        self.flags = flags;
        self
    }

    /// Set user key for tracking
    #[must_use]
    pub fn with_user_key(mut self, user_key: &'a str) -> Self {
        self.user_key = Some(user_key);
        self
    }

    /// Set encoding
    #[must_use]
    pub fn with_encoding(mut self, encoding: Encoding) -> Self {
        self.encoding = encoding;
        self
    }
}

impl SmsClient {
    /// Create a new SMS client with the given API base URL
    #[must_use]
    pub fn new(api_base_url: String) -> Self {
        Self {
            api_base_url,
            client: Client::new(),
        }
    }

    /// Send an SMS message
    ///
    /// Returns the message ID on success (> 100)
    ///
    /// # Errors
    ///
    /// Returns `SmsError::SendFailed` if the API returns an error code (< 100)
    /// or `SmsError::RequestFailed` if the HTTP request fails
    pub async fn send_sms(&self, request: SmsRequest<'_>) -> Result<i32, SmsError> {
        let mut params: HashMap<&str, String> = HashMap::new();

        params.insert("api-key", request.api_key.to_string());
        params.insert("sender", request.sender.to_string());
        params.insert("number", request.number.trim_start_matches('+').to_string());
        params.insert("text", request.text.to_string());

        if let Some(time) = request.time {
            params.insert("time", time.format("%Y-%m-%dT%H:%M:%S").to_string());
        }

        if let Some(dlr_url) = request.dlr_url {
            params.insert("dlr-url", dlr_url.to_string());
        }

        if let Some(expired) = request.expired {
            params.insert("expired", expired.to_string());
        }

        if request.flags.contains(SmsFlags::DEBUG) {
            params.insert("flag-debug", "1".to_string());
        }

        if request.flags.contains(SmsFlags::NOLOG) {
            params.insert("flag-nolog", "3".to_string());
        }

        if request.flags.contains(SmsFlags::FLASH) {
            params.insert("flag-flash", "1".to_string());
        }

        if request.flags.contains(SmsFlags::TEST) {
            params.insert("flag-test", "1".to_string());
        }

        if request.flags.contains(SmsFlags::NOBL) {
            params.insert("flag-nobl", "1".to_string());
        }

        if request.flags.contains(SmsFlags::CONVERT) {
            params.insert("flag-convert", "1".to_string());
        }

        if let Some(user_key) = request.user_key {
            params.insert("user-key", user_key.to_string());
        }

        match request.encoding {
            Encoding::Udh => {
                params.insert("udh", "1".to_string());
                params.insert("coding", "1".to_string());
            }
            Encoding::EightBit => {
                params.insert("coding", "1".to_string());
            }
            Encoding::Default => {}
        }

        let url = format!("{}/send", self.api_base_url);
        let response = self.client.get(&url).query(&params).send().await?;

        let resp_text = response.text().await?;

        let result = resp_text.trim().parse::<i32>().ok();

        if let Some(code) = result {
            if code > 100 {
                return Ok(code);
            }

            let error_msg = get_response_code_message(code);
            log::error!("SMS sending failed to: {}, {}", request.number, error_msg);

            return Err(SmsError::SendFailed {
                number: request.number.to_string(),
                message: error_msg.to_string(),
            });
        }

        log::error!("SMS sending failed to: {}, unknown error", request.number);
        Err(SmsError::SendFailed {
            number: request.number.to_string(),
            message: "unknown error".to_string(),
        })
    }
}

fn get_response_code_message(code: i32) -> &'static str {
    match code {
        1 => "system internal error",
        2 => "missing PARAM_NAME parameter",
        3 => "unable to authenticate",
        4 => "IP ADDRESS is not allowed",
        5 => "invalid SENDER parameter",
        6 => "SENDER is not allowed",
        7 => "invalid NUMBER parameter",
        8 => "invalid CODING parameter",
        9 => "unable to convert TEXT",
        10 => "length of UDH and TEXT too long",
        11 => "empty TEXT parameter",
        12 => "invalid TIME parameter",
        13 => "invalid EXPIRED parameter",
        14 => "invalid DLR-URL parameter",
        15 => "Invalid FLAG-FLASH parameter",
        16 => "invalid FLAG-NOLOG parameter",
        17 => "invalid FLAG-TEST parameter",
        18 => "invalid FLAG-NOBL parameter",
        19 => "invalid FLAG-CONVERT parameter",
        _ => "unknown error",
    }
}
