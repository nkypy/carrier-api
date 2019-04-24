// 标准查询返回
// 使用 emum 后续返回多张卡时可以显示单张的异常信息。
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum CardReply {
    // 错误信息
    Error {
        id: String,
        error_code: String,
        error_message: String,
    },
    // 状态
    Status {
        status_code: String,
        status_name: String,
        date_activated: String,
    },
    // 在线状态
    NetStatus {
        net_status_code: String,
        net_status_name: String,
    },
    // 详细信息
    Info {
        // 基本信息
        iccid: String,
        imsi: String,
        msisdn: String,
        imei: String,
        // 其他信息
        region_name: String,
        customer_name: String,
        brand: String,
    },
    // 套餐
    Plan {
        plans: Vec<CardRatePlan>,
    },
    // 使用量
    Usage {
        data_used: u64,
        sms_used: u32,
        voice_used: u32,
    },
}


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
    pub attributes: Vec<CardInfoAttribute>,
}

#[derive(Debug, Serialize)]
pub struct CardInfoAttribute {
    pub attribute_name: String,
    pub attribute_value: String,
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
