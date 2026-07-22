mod blocks;
mod utils;

use std::sync::mpsc::{channel, Sender};
use std::thread;

use crate::blocks::BlockUpdate;

fn main() {
    let (sender, receiver) = channel::<BlockUpdate>();

    let mut blocks_run: Vec<fn(usize, Sender<BlockUpdate>)> = Vec::new();
    blocks_run.push(blocks::battery::run);
    blocks_run.push(blocks::clock::run);

    for (i, block) in blocks.into_iter().enumerate() {
        let sender = sender.clone();
        thread::spawn(move || block(i, sender));
    }

    while let Ok(block_update) = receiver.recv() {
        println!("{}", block_update.full_text());
    }

    println!("shutting down");
}
