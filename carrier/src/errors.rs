use std::convert::From;
use std::fmt;

use reqwest;
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub err_code: String,
    pub err_msg: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            r#"Error {{ err_code: "{}", err_msg: "{}" }}"#,
            self.err_code, self.err_msg
        )
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Error {
            err_code: "10000001".to_owned(),
            err_msg: e,
        }
    }
}

impl From<&str> for Error {
    fn from(e: &str) -> Self {
        Error {
            err_code: "10000002".to_owned(),
            err_msg: e.to_owned(),
        }
    }
}

impl From<(&str, &str)> for Error {
    fn from(e: (&str, &str)) -> Self {
        Error {
            err_code: e.0.to_owned(),
            err_msg: e.1.to_owned(),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        if e.is_timeout() {
            return Error {
                err_code: "20000001".to_owned(),
                err_msg: "运营商接口请求超时。".to_owned(),
            };
        }
        if e.is_serialization() {
            return Error {
                err_code: "20000002".to_owned(),
                err_msg: "运营商接口返回解析失败。".to_owned(),
            };
        }
        if e.is_client_error() {
            return Error {
                err_code: "20000004".to_owned(),
                err_msg: "运营商接口返回 4xx 异常。".to_owned(),
            };
        }
        if e.is_server_error() {
            return Error {
                err_code: "20000005".to_owned(),
                err_msg: "运营商接口返回 5xx 异常。".to_owned(),
            };
        }
        Error {
            err_code: "20999999".to_owned(),
            err_msg: "运营商接口请求发生未知异常。".to_owned(),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        dbg!(e);
        Error {
            err_code: "21000001".to_owned(),
            err_msg: "返回的数据格式异常，导致解析失败。".to_owned(),
        }
    }
}

impl From<chrono::ParseError> for Error {
    fn from(e: chrono::ParseError) -> Self {
        dbg!(e);
        Error {
            err_code: "22000001".to_owned(),
            err_msg: "请求日期格式不符合要求，请按照 200601 格式输入日期"
                .to_owned(),
        }
    }
}
