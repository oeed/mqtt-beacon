use std::sync::mpsc::{RecvError, SendError};

use btleplug::api::BDAddr;
use thiserror::Error;
use tokio::task::JoinError;

pub type BeaconResult<T> = Result<T, BeaconError>;

#[derive(Debug, Error)]
pub enum BeaconError {
  #[error(transparent)]
  MQTTClient(#[from] rumqttc::ClientError),
  #[error(transparent)]
  MQTTConnection(#[from] rumqttc::ConnectionError),
  #[error(transparent)]
  JoinError(#[from] JoinError),
  #[cfg(target_os = "linux")]
  #[error("rumble error: {0}")]
  Rumble(String),
  #[error(transparent)]
  MpscRecv(#[from] RecvError),
  #[error(transparent)]
  MpscSend(#[from] SendError<BDAddr>),
}

#[cfg(target_os = "linux")]
impl From<rumble::Error> for BeaconError {
  fn from(err: rumble::Error) -> Self {
    // rumble uses an outdated error handling crate that doesn't use Error
    // thus we need to convert ourselves
    BeaconError::Rumble(err.to_string())
  }
}
