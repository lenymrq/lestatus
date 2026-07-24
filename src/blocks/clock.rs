use chrono::Local;
use std::sync::mpsc::Sender;
use std::thread::sleep;
use std::time::Duration;

use crate::blocks::BlockUpdate;

const DEFAULT_INTERVAL: u64 = 1_000_000_000; // nanoseconds

pub fn run(block_id: usize, sender: Sender<BlockUpdate>) {
    loop {
        let now = Local::now();

        if sender
            .send(BlockUpdate::new(
                block_id,
                &format!("{}", now.format("%Y-%m-%d %H:%M:%S")),
            ))
            .is_err()
        {
            eprintln!("clock: could not send block update");
            break;
        };

        let nanos = now.timestamp_subsec_nanos();
        sleep(Duration::from_nanos(DEFAULT_INTERVAL - nanos as u64));
    }
}
