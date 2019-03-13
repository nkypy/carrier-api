// 江苏移动请求格式
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "operation_in")]
struct CardRequest<'a> {
    process_code: &'a str,
    app_id: &'a str,
    access_token: &'a str,
    sign: &'a str,
    verify_code: &'a str,
    req_type: &'a str,
    terminal_id: &'a str,
    accept_seq: &'a str,
    req_seq: &'a str,
    req_time: &'a str,
    content: CardRequestContent<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CardRequestContent<'a> {
    groupid: &'a str,
    ddr_city: &'a str,
    iccid: &'a str,
    msisdn: &'a str,
    telnum: &'a str,
    service_number: &'a str,
    cycle: &'a str,
    oprtype: &'a str,
    reason: &'a str,
    service: &'a str,
    #[serde(rename = "SUB_SERVICE_STATUS")]
    sub_service_status: &'a str,
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
