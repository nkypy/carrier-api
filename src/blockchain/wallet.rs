use std::fmt;

#[derive(Debug, Clone)]
pub struct Wallet {
    pub private_key: String,
    pub public_key: String,
}

#[derive(Debug, Clone)]
pub struct Wallets {
    pub wallets: Vec<Wallet>,
}

impl fmt::Display for Wallet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "private_key: {}, public_key: {}",
            self.private_key, self.public_key
        )
    }
}

impl Wallet {
    pub fn new() -> Wallet {
        Wallet {
            private_key: "priv".to_string(),
            public_key: "pub".to_string(),
        }
    }
}
