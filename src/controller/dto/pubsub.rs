use serde_json::Value;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PublishRequestBodyDTO {
    pub message: Value
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublishResponseDTO {
    pub count: usize
}