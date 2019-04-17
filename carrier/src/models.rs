// 标准状态查询返回
#[derive(Debug, Serialize)]
pub struct CardStatus {
    pub status_code: String,
    pub status_name: String,
    pub date_activated: String,
}

// 标准网络状态查询返回
#[derive(Debug, Serialize)]
pub struct CardNetStatus {
    pub net_status_code: String,
    pub net_status_name: String,
}

// 标准信息查询返回
#[derive(Debug, Default, Serialize)]
pub struct CardInfo {
    pub iccid: String,
    pub imsi: String,
    pub msisdn: String,
    pub imei: String,
    pub region_name: String,
    pub customer_name: String,
    pub brand: String,
    pub attrs: Vec<CardInfoAttr>,
}

#[derive(Debug, Serialize)]
pub struct CardInfoAttr {
    pub attr_name: String,
    pub attr_value: String,
}

// 标准套餐查询返回
#[derive(Debug, Serialize)]
pub struct CardRatePlan {
    pub plan_code: String,
    pub plan_name: String,
}

// 标准用量查询返回
#[derive(Debug, Serialize)]
pub struct CardUsage {
    pub data_used: u64,
    pub sms_used: u32,
    pub voice_used: u32,
}
