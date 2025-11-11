# Esteria API Client

A Rust-based client library for sending SMS messages via the Esteria API (https://esteria.eu). This project provides:

- A core Rust library for programmatic SMS sending.
- A command-line interface (CLI) for quick SMS dispatch.
- Python bindings for easy integration into Python applications.

The client supports advanced features like scheduled sending, delivery reports, flash SMS, test mode, and custom encodings.

## Features

- **Authentication**: Secure API key-based access.
- **SMS Options**:
  - Scheduled delivery.
  - Delivery report (DLR) callbacks.
  - Expiration timeouts.
  - Flags for debug, no-log, flash, test, no-blacklist, and character conversion.
- **Encodings**: Default, 8-bit, or UDH (User Data Header).
- **Error Handling**: Detailed error codes and messages from the API.
- **Cross-Platform**: Works on Linux, macOS, and Windows.
- **Python Integration**: Seamless Python API via PyO3 bindings.
- **CLI Tool**: Simple command-line usage with environment variable support.

## Installation

### For Python Users (via PyPI)

Install the Python package directly:

```bash
pip install esteria-api-client
```

This installs the Python bindings, which include the underlying Rust library.

### For Rust Users (via crates.io)

Add the library to your `Cargo.toml`:

```toml
[dependencies]
esteria-api-client = "0.1.0"  # Replace with the latest version
```

To install the CLI globally:

```bash
cargo install esteria-api-client --features cli
```

Note: The `cli` feature enables the command-line tool, and `python` enables Python bindings (used for building wheels).

### Building from Source

Clone the repository:

```bash
git clone https://github.com/yourusername/esteria-api-client.git
cd esteria-api-client
```

Build the Rust library and CLI:

```bash
cargo build --features cli
```

For Python bindings, ensure you have `maturin` installed (for building wheels):

```bash
pip install maturin
maturin develop  # Builds and installs locally for development
```

To build a PyPI wheel:

```bash
maturin build --release
```

## Usage

### Environment Variables

The client supports these env vars for convenience:

- `ESTERIA_API_BASE_URL`: API endpoint (default: `https://api.esteria.eu`).
- `ESTERIA_API_KEY`: Your API key.

### CLI Usage

The CLI tool (`esteria-api-client`) allows sending SMS from the terminal.

Basic example:

```bash
esteria-api-client \
  --api-url https://api.esteria.eu \
  --api-key YOUR_API_KEY \
  --sender "MySender" \
  --number "+1234567890" \
  --text "Hello, world!"
```

Full options:

```bash
esteria-api-client --help
```

Output:

```
Send SMS via Esteria API

Usage: esteria-api-client [OPTIONS] --api-url <API_URL> --api-key <API_KEY> --sender <SENDER> --number <NUMBER> --text <TEXT>

Options:
  -u, --api-url <API_URL>      API base URL (e.g., https://api.esteria.eu)
  -k, --api-key <API_KEY>      API key for authentication
  -s, --sender <SENDER>        Sender name or number
  -n, --number <NUMBER>        Recipient phone number (with or without +)
  -t, --text <TEXT>            Message text to send
      --time <TIME>            Schedule time (RFC3339 format, e.g., 2024-12-31T23:59:59Z)
      --dlr-url <DLR_URL>      Delivery report URL
      --expired <EXPIRED>      Expiration time in minutes
      --user-key <USER_KEY>    User key for tracking
      --debug                  Enable debug mode
      --nolog                  Disable logging
      --flash                  Send as flash SMS
      --test                   Test mode (don't actually send)
      --nobl                   No blacklist check
      --convert                Convert characters
      --encoding <ENCODING>    Encoding: default, 8bit, or udh [default: 8bit]
  -h, --help                   Print help
  -V, --version                Print version
```

On success, it prints the message ID (e.g., `Message ID: 12345`).

### Python Usage

Import and use the `SmsClient` class:

```python
import asyncio
from esteria_api_client import SmsClient, SmsFlags, PyEncoding

async def main():
    client = SmsClient("https://api.esteria.eu")

    # Basic send
    result = await client.send_sms(
        api_key="YOUR_API_KEY",
        sender="MySender",
        number="+1234567890",
        text="Hello from Python!"
    )
    print(result)  # Message ID on success

    # With options
    flags = SmsFlags.debug() | SmsFlags.flash()
    result = await client.send_sms(
        api_key="YOUR_API_KEY",
        sender="MySender",
        number="+1234567890",
        text="Scheduled flash SMS",
        time=1735689599,  # Unix timestamp
        dlr_url="https://your-callback-url.com",
        expired=60,  # Expires in 60 minutes
        flag_debug=True,
        flag_flash=True,
        user_key="my-tracking-key",
        use_8bit=False,  # Use default encoding
        udh=True  # UDH encoding
    )
    print(result)

# Run the async function
asyncio.run(main())
```

- `SmsFlags`: Bitflags for options (e.g., `SmsFlags.debug()`, `SmsFlags.flash()`). Combine with `|`.
- `PyEncoding`: Constants like `PyEncoding.DEFAULT`, `PyEncoding.EIGHT_BIT`, `PyEncoding.UDH`.
- Errors: Raises `RuntimeError` on failure with details.

Note: The `time` parameter is a Unix timestamp (seconds since epoch).

### Rust Usage (Library)

Use the `SmsClient` and `SmsRequest` structs:

```rust
use esteria_api_client::{SmsClient, SmsRequest, SmsFlags, Encoding};
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SmsClient::new("https://api.esteria.eu".to_string());

    let request = SmsRequest::new(
        "YOUR_API_KEY",
        "MySender",
        "+1234567890",
        "Hello from Rust!"
    )
    .with_flags(SmsFlags::DEBUG | SmsFlags::FLASH)
    .with_encoding(Encoding::Udh)
    .with_time(Utc::now())  // Schedule for now (or future)
    .with_dlr_url("https://your-callback-url.com")
    .with_expired(60)
    .with_user_key("my-tracking-key");

    match client.send_sms(request).await {
        Ok(code) => println!("Message ID: {}", code),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
```

- `SmsFlags`: Bitflags (e.g., `SmsFlags::DEBUG`).
- `Encoding`: Enum for `Default`, `EightBit`, `Udh`.
- Errors: `SmsError` variants for handling.

## API Error Codes

If sending fails, the client returns detailed errors based on Esteria's response codes:

- 1: System internal error
- 2: Missing parameter
- 3: Unable to authenticate
- ... (see full list in `esteria.rs`)

## Developer Notes

- **Features**: Enable `cli` for the command-line tool or `python` for bindings via Cargo.
- **Dependencies**: Uses `reqwest` for HTTP, `chrono` for dates, `clap` for CLI, `pyo3` for Python, and `bitflags` for flags.
- **Logging**: Uses `env_logger` (init in CLI).
- **Testing**: Run `cargo test`. Use `--flag-test` for API test mode.
- **Contributing**: Pull requests welcome! Focus on bug fixes, features, or docs.
- **License**: GPLv3.

For issues or suggestions, open a GitHub issue.

---

This project is not affiliated with Esteria.eu. Ensure you have an active Esteria account and API key.
