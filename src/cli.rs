use crate::esteria::{Encoding, SmsClient, SmsFlags, SmsRequest};
use chrono::{DateTime, Utc};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "esteria-api-client")]
#[command(author, version, about = "Send SMS via Esteria API", long_about = None)]
#[allow(clippy::struct_excessive_bools)]
struct Cli {
    /// API base URL (e.g., <https://api.esteria.eu>)
    #[arg(short = 'u', long, env = "ESTERIA_API_BASE_URL")]
    api_url: String,

    /// API key for authentication
    #[arg(short = 'k', long, env = "ESTERIA_API_KEY")]
    api_key: String,

    /// Sender name or number
    #[arg(short = 's', long)]
    sender: String,

    /// Recipient phone number (with or without +)
    #[arg(short = 'n', long)]
    number: String,

    /// Message text to send
    #[arg(short = 't', long)]
    text: String,

    /// Schedule time (RFC3339 format, e.g., 2024-12-31T23:59:59Z)
    #[arg(long)]
    time: Option<String>,

    /// Delivery report URL
    #[arg(long)]
    dlr_url: Option<String>,

    /// Expiration time in minutes
    #[arg(long)]
    expired: Option<i32>,

    /// User key for tracking
    #[arg(long)]
    user_key: Option<String>,

    /// Enable debug mode
    #[arg(long)]
    debug: bool,

    /// Disable logging
    #[arg(long)]
    nolog: bool,

    /// Send as flash SMS
    #[arg(long)]
    flash: bool,

    /// Test mode (don't actually send)
    #[arg(long)]
    test: bool,

    /// No blacklist check
    #[arg(long)]
    nobl: bool,

    /// Convert characters
    #[arg(long)]
    convert: bool,

    /// Encoding: default, 8bit, or udh
    #[arg(long, default_value = "8bit")]
    encoding: String,
}

/// Run the CLI application
///
/// # Errors
///
/// Returns an error if:
/// - Invalid timestamp format is provided
/// - SMS sending fails (API error or network issue)
/// - Any other IO or parsing error occurs
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let cli = Cli::parse();

    let client = SmsClient::new(cli.api_url);

    let mut flags = SmsFlags::empty();
    if cli.debug {
        flags |= SmsFlags::DEBUG;
    }
    if cli.nolog {
        flags |= SmsFlags::NOLOG;
    }
    if cli.flash {
        flags |= SmsFlags::FLASH;
    }
    if cli.test {
        flags |= SmsFlags::TEST;
    }
    if cli.nobl {
        flags |= SmsFlags::NOBL;
    }
    if cli.convert {
        flags |= SmsFlags::CONVERT;
    }

    let encoding = match cli.encoding.to_lowercase().as_str() {
        "default" => Encoding::Default,
        "8bit" => Encoding::EightBit,
        "udh" => Encoding::Udh,
        _ => {
            eprintln!("Invalid encoding '{}'. Using 8bit.", cli.encoding);
            Encoding::EightBit
        }
    };

    let time = if let Some(time_str) = cli.time {
        Some(DateTime::parse_from_rfc3339(&time_str)?.with_timezone(&Utc))
    } else {
        None
    };

    let mut request = SmsRequest::new(&cli.api_key, &cli.sender, &cli.number, &cli.text)
        .with_flags(flags)
        .with_encoding(encoding);

    if let Some(t) = time {
        request = request.with_time(t);
    }

    if let Some(url) = cli.dlr_url.as_deref() {
        request = request.with_dlr_url(url);
    }

    if let Some(exp) = cli.expired {
        request = request.with_expired(exp);
    }

    if let Some(key) = cli.user_key.as_deref() {
        request = request.with_user_key(key);
    }

    println!("Sending SMS to {}...", cli.number);

    match client.send_sms(request).await {
        Ok(code) => {
            println!("✓ SMS sent successfully!");
            println!("Message ID: {code}");
            Ok(())
        }
        Err(e) => {
            eprintln!("✗ Failed to send SMS: {e}");
            Err(Box::new(e))
        }
    }
}
