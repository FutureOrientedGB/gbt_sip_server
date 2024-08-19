use tokio;
use uuid::Uuid;

use crate::store::base::StoreEngine;
use crate::utils::cli::CommandLines;

pub struct MemoryStore {
    pub quit_flag: bool,
    pub task_handle: Option<tokio::task::JoinHandle<()>>,
    pub service_id: String, // random generated on boot, report to load balence
    pub stream_timeout_seconds: u32,
    pub device_timeout_seconds: u32,
    pub live_stream_id: std::sync::atomic::AtomicU32, // auto increment
    pub replay_stream_id: std::sync::atomic::AtomicU32, // auto increment
    pub global_sn: std::sync::atomic::AtomicU32, // SN
    pub register_sequence: std::sync::atomic::AtomicU32, // CSeq
    pub global_sequence: std::sync::atomic::AtomicU32, // CSeq
    pub sip_devices: std::sync::Arc<
        std::sync::Mutex<
            std::collections::HashMap<
                String,
                (
                    String,
                    std::net::SocketAddr,
                    Option<std::sync::Arc<tokio::sync::Mutex<tokio::net::TcpStream>>>,
                    u32,
                ),
            >,
        >,
    >, // device gb_code -> (branch, net addr, ts)
    pub gb_streams: std::sync::Arc<std::sync::Mutex<std::collections::HashMap<u32, (String, u32)>>>, // stream_id -> (device gb_code, ts)
    pub gb_streams_rev: std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, u32>>>, // device gb_code -> stream_id
}

impl MemoryStore {
    pub fn new(cli_args: &CommandLines) -> Self {
        MemoryStore {
            quit_flag: true,
            task_handle: None,
            service_id: Uuid::new_v4().to_string(),
            stream_timeout_seconds: cli_args.stream_timeout_seconds,
            device_timeout_seconds: cli_args.stream_timeout_seconds,
            live_stream_id: std::sync::atomic::AtomicU32::new(0),
            replay_stream_id: std::sync::atomic::AtomicU32::new(0),
            global_sn: std::sync::atomic::AtomicU32::new(0),
            register_sequence: std::sync::atomic::AtomicU32::new(0),
            global_sequence: std::sync::atomic::AtomicU32::new(0),
            sip_devices: std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::<
                String,
                (
                    String,
                    std::net::SocketAddr,
                    Option<std::sync::Arc<tokio::sync::Mutex<tokio::net::TcpStream>>>,
                    u32,
                ),
            >::default())),
            gb_streams: std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::<
                u32,
                (String, u32),
            >::default())),
            gb_streams_rev: std::sync::Arc::new(std::sync::Mutex::new(
                std::collections::HashMap::<String, u32>::default(),
            )),
        }
    }
}

impl StoreEngine for MemoryStore {
    fn is_connected(&self) -> bool {
        return true;
    }

    fn set_global_sn(&self, v: u32) {
        self.global_sn
            .store(v, std::sync::atomic::Ordering::Relaxed);
    }

    fn add_fetch_global_sn(&self) -> u32 {
        self.global_sn
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            + 1
    }

    fn set_register_sequence(&self, seq: u32) {
        self.register_sequence
            .store(seq, std::sync::atomic::Ordering::Relaxed);
    }

    fn add_fetch_register_sequence(&self) -> u32 {
        self.register_sequence
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            + 1
    }

    fn set_global_sequence(&self, seq: u32) {
        self.global_sequence
            .store(seq, std::sync::atomic::Ordering::Relaxed);
    }

    fn add_fetch_global_sequence(&self) -> u32 {
        self.global_sequence
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            + 1
    }

    fn find_device_by_gb_code(
        &self,
        key: &String,
    ) -> Option<(
        String,
        std::net::SocketAddr,
        Option<std::sync::Arc<tokio::sync::Mutex<tokio::net::TcpStream>>>,
    )> {
        if let Some((branch, addr, tcp_stream, _ts)) = self.sip_devices.lock().unwrap().get(key) {
            return Some((branch.clone(), addr.clone(), tcp_stream.clone()));
        }
        return None;
    }

    fn find_device_by_stream_id(
        &self,
        key: u32,
    ) -> Option<(
        String,
        std::net::SocketAddr,
        Option<std::sync::Arc<tokio::sync::Mutex<tokio::net::TcpStream>>>,
    )> {
        let gb_code = self.find_gb_code(key);
        if !gb_code.is_empty() {
            return self.find_device_by_gb_code(&gb_code);
        }
        return None;
    }

    fn find_gb_code(&self, stream_id: u32) -> String {
        if let Some((gb_code, _ts)) = self.gb_streams.lock().unwrap().get(&stream_id) {
            return gb_code.to_string();
        }
        return String::new();
    }

