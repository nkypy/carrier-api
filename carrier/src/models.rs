use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref STATUS_NAME_HASHMAP: HashMap<&'static str, HashMap<&'static str, &'static str>> = {
        // 中国电信
        let ctm: HashMap<&'static str, &'static str> = [
            ("1", "在用"),
            ("2", "用户报停"),
            ("3", "用户拆机"),
            ("5", "欠停(双向)"),
            ("6", "欠停(单向)"),
            ("7", "违章停机"),
            ("8", "挂失"),
            ("19", "活卡待激活"),
            ("120000", "停机"),
            ("100001", "已激活"),
            ("140003", "未激活"),
            ("150001", "未实名制违规停机"),
            ("99999", "销户"),
        ].iter().cloned().collect();
        // 中国联通
        let cum: HashMap<&'static str, &'static str> = [
            ("INVENTORY", "库存"),
            ("ACTIVATION_READY", "可激活"),
            ("ACTIVATED", "已激活"),
            ("DEACTIVATED", "已停用"),
            ("RETIRED", "已失效"),
        ].iter().cloned().collect();
        // 中国移动
        let cmm: HashMap<&'static str, &'static str> = [
            ("1", "正常"),
            ("2", "待激活"),
            ("3", "停机"),
            ("4", "销户"),
            ("8", "全停"),
            ("9", "全停"),
        ].iter().cloned().collect();
        // 江苏移动 和中国移动一样
        let jsmm = cmm.clone();
        // 广东移动
        let gdmm: HashMap<&'static str, &'static str> = [
            ("test", "测试期"),
            ("silent", "沉默期"),
            ("inventory", "库存期"),
            ("normal", "正使用"),
            ("stop", "停机"),
            ("preclose", "销户"),
            ("bespeakClose", "预约销户"),
            ("others", "其他"),
        ].iter().cloned().collect();
        let m: HashMap<&'static str, HashMap<&'static str, &'static str>> = [
            ("china_telecom", ctm),
            ("china_unicom", cum),
            ("china_mobile", cmm),
            ("jiangsu_mobile", jsmm),
            ("guangdong_mobile", gdmm),
        ].iter().cloned().collect();
        m
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

// 标准用量查询返回
#[derive(Debug, Serialize, Deserialize)]
pub struct CardUsage {
    pub data_total: String,
    pub data_used: String,
    pub data_left: String,
}
