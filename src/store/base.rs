pub trait StoreEngine: Send + Sync {
    fn is_connected(&self) -> bool;

    fn set_global_sn(&self, v: u32);
    fn add_fetch_global_sn(&self) -> u32;

    fn set_register_sequence(&self, seq: u32);
    fn add_fetch_register_sequence(&self) -> u32;

    fn set_global_sequence(&self, seq: u32);
    fn add_fetch_global_sequence(&self) -> u32;

    fn find_device_by_gb_code(&self, key: &String) -> Option<(String, std::net::SocketAddr)>;
    fn find_device_by_stream_id(&self, key: u32) -> Option<(String, std::net::SocketAddr)>;
    fn find_gb_code(&self, stream_id: u32) -> String;

    fn register(&self, branch: &String, gb_code: &String, socket_addr: std::net::SocketAddr) -> bool;
    fn unregister(&self, gb_code: &String) -> bool;
    fn register_keep_alive(&self, gb_code: &String) -> bool;

    fn invite(&self, gb_code: &String, is_live: bool) -> (bool, bool, u32);
    fn bye(&self, gb_code: &String, stream_id: u32) -> bool;
    fn stream_keep_alive(&self, gb_code: &String, stream_id: u32) -> bool;

    fn start_timeout_check(
        &mut self,
        timeout_devices_sender: std::sync::mpsc::Sender<Option<String>>,
        timeout_streams_sender: std::sync::mpsc::Sender<Option<(String, u32)>>,
    );
    fn stop_timeout_check(&mut self);
}
