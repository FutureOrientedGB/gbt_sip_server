use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplayKeepAliveRequest {
    pub gb_code: String,
    pub stream_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplayKeepAliveResponse {
    pub code: u32,
    pub msg: String,
    pub gb_code: String,
    pub stream_id: u64,
}

