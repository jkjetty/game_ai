mod base_game_entity;
mod miner;
mod state;
mod state_machine;
use std::{thread, time};

use miner::Miner;

fn main() {
    println!("starting");
    let mut miner = Miner::new(0);
    let millis = time::Duration::from_millis(100);
    for _ in 0..20 {
        miner.update();
        thread::sleep(millis);
    }
}
