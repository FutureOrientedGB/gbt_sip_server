pub trait StoreEngine: Send + Sync {
    fn is_connected(&self) -> bool {
        return false;
    }

    fn set_global_sn(&self, _v: u32) {}

    fn add_fetch_global_sn(&self) -> u32 {
        return 0;
    }

    fn set_register_sequence(&self, _seq: u32) {}

    fn add_fetch_register_sequence(&self) -> u32 {
        return 0;
    }

    fn set_global_sequence(&self, _seq: u32) {}

    fn add_fetch_global_sequence(&self) -> u32 {
        return 0;
    }

    fn find_device_by_gb_code(
        &self,
        _key: &String,
    ) -> Option<(
        String,
        std::net::SocketAddr,
        Option<std::sync::Arc<tokio::sync::Mutex<tokio::net::TcpStream>>>,
    )> {
        return None;
    }

    fn find_device_by_stream_id(
        &self,
        _key: u32,
    ) -> Option<(
        String,
        std::net::SocketAddr,
        Option<std::sync::Arc<tokio::sync::Mutex<tokio::net::TcpStream>>>,
    )> {
        return None;
    }

    fn find_gb_code(&self, _stream_id: u32) -> String {
        return String::new();
    }

    fn register(
        &self,
        _branch: &String,
        _gb_code: &String,
        _socket_addr: std::net::SocketAddr,
        _tcp_stream: &Option<std::sync::Arc<tokio::sync::Mutex<tokio::net::TcpStream>>>,
    ) -> bool {
        return false;
    }

    fn unregister(&self, _gb_code: &String) -> bool {
        return false;
    }

    fn register_keep_alive(&self, _gb_code: &String) -> bool {
        return false;
    }

    fn invite(
        &self,
        _gb_code: &String,
        _is_live: bool,
    ) -> Option<(
        bool,
        u32,
        String,
        std::net::SocketAddr,
        Option<std::sync::Arc<tokio::sync::Mutex<tokio::net::TcpStream>>>,
    )> {
        return None;
    }

    fn bye(
        &self,
        _gb_code: &String,
        _stream_id: u32,
    ) -> (
        bool,
        Option<(
            String,
            std::net::SocketAddr,
            Option<std::sync::Arc<tokio::sync::Mutex<tokio::net::TcpStream>>>,
        )>,
    ) {
        return (false, None);
    }

    fn stream_keep_alive(&self, _gb_code: &String, _stream_id: u32) -> bool {
        return false;
    }

    fn start_timeout_check(
        &mut self,
        _timeout_devices_sender: std::sync::mpsc::Sender<Option<String>>,
        _timeout_streams_sender: std::sync::mpsc::Sender<Option<(String, u32)>>,
    ) {
    }

    fn stop_timeout_check(&mut self) {}
}
