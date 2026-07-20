mod blocks;
mod utils;

use std::sync::mpsc::{channel, Sender};
use std::thread;

use crate::blocks::BlockUpdate;

fn main() {
    let (sender, receiver) = channel::<blocks::BlockUpdate>();

    let mut blocks: Vec<fn(usize, Sender<BlockUpdate>)> = Vec::new();
    blocks.push(blocks::date::run);

    for (i, block) in blocks.into_iter().enumerate() {
        let sender = sender.clone();
        thread::spawn(move || block(i, sender));
    }

    while let Ok(block_update) = receiver.recv() {
        println!("{}", block_update.full_text());
    }

    println!("shutting down");
}
