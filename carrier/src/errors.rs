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
    fn from(err: String) -> Self {
        Error {
            err_code: "10000001".to_owned(),
            err_msg: err,
        }
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error {
            err_code: "10000002".to_owned(),
            err_msg: err.to_owned(),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(_err: reqwest::Error) -> Self {
        Error {
            err_code: "20000003".to_owned(),
            err_msg: "请求超时或返回结果获取失败".to_owned(),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(_err: serde_json::Error) -> Self {
        Error {
            err_code: "20000004".to_owned(),
            err_msg: "serde 解析失败".to_owned(),
        }
    }
}
