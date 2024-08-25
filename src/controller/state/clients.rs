use std::{collections::HashMap, sync::{Arc, Mutex}};

use serde_json::Value;
use tokio::sync::mpsc::UnboundedSender;

pub type Clients = Arc<Mutex<HashMap<String, Vec<UnboundedSender<Value>>>>>;