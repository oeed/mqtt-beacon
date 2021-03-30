use std::sync::mpsc;

use rumqttc::{Connection, LastWill, MqttOptions, QoS};
use serde::Deserialize;

use crate::error::{BeaconError, BeaconResult};

#[derive(Debug, Deserialize)]
pub struct MqttClientConfig {
  /// The domain name of the broker
  pub broker_domain: String,
  /// The port of the broker, 1883 by default
  pub broker_port: u16,
  /// The name of the MQTT topic availability states are sent on
  pub availability_topic: String,
  /// The payload of the state indicating the door is online
  pub online_availability: String,
  /// The payload of the state indicating the door is offline
  pub offline_availability: String,
}

pub type PublishSender = mpsc::Sender<MqttPublish>;
pub type PublishReceiver = mpsc::Receiver<MqttPublish>;

#[derive(Debug)]
pub struct MqttPublish {
  pub topic: String,
  pub qos: QoS,
  pub retain: bool,
  pub payload: String,
}

pub struct MqttClient {
  client: rumqttc::Client,
  channel: PublishReceiver,
}

impl MqttClient {
  pub fn with_config(config: MqttClientConfig) -> BeaconResult<(PublishSender, Connection, Self)> {
    let mut mqttoptions = MqttOptions::new("mqtt-beacon2", config.broker_domain, config.broker_port);
    mqttoptions.set_last_will(LastWill::new(
      &config.availability_topic,
      config.offline_availability,
      QoS::AtLeastOnce,
      true,
    ));
    mqttoptions.set_keep_alive(5);

    let (mut client, connection) = rumqttc::Client::new(mqttoptions, 10);
    client
      .publish(
        config.availability_topic,
        QoS::AtLeastOnce,
        true,
        config.online_availability,
      )
      .expect("unable to publish availablility");

    let (send_tx, send_rx) = mpsc::channel();

    Ok((
      send_tx,
      connection,
      MqttClient {
        client,
        channel: send_rx,
      },
    ))
  }

  pub fn publish(&mut self, topic: &str, qos: QoS, retain: bool, payload: &str) -> BeaconResult<()> {
    self
      .client
      .publish(topic, qos, retain, payload)
      .map_err(|err| err.into())
  }

  pub fn send_messages(&mut self) -> BeaconResult<()> {
    loop {
      if let Ok(publish) = self.channel.recv() {
        self
          .client
          .publish(publish.topic, publish.qos, publish.retain, publish.payload)
          .map_err(|err| BeaconError::from(err))?;
      }
      else {
        return Ok(());
      }
    }
  }
}
