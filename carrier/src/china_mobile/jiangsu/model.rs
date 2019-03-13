// 江苏移动请求格式
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "operation_in")]
pub struct CardRequest<'a> {
    pub process_code: &'a str,
    pub app_id: &'a str,
    pub access_token: &'a str,
    pub sign: &'a str,
    pub verify_code: &'a str,
    pub req_type: &'a str,
    pub terminal_id: &'a str,
    pub accept_seq: &'a str,
    pub req_seq: &'a str,
    pub req_time: &'a str,
    pub content: CardRequestContent<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardRequestContent<'a> {
    pub groupid: &'a str,
    pub ddr_city: &'a str,
    pub iccid: &'a str,
    pub msisdn: &'a str,
    pub telnum: &'a str,
    pub service_number: &'a str,
    pub cycle: &'a str,
    pub oprtype: &'a str,
    pub reason: &'a str,
    pub service: &'a str,
    #[serde(rename = "SUB_SERVICE_STATUS")]
    pub sub_service_status: &'a str,
}

impl<'a> CardRequest<'a> {
    pub fn new(
        process_code: &'a str, app_id: &'a str, access_token: &'a str,
        sign: &'a str, verify_code: &'a str, req_type: &'a str,
        terminal_id: &'a str, accept_seq: &'a str, req_seq: &'a str,
        req_time: &'a str,
        groupid: &'a str, ddr_city: &'a str, iccid: &'a str, msisdn: &'a str,
        telnum: &'a str, service_number: &'a str, cycle: &'a str,
        oprtype: &'a str, reason: &'a str, service: &'a str,
        sub_service_status: &'a str) -> CardRequest<'a> {
        CardRequest{
                process_code: process_code,
                app_id: app_id,
                access_token: access_token,
                sign: sign,
                verify_code: verify_code,
                req_type: req_type,
                terminal_id: terminal_id,
                accept_seq: accept_seq,
                req_seq: req_seq,
                req_time: req_time,
                content: CardRequestContent{
                    groupid: groupid,
                    ddr_city: ddr_city,
                    iccid: iccid,
                    msisdn: msisdn,
                    telnum: telnum,
                    service_number: service_number,
                    cycle: cycle,
                    oprtype: oprtype,
                    reason: reason,
                    service: service,
                    sub_service_status: sub_service_status,
                },
            }
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
