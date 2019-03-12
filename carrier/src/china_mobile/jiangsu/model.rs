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