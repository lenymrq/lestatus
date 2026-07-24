mod blocks;
mod protocol;
mod utils;

use std::sync::mpsc::{channel, Sender};
use std::thread;

use crate::blocks::BlockUpdate;
use crate::protocol::format_blocks_text;

fn main() {
    let (sender, receiver) = channel::<BlockUpdate>();

    let mut blocks_run: Vec<fn(usize, Sender<BlockUpdate>)> = Vec::new();

    blocks_run.push(blocks::net::run);
    blocks_run.push(blocks::power_supply::run);
    blocks_run.push(blocks::clock::run);

    let mut blocks_text: Vec<String> = Vec::new();

    for (i, block) in blocks_run.into_iter().enumerate() {
        let sender = sender.clone();
        thread::spawn(move || block(i, sender));
        blocks_text.push(String::from(""));
    }

    // Start displaying
    println!("{{\"version\":1}}");
    println!("[");

    // Display blocks
    while let Ok(block_update) = receiver.recv() {
        blocks_text[block_update.block_id()] = String::from(block_update.full_text());
        println!("{}", format_blocks_text(&blocks_text));
    }

    // Stop displaying
    println!("]");

    eprintln!("shutting down");
}
