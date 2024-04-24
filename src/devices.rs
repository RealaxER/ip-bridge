use serde_json::Value;

use crate::error::BridgeIpErr;

#[derive(Clone, Default, Debug)]
pub struct Device {
    pub mac: String,
    pub hash :Vec<String>,
}
impl Device {
    pub fn new() -> Self {
        Device {
            mac: String::new(),
            hash: Vec::new(),
        }
    }
}

pub struct DevicesDriver {
    device: Vec<Device>,
}

impl DevicesDriver {
    pub fn new() -> Self {
        DevicesDriver { device: Vec::new() }
    }

    pub async fn get_local_ip(&mut self) -> Result<String, BridgeIpErr> {
        let socket = std::net::UdpSocket::bind("0.0.0.0:0").unwrap();
        socket.connect("8.8.8.8:80").unwrap();
        let local_ip = socket.local_addr().unwrap().ip();
        return Ok(local_ip.to_string());
    }

    pub async fn clear(&mut self) {
        self.device.clear();
    }

    pub async fn upgrade_device(&mut self, device_new: Device, mac: String) {
        for device in &mut self.device {
            if device.mac == mac {
                *device = device_new;
                break;
            }
        }
    }

    pub async fn get_device(&mut self, mac: String) -> Result<Device, BridgeIpErr> {
        for device in &self.device {
            if device.mac == mac {
                return Ok(device.clone());
            }
        }
        return Err(BridgeIpErr::DeviceNotFound);
    }

    pub async fn delete_device(&mut self, mac: String) -> Result<(), BridgeIpErr> {
        for (index, device) in self.device.iter_mut().enumerate() {
            if device.mac == mac {
                self.device.remove(index);
                return Ok(());
            }
        }
        Err(BridgeIpErr::DeviceNotFound)
    }

    pub async fn push_device(&mut self, device:Device) {
        self.device.push(device);
    } 

    pub async fn add_device(&mut self, parsed_json: Value) -> Result<Device, BridgeIpErr> {
        if let Some(objects) = parsed_json["objects"].as_array() {
            let mut device = Device::new();
            for object in objects {
                if let Some(data) = object["data"].as_array() {
                    for device_data in data {
                        if let Some(mac) = device_data["macdev"].as_str() {
                            device.mac = mac.to_string();
                        }

                        if let Some(hash) = device_data["hash"].as_str() {
                            device.hash.push(hash.to_string().clone());   
                        }
                    }
                }
            }
            self.device.push(device.clone());

            return Ok(device);
        }
        return Err(BridgeIpErr::AddDeviceErr);
    }
}
