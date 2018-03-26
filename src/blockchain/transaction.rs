#[derive(Debug, Clone)]
pub struct Transaction {
    id: Vec<u8>,
    tx_in: Vec<TxInput>,
    tx_out: Vec<TxOutput>,
}

#[derive(Debug, Clone)]
pub struct TxInput{
    tx_id: Vec<u8>,
    from: String,
    out: String,
}

#[derive(Debug, Clone)]
pub struct TxOutput{
    value: i32,
    pub_key: String,
}