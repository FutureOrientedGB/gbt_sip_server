use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LiveStopRequest {
    pub gb_code: String,
    pub stream_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LiveStopResponse {
    pub code: u32,
    pub msg: String,
    pub gb_code: String,
    pub stream_id: u64,
}

