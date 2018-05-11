#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: Vec<u8>,
    pub tx_in: Vec<TxInput>,
    pub tx_out: Vec<TxOutput>,
}

#[derive(Debug, Clone)]
pub struct TxInput {
    pub tx_id: Vec<u8>,
    pub out: i32,
    pub signature: String,
}

#[derive(Debug, Clone)]
pub struct TxOutput {
    pub value: i32,
    pub pub_key: String,
}

impl Transaction {
    pub fn verify() -> Result<&'static str, &'static str> {
        Ok("成功")
    }
}
