use std::{fmt::Display, time::Duration};

use btleplug::api::BDAddr;
use mqtt_garage::mqtt_client::{sender::PublishSender, MqttPublish};
use rumqttc::QoS;
use serde::Deserialize;
use tokio::{sync::RwLock, task::JoinHandle, time::sleep};

#[derive(Debug, Deserialize)]
pub struct BeaconConfig {
  /// The bluetooth address of the beacon
  beacon_address: BDAddr,
  /// Topic to broadcast to when the beacon is present
  topic: String,
  /// Expiry time in seconds for the beacon to switch to not home after the last seen time
  expiry: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BeaconState {
  Home,
  NotHome,
}

impl Display for BeaconState {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Home => write!(f, "home"),
      Self::NotHome => write!(f, "not_home"),
    }
  }
}

#[derive(Debug)]
pub struct TrackedBeacon {
  config: BeaconConfig,
  presence: RwLock<Option<BeaconState>>,
  send_channel: &'static PublishSender,
  not_home_timeout: RwLock<Option<JoinHandle<()>>>,
}

impl TrackedBeacon {
  pub fn new(config: BeaconConfig, send_channel: &'static PublishSender) -> Self {
    TrackedBeacon {
      config,
      presence: RwLock::new(None),
      send_channel,
      not_home_timeout: RwLock::new(None),
    }
  }

  pub fn matches_address(&self, address: &BDAddr) -> bool {
    &self.config.beacon_address == address
  }

  pub async fn on_discovery(&'static self) {
    log::debug!("Discovered known beacon: {}", &self.config.topic);

    self.publish_state(BeaconState::Home).await;


    if let Some(handle) = &*self.not_home_timeout.read().await {
      handle.abort();
    }

    *self.not_home_timeout.write().await = Some(tokio::spawn(async move {
      sleep(Duration::from_secs(self.config.expiry)).await;
      log::debug!("Not home timeout expired: {}", &self.config.topic);
      self.publish_state(BeaconState::NotHome).await;
      self.not_home_timeout.write().await.take();
    }));
  }

  async fn publish_state(&self, state: BeaconState) {
    if *self.presence.read().await != Some(state) {
      *self.presence.write().await = Some(state);

      // state has changed, broadcast it
      self
        .send_channel
        .send(MqttPublish {
          topic: self.config.topic.clone(),
          qos: QoS::AtLeastOnce,
          retain: true,
          payload: state.to_string(),
        })
        .ok(); // error only if channel closed (program closing)
    }
  }
}
