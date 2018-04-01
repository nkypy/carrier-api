#[derive(Debug, Clone)]
pub struct Wallet {
    pub private_key: String,
    pub public_key: String,
}

#[derive(Debug, Clone)]
pub struct Wallets {
    pub wallets: Vec<Wallet>,
}