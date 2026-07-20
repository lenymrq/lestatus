use chrono::Local;
use std::thread::sleep;
use std::time::Duration;

pub fn run() {
    loop {
        let now = Local::now();
        println!("{}", now.format("%Y-%m-%d %H:%M:%S"));

        let nanos = now.timestamp_subsec_nanos();
        sleep(Duration::from_nanos((1_000_000_000 - nanos) as u64));
    }
}
