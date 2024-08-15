use std::fs;

use btleplug::api::BDAddr;
use mqtt_beacon::{
  beacon::TrackedBeacon,
  config::Config,
  error::{BeaconError, BeaconResult},
  listener::Listener,
};
use mqtt_garage::mqtt_client::MqttClient;
use tokio::{sync::mpsc, task::JoinHandle};

#[tokio::main]
async fn main() -> BeaconResult<()> {
  env_logger::init();

  let config = fs::read_to_string("beacon-config.toml").expect("unable to read beacon-config.toml");
  let config: Config = toml::from_str(&config).expect("unable to parse beacon-config.toml");

  let (send_channel2, mut client) = MqttClient::with_config("mqtt-beacon", config.mqtt_client);
  let send_channel: &'static tokio::sync::mpsc::UnboundedSender<mqtt_garage::mqtt_client::MqttPublish> =
    Box::leak(Box::new(send_channel2));

  let beacons: &'static Vec<_> = Box::leak(Box::new(
    config
      .beacons
      .into_iter()
      .map(|beacon| TrackedBeacon::new(beacon, &send_channel))
      .collect(),
  ));

  let (tx, mut rx) = mpsc::unbounded_channel::<BDAddr>();
  let listen = tokio::task::spawn(async move { Listener::listen(tx).await });

  let process_addresses = tokio::task::spawn(async move {
    loop {
      match rx.recv().await {
        Some(address) => {
          log::debug!("Discovered: {:?}", &address);
          for beacon in beacons {
            if beacon.matches_address(&address) {
              beacon.on_discovery().await;
            }
          }
        }
        None => return Err::<(), _>(BeaconError::EmptyChannel), // channel close
      }
    }
  });

  client.announce().await.expect("failed to announce client");

  let mut receiver = client.receiver;
  let receive = tokio::spawn(async move { receiver.receive_messages().await });

  let mut sender = client.sender;
  let send = tokio::spawn(async move { sender.send_messages().await });

  // the two tasks will only end if an error occurs (most likely MQTT broker disconnection)
  tokio::try_join!(
    flatten(receive),
    flatten(send),
    flatten(listen),
    flatten(process_addresses)
  )
  .unwrap_err();
  Ok(())
}

async fn flatten<T, E: Into<BeaconError>>(handle: JoinHandle<Result<T, E>>) -> Result<T, BeaconError> {
  match handle.await {
    Ok(Ok(result)) => Ok(result),
    Ok(Err(err)) => Err(err.into()),
    Err(err) => Err(err.into()),
  }
}
