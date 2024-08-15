use tokio;
use uuid::Uuid;

use crate::utils::cli::CommandLines;
use crate::store::base::StoreEngine;

pub struct RedisStore {
    pub quit_flag: bool,
    pub task_handle: Option<tokio::task::JoinHandle<()>>,
    pub service_id: String, // random generated on boot, report to load balence
    pub sip_socket: std::sync::Arc<tokio::net::UdpSocket>, // self socket communicate with devices
}

impl RedisStore {
    pub fn new(
        sip_socket: std::sync::Arc<tokio::net::UdpSocket>,
        _cli_args: &CommandLines,
    ) -> Self {
        RedisStore {
            quit_flag: true,
            task_handle: None,
            service_id: Uuid::new_v4().to_string(),
            sip_socket: sip_socket,
        }
    }
}

impl StoreEngine for RedisStore {
    fn is_connected(&self) -> bool {
        return false;
    }

    fn set_sn(&self, _v: u32) {
    }

    fn add_fetch_sn(&self) -> u32 {
        return 0;
    }

    fn set_call_sequence(&self, _seq: u32) {
    }

    fn add_fetch_call_sequence(&self) -> u32 {
        return 0;
    }

    fn find_device_by_gb_code(&self, _key: &String) -> Option<(String, std::net::SocketAddr)> {
        return None;
    }

    fn find_device_by_stream_id(&self, _key: u32) -> Option<(String, std::net::SocketAddr)> {
        return None;
    }

    fn find_gb_code(&self, _stream_id: u32) -> String {
        return String::new();
    }

    fn register(&self, _branch: &String, _gb_code: &String, _socket_addr: std::net::SocketAddr) -> bool {
        return false;
    }

    fn unregister(&self, _gb_code: &String) -> bool {
        return false;
    }

    fn register_keep_alive(&self, _gb_code: &String) -> bool {
        return false;
    }

    fn invite(&self, _gb_code: &String, _is_live: bool) -> (bool, bool, u32) {
        return (false, false, 0);
    }

    fn bye(&self, _gb_code: &String, _stream_id: u32) -> bool {
        return false;
    }

    fn stream_keep_alive(&self, _gb_code: &String, _stream_id: u32) -> bool {
        return false;
    }

    fn start_timeout_check(
        &mut self,
        _timeout_devices_sender: std::sync::mpsc::Sender<Option<String>>,
        _timeout_streams_sender: std::sync::mpsc::Sender<Option<(String, u32)>>,
    ) {
        self.quit_flag = false;

        let quit_flag = std::sync::Arc::new(self.quit_flag);

        self.task_handle = Some(tokio::spawn(async move {
            tracing::info!("start_timeout_check begin");

            while !*quit_flag {
                if !*quit_flag {
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            }

            tracing::info!("start_timeout_check end");
        }));
    }

    fn stop_timeout_check(&mut self) {
        self.quit_flag = true;
    }
}

unsafe impl Send for RedisStore {}

unsafe impl Sync for RedisStore {}
