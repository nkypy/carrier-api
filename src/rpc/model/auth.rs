#[derive(Debug, Serialize, Deserialize)]
pub struct SigninRequest {
    pub username: &'static str,
    pub password: &'static str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SigninReply {
    pub token: &'static str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecoverRequest {
    pub email: &'static str,
}
