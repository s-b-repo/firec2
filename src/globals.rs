use lazy_static::lazy_static;
use std::sync::Mutex;

/// This stores the current C2 IP address
lazy_static! {
    pub static ref GLOBAL_C2_IP: Mutex<String> = Mutex::new(String::from("127.0.0.1"));
}
