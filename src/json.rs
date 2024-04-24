extern crate serde_json;
use crate::devices::Device;
use rand::Rng;
use serde_json::{json, Value};

pub enum JsonIn {
    ConvertAckNewDevice { device: Device },
    ConvertSource { json_data: Value },
    ConvertDelete { json_data: Value },

    KeepAlive,
}

#[derive(Clone)]
pub struct JsonDriver {}

impl JsonDriver {
    pub async fn get_reqid(&mut self) -> String {
        let random_string: String = rand::thread_rng()
            .sample_iter(rand::distributions::Alphanumeric)
            .take(12)
            .map(char::from)
            .collect();
        random_string.to_string()
    }

    pub async fn convert(&mut self, type_convert: JsonIn) -> String {
        match type_convert {

            JsonIn::ConvertAckNewDevice { device } => {
                let mut json_value = json!({
                    "cmd": "ack",
                    "objects": [{
                        "data": [],
                        "type": "devices"
                    }],
                    "reqid": self.get_reqid().await,
                    "source": "ip"
                });
            
                let data_array = json_value["objects"][0]["data"].as_array_mut().unwrap();
            
                for hash in device.hash {
                    data_array.push(Value::String(hash));
                }
            
                return json_value.to_string();
            }
            
            JsonIn::ConvertDelete { mut json_data } => {
                json_data["source"] = json!("ip".to_string());

                if let Some(objects) = json_data["objects"].as_array_mut() {
                    for obj in objects.iter_mut() {
                        if let Some(bridge_key) = obj.get_mut("bridge_key") {
                            *bridge_key = json!("ip".to_string());
                        }

                        if let Some(type_) = obj.get_mut("type") {
                            *type_ = json!("devices_local".to_string());
                        }
                    }
                }

                return json_data.to_string();
            }

            JsonIn::ConvertSource { mut json_data } => {
                json_data["source"] = json!("ip".to_string());

                if let Some(objects) = json_data["objects"].as_array_mut() {
                    for obj in objects.iter_mut() {
                        if let Some(bridge_key) = obj.get_mut("bridge_key") {
                            *bridge_key = json!("ip".to_string());
                        }
                    }
                }

                return json_data.to_string();
            }

            JsonIn::KeepAlive => {
                let mut json_ka = json!({
                    "cmd": "status",
                    "objects": [
                        {
                            "bridge_key": "ip",
                            "data": [],
                            "type": "keepalive"
                        }
                    ],
                    "reqid": "",
                    "source": "ip"
                });

                json_ka["reqid"] = json!(self.get_reqid().await);

                return json_ka.to_string();
            }
        }
    }

    pub fn check_packet(&mut self, parsed_json: Value) -> String {
        // Truy cập vào trường "type" và chuyển đổi thành chuỗi
        if let Some(object) = parsed_json["objects"].get(0) {
            if let Some(ob_type) = object["type"].as_str() {
                if ob_type == "reset_brigde" {
                    return ob_type.to_string();
                }
            }
        }
        if let Some(objs) = parsed_json["cmd"].as_str() {
            if parsed_json["control_source"].is_null() {
                return objs.to_string();
            } else {
                let cmd = format!("{}_ctl", objs.to_string());
                return cmd;
            }
        }

        return "None".to_string();
    }
}
