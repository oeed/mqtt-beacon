use std::sync::mpsc::RecvError;

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
  #[error(transparent)]
  Rumble(#[from] rumble::Error),
  #[error(transparent)]
  Recv(#[from] RecvError),
}
