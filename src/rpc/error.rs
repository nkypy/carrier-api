#[derive(Serialize, Deserialize)]
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
}

pub const ERROR_NOT_FOUND: Error = Error {
    error_code: 404,
    error_message: "Not Found",
};

pub const ERROR_TOKEN_VERIFY_FAILED: Error = Error {
    error_code: 100001,
    error_message: "Token 验证失败！",
};
