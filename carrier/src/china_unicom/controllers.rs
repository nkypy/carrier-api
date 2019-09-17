use quick_xml::{Reader, events::Event};

use crate::china_unicom::models::RequestEnvelope;
use crate::china_unicom::ChinaUnicomClient;
use crate::Result;

static API_SOAP_URL: &'static str = "https://api.10646.cn/ws/service/terminal";

impl ChinaUnicomClient {
    fn soap_request(&self, method: &str, iccids: Vec<&str>) -> Result<String> {
        let text = dbg!(RequestEnvelope::new(
            &self.username,
            &self.password,
            &self.soap_license,
            method,
            iccids
        ));
        Ok(crate::http_client()?
            .post(API_SOAP_URL)
            .header("Content-Type", "text/xml")
            .header(
                "SOAPAction",
                format!(
                    "http://api.jasperwireless.com/ws/service/terminal/{}",
                    method
                ),
            )
            .body(text)
            .send()?
            .text()?
        )
    }
    pub fn get_terminal_details(&self, iccids: Vec<&str>) -> Result<String> {
        let s = self.soap_request("GetTerminalDetails", iccids)?;
        // for token in xmlparser::Tokenizer::from(s.as_ref()) {
        //     println!("{:?}", token);
        // };
        // Ok("123".to_string())
        println!("resp is {}", &s);
        let mut reader = Reader::from_str(&s);
        reader.trim_text(true);
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => println!("xml name is {}", String::from_utf8_lossy(e.name())),
                Ok(Event::Text(e)) => println!("xml value is {}", e.unescape_and_decode(&reader).unwrap()),
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // There are several other `Event`s we do not consider here
            }

            // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
            buf.clear();
        }
        Ok(s)
    }
}
