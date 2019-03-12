// 标准状态查询返回
#[derive(Debug, Serialize, Deserialize)]
pub struct CardStatus<'a> {
    pub status_code: &'a str,
    pub status_name: &'a str,
    pub date_created: &'a str,
}

// 标准信息查询返回
#[derive(Debug, Serialize, Deserialize)]
pub struct CardInfo<'a> {
    pub iccid: &'a str,
    pub imsi: &'a str,
    pub msisdn: &'a str,
    pub imei: &'a str,
    pub region_name: &'a str,
    pub customer_name: &'a str,
    pub brand: &'a str,
}