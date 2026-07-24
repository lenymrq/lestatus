use std::sync::mpsc::Sender;
use std::thread::sleep;
use std::time::Duration;

use crate::blocks::utils::SysFs;
use crate::blocks::BlockUpdate;
use crate::utils::get_device_dir;

const DEFAULT_INTERVAL: u64 = 5;
const POWER_SUPPLY_DEVICES_DIR: &str = "/sys/class/power_supply";

pub fn run(block_id: usize, sender: Sender<BlockUpdate>) {
    let power_supply_path =
        match get_device_dir(POWER_SUPPLY_DEVICES_DIR, |name| name.starts_with("BAT")) {
            Ok(Some(value)) => value,
            _ => todo!("handle error"),
        };

    let battery = SysFs::new(power_supply_path);

    let mut capacity: u8 = 0;
    let mut status: String = String::from("none");

    loop {
        let capacity_new = match battery.read_u8("capacity") {
            Ok(value) => value,
            Err(_) => todo!("handle error"),
        };
        let status_new = match battery.read_string("status") {
            Ok(value) => value,
            Err(_) => todo!("handle error"),
        };

        if capacity_new != capacity || status_new != status {
            capacity = capacity_new;
            status = status_new;
            match sender.send(BlockUpdate::new(
                block_id,
                &format!("bat: {} {}%", status, capacity),
            )) {
                Ok(()) => (),
                Err(_) => todo!("handle error"),
            };
        }

        sleep(Duration::from_secs(DEFAULT_INTERVAL));
    }
}
