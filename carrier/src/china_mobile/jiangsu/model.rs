use crate::{CardInfo, CardStatus, Result};

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename = "operation_in")]
// pub struct CardRequest {
//     #[serde(rename = "$value")]
//     pub items: Vec<CardRequestInfo>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub enum CardRequestInfo {
//     ProcessCode(String),
//     AppId(String),
//     AccessToken(String),
//     Sign(String),
//     VerifyCode(String),
//     ReqType(String),
//     TerminalId(String),
//     AcceptSeq(String),
//     ReqSeq(String),
//     ReqTime(String),
//     Content {
//         groupid: String,
//         ddr_city: String,
//         iccid: String,
//         msisdn: String,
//         telnum: String,
//         service_number: String,
//         cycle: String,
//         oprtype: String,
//         reason: String,
//         service: String,
//         // #[serde(rename = "SUB_SERVICE_STATUS")]
//         sub_service_status: String,
//     },
// }

// impl CardRequest {
//     pub fn new(
//         process_code: &str,
//         app_id: &str,
//         access_token: &str,
//         sign: &str,
//         verify_code: &str,
//         req_type: &str,
//         terminal_id: &str,
//         accept_seq: &str,
//         req_seq: &str,
//         req_time: &str,
//         groupid: &str,
//         ddr_city: &str,
//         iccid: &str,
//         msisdn: &str,
//         telnum: &str,
//         service_number: &str,
//         cycle: &str,
//         oprtype: &str,
//         reason: &str,
//         service: &str,
//         sub_service_status: &str,
//     ) -> Self {
//         CardRequest {
//             items: vec![
//                 CardRequestInfo::ProcessCode(process_code.to_owned()),
//                 CardRequestInfo::AppId(app_id.to_owned()),
//                 CardRequestInfo::AccessToken(access_token.to_owned()),
//                 CardRequestInfo::Sign(sign.to_owned()),
//                 CardRequestInfo::VerifyCode(verify_code.to_owned()),
//                 CardRequestInfo::ReqType(req_type.to_owned()),
//                 CardRequestInfo::TerminalId(terminal_id.to_owned()),
//                 CardRequestInfo::AcceptSeq(accept_seq.to_owned()),
//                 CardRequestInfo::ReqSeq(req_seq.to_owned()),
//                 CardRequestInfo::ReqTime(req_time.to_owned()),
//                 CardRequestInfo::Content {
//                     groupid: groupid.to_owned(),
//                     ddr_city: ddr_city.to_owned(),
//                     iccid: iccid.to_owned(),
//                     msisdn: msisdn.to_owned(),
//                     telnum: telnum.to_owned(),
//                     service_number: service_number.to_owned(),
//                     cycle: cycle.to_owned(),
//                     oprtype: oprtype.to_owned(),
//                     reason: reason.to_owned(),
//                     service: service.to_owned(),
//                     sub_service_status: sub_service_status.to_owned(),
//                 },
//             ],
//         }
//     }
// }

// 江苏移动请求格式
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "operation_in")]
// #[serde(rename_all = "camelCase")]
pub struct CardRequest {
    pub process_code: String,
    pub app_id: String,
    pub access_token: String,
    pub sign: String,
    pub verify_code: String,
    pub req_type: String,
    pub terminal_id: String,
    pub accept_seq: String,
    pub req_seq: String,
    pub req_time: String,
    #[serde(rename = "$value")]
    // #[serde(flatten)]
    pub content: CardRequestContent,
}

#[derive(Debug, Serialize, Deserialize)]
// #[serde(rename = "content")]
pub struct CardRequestContent {
    pub groupid: String,
    pub ddr_city: String,
    pub iccid: String,
    pub msisdn: String,
    pub telnum: String,
    pub service_number: String,
    pub cycle: String,
    pub oprtype: String,
    pub reason: String,
    pub service: String,
    #[serde(rename = "SUB_SERVICE_STATUS")]
    pub sub_service_status: String,
}

impl CardRequest {
    pub fn new(
        process_code: &str,
        app_id: &str,
        access_token: &str,
        sign: &str,
        verify_code: &str,
        req_type: &str,
        terminal_id: &str,
        accept_seq: &str,
        req_seq: &str,
        req_time: &str,
        groupid: &str,
        ddr_city: &str,
        iccid: &str,
        msisdn: &str,
        telnum: &str,
        service_number: &str,
        cycle: &str,
        oprtype: &str,
        reason: &str,
        service: &str,
        sub_service_status: &str,
    ) -> String {
        format!("<operation_in><process_code>{}</process_code><app_id>{}</app_id><access_token>{}</access_token><sign>{}</sign><verify_code>{}</verify_code><req_type>{}</req_type><terminal_id>{}</terminal_id><accept_seq>{}</accept_seq><req_seq>{}</req_seq><req_time>{}</req_time><content><groupid>{}</groupid><ddr_city>{}</ddr_city><iccid>{}</iccid><msisdn>{}</msisdn><telnum>{}</telnum><service_number>{}</service_number><cycle>{}</cycle><oprtype>{}</oprtype><reason>{}</reason><service>{}</service><SUB_SERVICE_STATUS>{}</SUB_SERVICE_STATUS></content></operation_in>",
            process_code, app_id, access_token, sign, verify_code, req_type,
            terminal_id, accept_seq, req_seq, req_time, groupid, ddr_city,
            iccid, msisdn, telnum, service_number, cycle, oprtype, reason,
            service, sub_service_status)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "operation_out")]
struct CardReply<'a> {
    req_seq: &'a str,
    resp_seq: &'a str,
    resp_time: &'a str,
    emergency_status: &'a str,
    #[serde(rename = "response")]
    result: CardReplyResult<'a>,
    #[serde(rename = "content")]
    content: CardReplyContent<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CardReplyResult<'a> {
    resp_type: i64,
    resp_code: &'a str,
    resp_result: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
struct CardReplyContent<'a> {
    ret_code: &'a str,
    ret_msg: &'a str,
    ret_desc: &'a str,
    order_id: &'a str,
    msisdn: &'a str,
    imsi: &'a str,
    iccid: &'a str,
    status: &'a str,
    username: &'a str,
    brand: &'a str,
    region: &'a str,
}
