#[derive(Debug, Clone)]
pub struct Wallet {
    pub private_key: String,
    pub public_key: String,
}

#[derive(Debug, Clone)]
pub struct Wallets {
    pub wallets: Vec<Wallet>,
}

impl Wallet {
    pub fn new() -> Wallet {
        Wallet {
            private_key: "priv".to_string(),
            public_key: "pub".to_string(),
        }
    }
}
