use crate::esteria::{Encoding, SmsClient, SmsError, SmsFlags, SmsRequest};
use chrono::DateTime;
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3_async_runtimes::tokio::future_into_py;
use std::sync::Arc;

#[pyclass]
#[derive(Clone)]
pub struct PySmsClient {
    inner: Arc<SmsClient>,
}

#[pymethods]
impl PySmsClient {
    #[new]
    fn new(api_base_url: String) -> Self {
        Self {
            inner: Arc::new(SmsClient::new(api_base_url)),
        }
    }

    #[pyo3(signature = (
        api_key,
        sender,
        number,
        text,
        time=None,
        dlr_url=None,
        expired=None,
        flag_debug=false,
        flag_nolog=false,
        flag_flash=false,
        flag_test=false,
        flag_nobl=false,
        flag_convert=false,
        user_key=None,
        use_8bit=true,
        udh=false
    ))]
    #[allow(clippy::fn_params_excessive_bools, clippy::too_many_arguments)]
    fn send_sms<'py>(
        &self,
        py: Python<'py>,
        api_key: String,
        sender: String,
        number: String,
        text: String,
        time: Option<i64>,
        dlr_url: Option<String>,
        expired: Option<i32>,
        flag_debug: bool,
        flag_nolog: bool,
        flag_flash: bool,
        flag_test: bool,
        flag_nobl: bool,
        flag_convert: bool,
        user_key: Option<String>,
        use_8bit: bool,
        udh: bool,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py(py, async move {
            let mut flags = SmsFlags::empty();
            if flag_debug {
                flags |= SmsFlags::DEBUG;
            }
            if flag_nolog {
                flags |= SmsFlags::NOLOG;
            }
            if flag_flash {
                flags |= SmsFlags::FLASH;
            }
            if flag_test {
                flags |= SmsFlags::TEST;
            }
            if flag_nobl {
                flags |= SmsFlags::NOBL;
            }
            if flag_convert {
                flags |= SmsFlags::CONVERT;
            }

            let encoding = if udh {
                Encoding::Udh
            } else if use_8bit {
                Encoding::EightBit
            } else {
                Encoding::Default
            };

            let datetime = time
                .map(|timestamp| {
                    DateTime::from_timestamp(timestamp, 0)
                        .ok_or_else(|| PyValueError::new_err("Invalid timestamp"))
                })
                .transpose()?;

            let mut request = SmsRequest::new(&api_key, &sender, &number, &text)
                .with_flags(flags)
                .with_encoding(encoding);

            if let Some(dt) = datetime {
                request = request.with_time(dt);
            }

            if let Some(url) = dlr_url.as_deref() {
                request = request.with_dlr_url(url);
            }

            if let Some(exp) = expired {
                request = request.with_expired(exp);
            }

            if let Some(key) = user_key.as_deref() {
                request = request.with_user_key(key);
            }

            client.send_sms(request).await.map_err(|e| match e {
                SmsError::SendFailed { number, message } => {
                    PyRuntimeError::new_err(format!("SMS sending failed to: {number}, {message}"))
                }
                SmsError::RequestFailed(e) => {
                    PyRuntimeError::new_err(format!("HTTP request failed: {e}"))
                }
            })
        })
    }
}

#[pyclass]
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct PyEncoding(Encoding);

#[pymethods]
impl PyEncoding {
    #[classattr]
    const DEFAULT: Self = Self(Encoding::Default);

    #[classattr]
    const EIGHT_BIT: Self = Self(Encoding::EightBit);

    #[classattr]
    const UDH: Self = Self(Encoding::Udh);
}

#[pyclass]
#[derive(Clone, Copy)]
pub struct PySmsFlags(SmsFlags);

#[pymethods]
impl PySmsFlags {
    #[new]
    fn new() -> Self {
        Self(SmsFlags::empty())
    }

    #[classattr]
    fn debug() -> Self {
        Self(SmsFlags::DEBUG)
    }

    #[classattr]
    fn nolog() -> Self {
        Self(SmsFlags::NOLOG)
    }

    #[classattr]
    fn flash() -> Self {
        Self(SmsFlags::FLASH)
    }

    #[classattr]
    fn test() -> Self {
        Self(SmsFlags::TEST)
    }

    #[classattr]
    fn nobl() -> Self {
        Self(SmsFlags::NOBL)
    }

    #[classattr]
    fn convert() -> Self {
        Self(SmsFlags::CONVERT)
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn __or__(&self, other: &Self) -> Self {
        Self(self.0 | other.0)
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn __and__(&self, other: &Self) -> Self {
        Self(self.0 & other.0)
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn __repr__(&self) -> String {
        format!("SmsFlags({:?})", self.0)
    }
}

#[pymodule]
fn esteria_api_client(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PySmsClient>()?;
    m.add_class::<PyEncoding>()?;
    m.add_class::<PySmsFlags>()?;
    Ok(())
}
