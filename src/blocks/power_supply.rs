use std::sync::mpsc::Sender;
use std::thread::sleep;
use std::time::Duration;

use crate::blocks::sysfs::SysFs;
use crate::blocks::BlockUpdate;
use crate::utils::get_device_dir;

const DEFAULT_INTERVAL: u64 = 5;
const POWER_SUPPLY_DEVICES_DIR: &str = "/sys/class/power_supply";

pub fn run(block_id: usize, sender: Sender<BlockUpdate>) {
    let Ok(Some(power_supply_path)) =
        get_device_dir(POWER_SUPPLY_DEVICES_DIR, |name| name.starts_with("BAT"))
    else {
        eprintln!("power supply: could not get device dir");
        return;
    };

    let battery = SysFs::new(power_supply_path);

    let mut capacity: String = String::from("none");
    let mut status: String = String::from("none");

    loop {
        let capacity_new = battery
            .read_string("capacity")
            .ok()
            .map_or(String::from("none"), |s| s.trim().to_owned());
        let status_new = battery
            .read_string("status")
            .ok()
            .map_or(String::from("none"), |s| s.trim().to_owned());

        if capacity_new != capacity || status_new != status {
            capacity = capacity_new;
            status = status_new;
            if sender
                .send(BlockUpdate::new(
                    block_id,
                    &format!("bat: {} {}%", status, capacity),
                ))
                .is_err()
            {
                eprintln!("power supply: could not send block update");
                break;
            }
        }

        sleep(Duration::from_secs(DEFAULT_INTERVAL));
    }
}
