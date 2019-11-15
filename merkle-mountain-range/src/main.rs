#![allow(dead_code)]
extern crate blake2;
extern crate colored;
extern crate rand;

mod block;
mod hasher;
mod merkle_mountain_range;
mod merkle_node;

use block::Block;
use merkle_mountain_range::MMR;

fn main() {
	let mut mmr = MMR::new();
	for _ in 0..11 {
		mmr.add(Block::forge());
	}
	println!("{}", mmr);
	let mmr_size = mmr.next_index - 1;
	println!("{:#?}", MMR::merkle_path_at_index(mmr_size, 15));
}
