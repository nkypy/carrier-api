#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub error_code: i64,
    pub error_message: &'static str,
}

impl Error {
    pub fn new(error_code: i64, error_message: &'static str) -> Error {
        Error {
            error_code: error_code,
            error_message: error_message,
        }
    }
    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
    pub fn from_str(para: &'static str) -> Error {
        let out: Error = serde_json::from_str(para).unwrap();
        out
    }
}

pub const NOT_FOUND: Error = Error {
    error_code: 100404,
    error_message: "Not Found",
};

pub const INTERNAL_SERVER_ERROR: Error = Error {
    error_code: 100500,
    error_message: "Internal Server Error",
};

pub const TOKEN_VERIFY_FAILED: Error = Error {
    error_code: 101001,
    error_message: "Token 验证失败！",
};
