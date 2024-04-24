use super::TransportOut;
use crate::error::BridgeIpErr;
use rumqttc::{AsyncClient, Event, EventLoop, MqttOptions, QoS};
use serde_json::Value;
use tokio::sync::mpsc;
use tokio::time::Duration;

pub struct MqttDriver {
    pub tx: mpsc::Sender<Result<TransportOut, BridgeIpErr>>,
    pub rx: mpsc::Receiver<Result<TransportOut, BridgeIpErr>>,
    pub options: MqttOptions,
    pub client: AsyncClient,
    pub eventloop: EventLoop,
    pub flag: bool,
    pub broker_id: String,
}
// packed install from app
impl MqttDriver {
    pub async fn new(
        id: String,
        host: String,
        port: u16,
        broker_id: String,
        keep_alive: u64,
    ) -> Self {
        let mut mqttoptions = MqttOptions::new(id, host, port);

        mqttoptions.set_credentials("component", "123");
        mqttoptions.set_keep_alive(Duration::from_secs(keep_alive));
        mqttoptions.set_max_packet_size(26000, 26000);

        let (client, eventloop) = AsyncClient::new(mqttoptions.clone(), 10);

        client
            .subscribe("component/ip/+", QoS::AtMostOnce)
            .await
            .unwrap();

        client
            .subscribe("component/ip/+/+", QoS::AtMostOnce)
            .await
            .unwrap();

        let (tx, rx) = mpsc::channel::<Result<TransportOut, BridgeIpErr>>(5);
        MqttDriver {
            tx: tx,
            rx: rx,
            options: mqttoptions.clone(),
            client: client,
            eventloop: eventloop,
            flag: false,
            broker_id: broker_id,
        }
    }
    pub async fn send(
        &mut self,
        topic: String,
        message: Vec<u8>,
        qos: QoS,
        retain: bool,
    ) -> Result<(), BridgeIpErr> {
        log::info!(
            "--> {} : {}",
            topic,
            String::from_utf8_lossy(&message).to_string()
        );

        match self.client.publish(topic, qos, retain, message).await {
            Ok(res) => Ok(res),
            Err(_) => Err(BridgeIpErr::MqttErr),
        }
    }

    pub async fn recv(&mut self) -> Result<TransportOut, BridgeIpErr> {
        loop {
            let event = self.eventloop.poll().await;
            match &event {
                Ok(v) => match v {
                    Event::Incoming(packet) => match packet {
                        rumqttc::Packet::Publish(publish) => {
                            let payload_str: String =
                                String::from_utf8_lossy(&publish.payload).to_string();

                            let result: Result<Value, BridgeIpErr> =
                                serde_json::from_str(&payload_str)
                                    .map_err(|_| BridgeIpErr::MqttConvertJsoErr);

                            match result {
                                Ok(parsed_json) => {
                                    if let Some(source) = parsed_json["source"].as_str() {
                                        if source == "ip" {
                                            self.tx.send(Err(BridgeIpErr::MqttErr)).await.unwrap();
                                        } else {
                                            self.tx
                                                .send(Ok(TransportOut::ResponseMqttEvent(
                                                    publish.topic.clone(),
                                                    parsed_json.clone(),
                                                )))
                                                .await
                                                .unwrap();
                                            log::info!("<-- {}:{}", publish.topic, payload_str);
                                        }
                                    }
                                    return self.rx.recv().await.unwrap();
                                }
                                Err(e) => {
                                    log::error!("Mqtt error: {:?}", e);
                                    log::error!("Packet mqtt response: {}", payload_str);
                                }
                            }
                        }
                        _ => {}
                    },
                    Event::Outgoing(_) => {}
                },
                Err(e) => {
                    log::info!("Error = {e:?}");
                    self.tx.send(Err(BridgeIpErr::MqttErr)).await.unwrap();
                    return self.rx.recv().await.unwrap();
                }
            }
        }
    }
}
