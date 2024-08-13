use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplayStartRequest {
    pub gb_code: String,
    pub ts_begin: u64,
    pub ts_end: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplayStartResponse {
    pub locate: String,
    pub code: u32,
    pub msg: String,
    pub gb_code: String,
    pub stream_id: u64,
}

