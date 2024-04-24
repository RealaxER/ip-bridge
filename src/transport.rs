pub mod mqtt;
use serde_json::Value;

pub enum TransportIn {}

#[derive(Clone)]
pub enum TransportOut {
    ResponseMqttEvent(String, Value),
}
