use {lazy_static::lazy_static, std::collections::HashMap};

lazy_static! {
    pub static ref STATUS_NAME_HASHMAP: HashMap<&'static str, HashMap<&'static str, &'static str>> = {
        let mut sm = HashMap::new();
        // 中国电信
        let mut m = HashMap::new();
        m.insert("1", "在用");
        m.insert("2", "用户报停");
        m.insert("3", "用户拆机");
        m.insert("5", "欠停(双向)");
        m.insert("6", "欠停(单向)");
        m.insert("7", "违章停机");
        m.insert("8", "挂失");
        m.insert("19", "活卡待激活");
        m.insert("120000", "停机");
        m.insert("100001", "已激活");
        m.insert("140003", "未激活");
        m.insert("150001", "未实名制违规停机");
        m.insert("99999", "销户");
        sm.insert("china_telecom", m);
        // 中国联通
        let mut m = HashMap::new();
        m.insert("INVENTORY", "库存");
        m.insert("ACTIVATION_READY", "可激活");
        m.insert("ACTIVATED", "已激活");
        m.insert("DEACTIVATED", "已停用");
        m.insert("RETIRED", "已失效");
        sm.insert("china_unicom", m);
        // 中国移动
        let mut m = HashMap::new();
        m.insert("1", "正常");
        m.insert("2", "待激活");
        m.insert("3", "停机");
        m.insert("4", "销户");
        m.insert("8", "全停");
        m.insert("9", "全停");
        let jsm = m.clone();
        sm.insert("china_mobile", m);
        // 江苏移动 和中国移动一样
        sm.insert("jiangsu_mobile", jsm);
        // 广东移动
        let mut m = HashMap::new();
        m.insert("test", "测试期");
        m.insert("silent", "沉默期");
        m.insert("inventory", "库存期");
        m.insert("normal", "正使用");
        m.insert("stop", "停机");
        m.insert("preclose", "销户");
        m.insert("bespeakClose", "预约销户");
        m.insert("others", "其他");
        sm.insert("guangdong_mobile", m);
        sm
    };
}

// 标准状态查询返回
#[derive(Debug, Serialize, Deserialize)]
pub struct CardStatus {
    pub status_code: String,
    pub status_name: String,
    pub date_activated: String,
}

// 标准网络状态查询返回
#[derive(Debug, Serialize, Deserialize)]
pub struct CardNetStatus {
    pub net_status_code: String,
    pub net_status_name: String,
}

// 标准信息查询返回
#[derive(Debug, Serialize, Deserialize)]
pub struct CardInfo {
    pub iccid: String,
    pub imsi: String,
    pub msisdn: String,
    pub imei: String,
    pub region_name: String,
    pub customer_name: String,
    pub brand: String,
}
