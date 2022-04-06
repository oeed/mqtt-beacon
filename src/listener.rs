use std::sync::mpsc::{channel, Receiver};

use btleplug::api::BDAddr;
#[cfg(target_os = "linux")]
use rumble::{
  api::{Central, CentralEvent},
  bluez::manager::Manager,
};

use crate::error::BeaconResult;

#[derive(Debug)]
pub struct Listener;

impl Listener {
  #[cfg(target_os = "linux")]
  pub fn listen() -> BeaconResult<Receiver<BDAddr>> {
    log::debug!("Starting BLE listen...");
    let (tx, rx) = channel();
    let manager = Manager::new()?;

    // get the first bluetooth adapter
    let adapters = manager.adapters()?;
    let mut adapter = adapters.into_iter().nth(0)?;

    // reset the adapter -- clears out any errant state
    adapter = manager.down(&adapter)?;
    adapter = manager.up(&adapter)?;

    // connect to the adapter
    let central = adapter.connect()?;
    central.active(false);
    central.filter_duplicates(false);
    // start scanning for devices
    central.start_scan()?;

    central.on_event(Box::new(move |event| {
      log::debug!("BLE event: {:?}", &event);
      match event {
        CentralEvent::DeviceDiscovered(address) | CentralEvent::DeviceUpdated(address) => {
          let addr = address.address;
          // rumble stores the address backwards for some reason
          let address = [addr[5], addr[4], addr[3], addr[2], addr[1], addr[0]];
          tx.send(BDAddr::from(address))?;
        }
        _ => (),
      }
    }));

    Ok(rx)
  }

  #[cfg(target_os = "macos")]
  pub fn listen() -> BeaconResult<Receiver<BDAddr>> {
    let (tx, rx) = channel();
    std::thread::spawn(move || {
      loop {
        // just demo/debug
        tx.send("01:23:45:67:89:AB".parse().unwrap()).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(2));
      }
    });

    Ok(rx)
  }
}
