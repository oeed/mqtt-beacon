use std::sync::mpsc::{channel, Receiver};

use btleplug::api::{BDAddr, Central, CentralEvent, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::Manager;
use futures::stream::StreamExt;

use crate::error::BeaconResult;

#[derive(Debug)]
pub struct Listener;

impl Listener {
  pub async fn listen() -> BeaconResult<Receiver<BDAddr>> {
    log::debug!("Starting BLE listen...");
    let (tx, rx) = channel();

    let manager = Manager::new().await?;

    // get the first bluetooth adapter
    // connect to the adapter
    let adapters = manager.adapters().await.unwrap();
    let central = adapters.into_iter().nth(0).unwrap();

    let mut events = central.events().await?;

    // start scanning for devices
    central.start_scan(ScanFilter::default()).await?;

    // Print based on whatever the event receiver outputs. Note that the event
    // receiver blocks, so in a real program, this should be run in its own
    // thread (not task, as this library does not yet use async channels).
    while let Some(event) = events.next().await {
      log::debug!("BLE event: {:?}", &event);
      match event {
        CentralEvent::DeviceDiscovered(id) | CentralEvent::DeviceUpdated(id) => {
          if let Ok(peripheral) = central.peripheral(&id).await {
            // we can ignore this error, if it fails the means something failed elsewhere and the program will soon end
            tx.send(peripheral.address()).ok();
          }
        }
        _ => (),
      }
    }

    Ok(rx)
  }
}
