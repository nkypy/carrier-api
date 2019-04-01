use hashbrown::HashMap;
use lazy_static::lazy_static;

use crate::{CardInfo, CardStatus, Result};

lazy_static! {
    static ref ERROR_HASHMAP: HashMap<&'static str, (&'static str, &'static str)> = {
        let m: HashMap<&'static str, (&'static str, &'static str)> = [].iter().cloned().collect();
        m
    };
    static ref STATUS_NAME_HASHMAP: HashMap<&'static str, &'static str> = {
        let m: HashMap<&'static str, &'static str> = [
            ("1", "正常"),
            ("2", "待激活"),
            ("3", "停机"),
            ("4", "销户"),
            ("8", "全停"),
            ("9", "全停"),
        ]
        .iter()
        .cloned()
        .collect();
        m
    };
}
// 江苏移动请求格式
pub struct CardRequest {}

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
        format!(
            r#"<operation_in>
<process_code>{}</process_code>
<app_id>{}</app_id>
<access_token>{}</access_token>
<sign>{}</sign>
<verify_code>{}</verify_code>
<req_type>{}</req_type>
<terminal_id>{}</terminal_id>
<accept_seq>{}</accept_seq>
<req_seq>{}</req_seq>
<req_time>{}</req_time>
<content>
<groupid>{}</groupid>
<ddr_city>{}</ddr_city>
<iccid>{}</iccid>
<msisdn>{}</msisdn>
<telnum>{}</telnum>
<service_number>{}</service_number>
<cycle>{}</cycle>
<oprtype>{}</oprtype>
<reason>{}</reason>
<service>{}</service>
<SUB_SERVICE_STATUS>{}</SUB_SERVICE_STATUS>
</content>
</operation_in>"#,
            process_code,
            app_id,
            access_token,
            sign,
            verify_code,
            req_type,
            terminal_id,
            accept_seq,
            req_seq,
            req_time,
            groupid,
            ddr_city,
            iccid,
            msisdn,
            telnum,
            service_number,
            cycle,
            oprtype,
            reason,
            service,
            sub_service_status
        )
        .replace('\n', "")
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
