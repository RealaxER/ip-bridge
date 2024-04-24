use crate::devices::DevicesDriver;
use crate::logic::TickLogicOut;
use crate::logic::{BrLogic, BrLogicIn, BrLogicOut};
use crate::sql::SqlDirver;
use crate::transport::mqtt::MqttDriver;
use crate::{
    error::BridgeIpErr,
    json::JsonIn,
};
use rumqttc::QoS;
use tokio::{
    select,
    time::{interval, Duration, Interval},
};


pub struct SystemIntergration {
    interval: Interval,
    pub logic: BrLogic,
    transport: MqttDriver,
    devices: DevicesDriver,
    sql: SqlDirver,
}

impl SystemIntergration {
    pub async fn new(path_db : String) -> Self {
        let mut device = DevicesDriver::new();

        let broker_id = device.get_local_ip().await.unwrap();
        log::info!("Local ip {}", broker_id);

        SystemIntergration {
            interval: interval(Duration::from_millis(45000)),
            logic: BrLogic::new(),
            transport: MqttDriver::new(
                "ip".to_string(),
                "localhost".to_string(),
                1883,
                broker_id,
                5,
            )
            .await,
            devices: device,
            sql: SqlDirver::new(path_db).await,
        }
    }

    pub async fn init(&mut self) {
        self.logic.init();
    }

    pub async fn recv(&mut self) -> Result<(), BridgeIpErr> {
        select! {
            _ = self.interval.tick() => {

                self.logic.on_event(BrLogicIn::Tick(TickLogicOut::KeepAlive));
                self.logic.on_event(BrLogicIn::Tick(TickLogicOut::CheckOff));

            },

            etransport  = self.transport.recv() => {
                self.logic.on_event(BrLogicIn::Transport(etransport));
            },
        }

        while let Some(out) = self.logic.pop_action() {
            match out {
                BrLogicOut::GetDeviceFromSql => {
                    match self.sql.connect().await {
                        Ok(_) => {
                            match self.sql.get_devices().await {
                                Ok(devices) => {
                                    for device in devices {
                                        self.devices.push_device(device).await;
                                    }
                                }   
                                Err(e) => {
                                    log::error!("Error: {:?}", e);
                                }
                            }
                        }
                        Err(_) => {
                            log::error!("Error connecting to sqlite database");
                        }
                    }
                }

                BrLogicOut::PostDeviceEvent { json_data } => {
                    let result = self.devices.add_device(json_data.clone()).await;
                    
                    match result {
                        Ok(device) =>{
                            match self.sql.add_device(device.clone()).await {
                                Ok(_) => {
                                    // post to hcg1
                                    let topic = format!("component/ip/config");
                                    let message = self.logic.json.convert(JsonIn::ConvertSource { json_data}).await;
                                    let _ = self.transport.send(topic, message.into(), QoS::AtMostOnce, false).await;

                                    // ack 
                                    let topic = format!("component/ip/{}/config", device.mac.clone());
                                    let message  = self.logic.json.convert(JsonIn::ConvertAckNewDevice {device}).await;
                                    let _ = self.transport.send(topic, message.into(), QoS::AtMostOnce, false).await;
                                },
                                Err(e) => {
                                    log::error!("Add device to sql err {:?}", e); 
                                }
                            }
                        }

                        Err(e) => {
                            log::error!("Add device err {:?}", e);
                        }
                    }
                }

                BrLogicOut::GetDataEvent {mac, json_data } => {
                    log::info!("mac: {:?}", mac);
                    let result = self.devices.get_device(mac.clone()).await;

                    match result {
                        Ok(_) => {
                            let topic = format!("component/ip/status");
                            let message = self.logic.json.convert(JsonIn::ConvertSource { json_data}).await;

        
                            let _ = self.transport.send(topic, message.into(), QoS::AtMostOnce, false).await;
                        }
                        Err(_) => {
                            log::error!("No device in database bridge ip");
                        }
                    }
                }
                
                BrLogicOut::GetStatusEvent { json_data } => {
                    let topic = format!("component/ip/status");
                    let message = self.logic.json.convert(JsonIn::ConvertSource { json_data}).await;

                    let _ = self.transport.send(topic, message.into(), QoS::AtMostOnce, false).await;
                }

                BrLogicOut::ControlEvent { mac, json_data } =>{
                    let topic = format!("component/ip/{}/config", mac);
                    let message = self.logic.json.convert(JsonIn::ConvertSource { json_data}).await;
                    

                    let _ = self.transport.send(topic, message.into(), QoS::AtMostOnce, false).await;
                }

                BrLogicOut::AdvanceEvent {mac, json_data } => {
                    let result = self.devices.get_device(mac.clone()).await;

                    match result {
                        Ok(_) => {
                            let topic = format!("component/ip/{}/config", mac);
                            let message = self.logic.json.convert(JsonIn::ConvertSource { json_data}).await;

        
                            let _ = self.transport.send(topic, message.into(), QoS::AtMostOnce, false).await;
                        }
                        Err(_) => {
                            log::error!("No device in database bridge ip");
                        }
                    }
                }

                BrLogicOut::KeepAlive => {
                    log::info!("Keep alive event");
                    let topic = "component/keepalive/ip".to_string();

                    let mess = self.logic.json.convert(JsonIn::KeepAlive).await;
                    self.transport
                        .send(topic, mess.into(), rumqttc::QoS::AtMostOnce, false)
                        .await
                        .unwrap();
                }

                BrLogicOut::DeleteEvent { json_data} => {
                    let result = self.logic.extract_mac_for_delete(json_data.clone());
                    log::info!("mac vec : {:?}", result);
                    match result {
                        Ok(mac_vec) => {
                            let message = self.logic.json.convert(JsonIn::ConvertDelete { json_data: json_data.clone() }).await;
                            for mac in mac_vec {    
                                match self.sql.delete_device(mac.clone()).await{
                                    Ok(_) => {
                                        let topic = format!("component/ip/{}/config", mac);
                                        self.transport
                                        .send(topic, message.clone().into(), rumqttc::QoS::AtMostOnce, false)
                                        .await
                                        .unwrap();
                                    
                                        log::info!("Deleted device {}", mac);
                                    }
                                    Err(e) => {
                                        log::error!("Delete device err {:?}", e);
                                    }   
                                }
                            }

                            let topic = "component/ip/config".to_string();
                            self.transport
                            .send(topic, message.into(), rumqttc::QoS::AtMostOnce, false)
                            .await
                            .unwrap();
                        }
                        Err(_) => {
                            log::error!("No device in database bridge ip");
                        }
                    }
                }
                BrLogicOut::ResetEvent => {
                    self.devices.clear().await;
                    let _ = self.sql.clear().await;
                    log::info!("Deleted all device");
                }
                       
                _ => {}
            }
        }
        Ok(())
    }
}
