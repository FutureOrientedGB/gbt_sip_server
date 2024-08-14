pub trait StoreEngine: Send + Sync {
    fn is_connected(&self) -> bool;

    fn find_device_by_gbcode(&self, key: &String) -> String;
    fn find_device_by_stream_id(&self, key: u64) -> String;
    fn find_gb_code(&self, stream_id: u64) -> String;

    fn register(&mut self, gb_code: &String, socket_addr: std::net::SocketAddr) -> bool;
    fn unregister(&mut self, gb_code: &String) -> bool;
    fn register_keep_alive(&mut self, gb_code: &String) -> bool;

    fn invite(&self, gb_code: &String, is_live: bool) -> (bool, bool, u64);
    fn bye(&self, gb_code: &String, stream_id: u64) -> bool;
    fn stream_keep_alive(&self, gb_code: &String, stream_id: u64) -> bool;

    fn start_timeout_check(
        &mut self,
        timeout_devices_sender: std::sync::mpsc::Sender<Option<String>>,
        timeout_streams_sender: std::sync::mpsc::Sender<Option<(String, u64)>>,
    );
    fn stop_timeout_check(&mut self);
}
