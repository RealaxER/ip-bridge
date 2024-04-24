use crate::error::BridgeIpErr;
use crate::json::JsonDriver;
use crate::transport::TransportOut;
use std::collections::HashSet;
use rand::Rng;
use regex::Regex;
use serde_json::Value;
use std::collections::VecDeque;

macro_rules! POST {
    () => {
        "post"
    };
}

macro_rules! STATUS {
    () => {
        "status"
    };
}

macro_rules! STATUS_CTL {
    () => {
        "status_ctl"
    };
}

macro_rules! CONTROL {
    () => {
        "set_ctl"
    };
}

macro_rules! DELETE {
    () => {
        "delete"
    };
}

macro_rules! SET {
    () => {
        "set"
    };
}


macro_rules! RESET {
    () => {
        "reset_brigde"
    };
}

#[derive(Clone)]

pub struct BrLogic {
    pub outputs: VecDeque<BrLogicOut>,
    pub json: JsonDriver,
    pub rnd: u32,
    pub acc_token: String,
    pub tick: u64,
    pub flag: bool,
}

#[derive(Clone)]
pub enum TickLogicOut {
    KeepAlive,
    ScanData,
    GetAccToken,
    CheckOff,
}

#[derive(Clone)]
pub enum BrLogicIn {
    Transport(Result<TransportOut, BridgeIpErr>),
    Tick(TickLogicOut),
}

#[derive(Debug, Clone)]
pub enum BrLogicOut {
    None,
    GetDeviceFromSql,

    CheckDeviceOff,
    ScanData,
    KeepAlive,

    PostDeviceEvent { json_data: Value },
    GetDataEvent { mac: String, json_data: Value },
    ControlEvent { mac: String, json_data: Value },
    GetStatusEvent { json_data: Value },
    DeleteEvent {json_data: Value },
    AdvanceEvent  { mac: String, json_data: Value },
    ResetEvent,


}

impl BrLogic {
    pub fn init(&mut self) {
        self.outputs.push_back(BrLogicOut::GetDeviceFromSql);
    }

    pub fn new() -> Self {
        let outputs = std::iter::once(BrLogicOut::None).collect();
        let mut rng = rand::thread_rng();

        let random_number: u32 = rng.gen_range(90..=120);

        BrLogic {
            outputs: outputs,
            json: JsonDriver {},
            rnd: random_number,
            acc_token: "".to_string(),
            tick: 0,
            flag: false,
        }
    }

    fn extract_mac_from_packet(&mut self, parsed_json: Value) -> Result<String, BridgeIpErr> {
        if let Some(objects) = parsed_json["objects"].as_array() {
            for obj in objects {
                if let Some(data) = obj["data"].as_array() {
                    for item in data {
                        if let Some(hash) = item["hash"].as_str() {
                            let parts: Vec<&str> = hash.split('-').collect();
                            if let Some(mac) = parts.get(1) {
                                return Ok(mac.to_string());
                            }
                        } else {
                            if let Some(data) = parsed_json["objects"][0]["data"].as_array() {
                                for item in data {
                                    if let Some(mac) = item.as_str() {
                                        let mac_address = mac.split('-').nth(1).unwrap_or("");
                                        return Ok(mac_address.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return Err(BridgeIpErr::MqttErr);
    }
    
    fn extract_mac_from_topic(&mut self, topic: String) -> Option<String> {
        let re = Regex::new(r"/component/ip/([0-9A-Fa-f:]+)").unwrap();
        if let Some(captures) = re.captures(&topic) {
            if let Some(mac) = captures.get(1) {
                return Some(mac.as_str().to_owned());
            }
        }
        None
    }
    
    fn get_mac(&mut self, topic: String, parsed_json: Value) -> String {
        if let Some(mac) = self.extract_mac_from_topic(topic.clone()) {
            return mac;
        } else {
            match self.extract_mac_from_packet(parsed_json.clone()) {
                Ok(mac) => return mac,
                Err(_) => return "None".to_string(),
            }
        }
    }
    

    pub fn extract_mac_for_delete(&mut self, parsed_json: Value) -> Result<Vec<String>, BridgeIpErr> {
        let mut mac_set: HashSet<String> = HashSet::new();
        let mut mac_vec: Vec<String> = Vec::new();
        
        if let Some(data) = parsed_json["objects"][0]["data"].as_array() {
            for item in data {
                if let Some(hash) = item.as_str() {
                    let mac_address = hash.split('-').nth(1).unwrap_or("");
                    if !mac_set.contains(mac_address) {
                        mac_set.insert(mac_address.to_string());
                        mac_vec.push(mac_address.to_string());
                    }
                }
            }
            return Ok(mac_vec);
        }
        
        return Err(BridgeIpErr::GetMacErr);
    }

    pub fn on_event(&mut self, _event: BrLogicIn) {
        match _event {
            BrLogicIn::Transport(result) => match result {
                Ok(transport) => match transport {
                    TransportOut::ResponseMqttEvent(topic , res) => {
                        let packet = self.json.check_packet(res.clone());

                        let mac = self.get_mac(topic, res.clone());
                        log::info!("Packet {}", packet);

                        if mac == "None" && packet != RESET!() {
                            log::error!("Mac none {}", res.clone());
                            return;
                        }

                        if packet == POST!() {
                            self.outputs
                                .push_back(BrLogicOut::PostDeviceEvent { json_data: res });
                        } else if packet == STATUS!() {
                            self.outputs.push_back(BrLogicOut::GetDataEvent {
                                mac,
                                json_data: res,
                            });
                        } else if packet == CONTROL!() {
                            self.outputs.push_back(BrLogicOut::ControlEvent {
                                mac,
                                json_data: res,
                            });
                        } else if packet == STATUS_CTL!() {
                            self.outputs.push_back(BrLogicOut::GetStatusEvent {
                                json_data: res,
                            });
                        } else if packet == DELETE!() {
                            self.outputs.push_back(BrLogicOut::DeleteEvent {
                                json_data: res,
                            });
                        }else if packet == SET!() {
                            self.outputs.push_back(BrLogicOut::AdvanceEvent {
                                mac,
                                json_data: res,
                            });
                        }else if packet == RESET!() {
                            self.outputs.push_back(BrLogicOut::ResetEvent);
                        }
                    }
                },
                Err(e) => match e {
                    _ => {}
                },
            },
            BrLogicIn::Tick(tick_event) => match tick_event {
                TickLogicOut::KeepAlive => {
                    self.outputs.push_back(BrLogicOut::KeepAlive);
                }
                TickLogicOut::ScanData => {
                    self.outputs.push_back(BrLogicOut::ScanData);
                }
                TickLogicOut::GetAccToken => {}
                TickLogicOut::CheckOff => {
                    self.outputs.push_back(BrLogicOut::CheckDeviceOff);
                }
            },
        }
    }
    pub fn pop_action(&mut self) -> Option<BrLogicOut> {
        self.outputs.pop_front()
    }
}

#[cfg(test)]
mod test {

    // use super::*;
}
