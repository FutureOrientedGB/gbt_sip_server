use serde::{Deserialize, Serialize};
use serde_xml_rs::{from_str, to_string};

use tracing;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AlarmStatus {
    #[serde(rename = "Num")]
    num: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeviceStatus {
    #[serde(rename = "CmdType")]
    cmd_type: String,
    #[serde(rename = "SN")]
    sn: u32,
    #[serde(rename = "DeviceID")]
    device_id: String,
    #[serde(rename = "Result")]
    result: String,
    #[serde(rename = "Online")]
    online: String,
    #[serde(rename = "Status")]
    status: String,
    #[serde(rename = "DeviceTime")]
    device_time: String,
    #[serde(rename = "Alarmstatus")]
    alarm_status: AlarmStatus,
    #[serde(rename = "Encode")]
    encode: String,
    #[serde(rename = "Record")]
    record: String,
}

impl DeviceStatus {
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
                return DeviceStatus::default();
            }
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "Query")]
pub struct DeviceStatusQuery {
    #[serde(rename = "CmdType")]
    cmd_type: String,
    #[serde(rename = "SN")]
    sn: u32,
    #[serde(rename = "DeviceID")]
    device_id: String,
}

impl DeviceStatusQuery {
    pub fn new(sn: u32, gb_code: &String) -> Self {
        DeviceStatusQuery {
            cmd_type: String::from("DeviceStatus"),
            sn: sn,
            device_id: gb_code.clone(),
        }
    }

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
                return DeviceStatusQuery::default();
            }
        }
    }
}
