use serde::{Deserialize, Serialize};
use serde_xml_rs::{from_str, to_string};

use tracing;

#[derive(Debug, Serialize, Deserialize)]
pub struct KeepAlive {
    #[serde(rename = "CmdType")]
    cmd_type: String,
    #[serde(rename = "SN")]
    sn: u32,
    #[serde(rename = "DeviceID")]
    device_id: String,
    #[serde(rename = "Status")]
    status: String,
}

impl Default for KeepAlive {
    fn default() -> Self {
        KeepAlive {
            cmd_type: String::new(),
            sn: 0,
            device_id: String::new(),
            status: String::new(),
        }
    }
}

impl KeepAlive {
    pub fn serialize_to_xml(&self) -> String {
        match to_string(self) {
            Ok(s) => {
                return s;
            }
            Err(e) => {
                tracing::error!("serde_xml_rs::to_string({:?}) error, e: {:?}", self, e);
                return String::new();
            }
        }
    }

    pub fn deserialize_from_xml(s: String) -> Self {
        match from_str(&s.as_str()) {
            Ok(k) => {
                return k;
            }
            Err(e) => {
                tracing::error!("serde_xml_rs::from_str({}) error, e: {:?}", s, e);
                return KeepAlive::default();
            }
        }
    }
}