    fn register(
        &self,
        branch: &String,
        gb_code: &String,
        socket_addr: std::net::SocketAddr,
        tcp_stream: &Option<std::sync::Arc<tokio::sync::Mutex<tokio::net::TcpStream>>>,
    ) -> bool {
        let locked_devices = self.sip_devices.lock().unwrap();
        if locked_devices.get(gb_code).is_none() {
            drop(locked_devices);

            let ts = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs() as u32;

            self.sip_devices.lock().unwrap().insert(
                gb_code.clone(),
                (branch.clone(), socket_addr, tcp_stream.clone(), ts),
            );
            return true;
        }
        return false;
    }

    fn unregister(&self, gb_code: &String) -> bool {
        let locked_device = self.sip_devices.lock().unwrap();
        if locked_device.get(gb_code).is_some() {
            drop(locked_device);

            self.sip_devices.lock().unwrap().remove(gb_code);
            return true;
        }
        return false;
    }

    fn register_keep_alive(&self, gb_code: &String) -> bool {
        let locked_device = self.sip_devices.lock().unwrap();
        if let Some((brh, addr, tcp_stream, _ts)) = locked_device.get(gb_code) {
            let address = addr.clone();
            let branch = brh.clone();
            let stream = tcp_stream.clone();
            drop(locked_device);

            let ts = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs() as u32;

            self.sip_devices
                .lock()
                .unwrap()
                .insert(gb_code.clone(), (branch.clone(), address, stream, ts));
            return true;
        }
        return false;
    }

    fn invite(
        &self,
        gb_code: &String,
        is_live: bool,
    ) -> (
        bool,
        bool,
        u32,
        std::net::SocketAddr,
        Option<std::sync::Arc<tokio::sync::Mutex<tokio::net::TcpStream>>>,
        String,
    ) {
        let result = self.find_device_by_gb_code(gb_code);
        if result.is_none() {
            return (
                false,
                false,
                0,
                std::net::SocketAddr::new(
                    std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
                    8080,
                ),
                None,
                String::new(),
            );
        }
        let (branch, device_addr, tcp_stream) = result.unwrap();

        let stream_id = if is_live {
            self.live_stream_id
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        } else {
            self.replay_stream_id
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        };

        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as u32;

        self.gb_streams
            .lock()
            .unwrap()
            .insert(stream_id, (gb_code.clone(), ts));

        let is_playing = self.gb_streams_rev.lock().unwrap().get(gb_code).is_some();

        self.gb_streams_rev
            .lock()
            .unwrap()
            .insert(gb_code.clone(), stream_id);

        return (true, is_playing, stream_id, device_addr, tcp_stream, branch);
    }

    fn bye(&self, _gb_code: &String, stream_id: u32) -> bool {
        if self.find_gb_code(stream_id).is_empty() {
            return false;
        }

        self.gb_streams.lock().unwrap().remove(&stream_id);
        return true;
    }

    fn stream_keep_alive(&self, gb_code: &String, stream_id: u32) -> bool {
        let locked_streams = self.gb_streams.lock().unwrap();
        if let Some((_gb_code, _ts)) = locked_streams.get(&stream_id) {
            drop(locked_streams);

            let ts = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs() as u32;

            self.gb_streams
                .lock()
                .unwrap()
                .insert(stream_id, (gb_code.clone(), ts));
            return true;
        }
        return false;
    }

    fn start_timeout_check(
        &mut self,
        timeout_devices_sender: std::sync::mpsc::Sender<Option<String>>,
        timeout_streams_sender: std::sync::mpsc::Sender<Option<(String, u32)>>,
    ) {
        self.quit_flag = false;

        let quit_flag = std::sync::Arc::new(self.quit_flag);
        let sip_devices = self.sip_devices.clone();
        let gb_streams = self.gb_streams.clone();

        let stream_timeout_seconds = self.stream_timeout_seconds;
        let device_timeout_seconds = self.device_timeout_seconds;
        self.task_handle = Some(tokio::spawn(async move {
            tracing::info!("start_timeout_check begin");

            while !*quit_flag {
                let ts_now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_secs() as u32;

                let mut timeout_streams = Vec::<(String, u32)>::default();
                for (stream_id, (gb_code, ts)) in gb_streams.lock().unwrap().iter() {
                    if *ts - ts_now > stream_timeout_seconds {
                        timeout_streams.push((gb_code.clone(), *stream_id));
                    }
                }

                for (gb_code, stream_id) in timeout_streams {
                    if let Err(e) = timeout_streams_sender.send(Some((gb_code, stream_id))) {
                        tracing::error!("timeout_streams_sender.send error, e: {:?}", e);
                    }
                }

                let mut timeout_devices = Vec::<String>::default();
                for (gb_code, (_branch, _sock, _tcp_stream, ts)) in
                    sip_devices.lock().unwrap().iter()
                {
                    if *ts - ts_now > device_timeout_seconds {
                        timeout_devices.push(gb_code.clone());
                    }
                }

                for gb_code in timeout_devices {
                    if let Err(e) = timeout_devices_sender.send(Some(gb_code)) {
                        tracing::error!("timeout_devices_sender.send error, e: {:?}", e);
                    }
                }

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

unsafe impl Send for MemoryStore {}

unsafe impl Sync for MemoryStore {}
