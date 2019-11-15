#![allow(dead_code)]
extern crate blake2;
extern crate colored;
extern crate rand;

// --- std ---
use std::fmt;
// --- external ---
use blake2::{digest::generic_array::GenericArray, Blake2b, Digest};
use colored::*;
use rand::{distributions::Standard, thread_rng, Rng};

fn main() {
	let mut mmr = MMR::new();
	for _ in 0..11 {
		mmr.add(Block::forge());
	}
	println!("{}", mmr);
}

struct MMR {
	next_index: u32,
	hashes: Vec<Vec<u8>>,
}

impl MMR {
	pub fn new() -> Self {
		Self {
			next_index: 0,
			hashes: vec![],
		}
	}

	pub fn add(&mut self, block: Block) -> u32 {
		let index = self.next_index;
		let mut height = 0;

		self.hashes.push(block.hash().to_vec());
		while node_height(self.next_index + 1) > height {
			self.next_index += 1;

			let left = self.next_index - (2 << height);
			let right = left + sibling_offset(height);

			self.hashes
				.push(chain_hash(&self.hashes[left as usize], &self.hashes[right as usize]));
			height += 1;
		}
		self.next_index += 1;

		index
	}

	pub fn root_hash(&self) -> Vec<u8> {
		self.bagging_peaks(None, peak_indexes(self.next_index)).unwrap()
	}

	pub fn to_proof_at_index(&self, mut index: u32) -> Self {
		let mut hashes = vec![];

		{
			let last_index = self.next_index - 1;
			let mut height = 0;
			while index <= last_index {
				let current_node_height = node_height(index);
				let next_node_height = node_height(index + 1);

				if next_node_height > current_node_height {
					let left_child_index = index - sibling_offset(height);

					if left_child_index > last_index {
						break;
					}

					hashes.push(self.hashes[left_child_index as usize].clone());
					index += 1;
				} else {
					let right_child_index = index + sibling_offset(height);

					if right_child_index > last_index {
						break;
					}

					hashes.push(self.hashes[right_child_index as usize].clone());
					index += 2 << height;
				}

				height += 1;
			}
		}

		{
			let peak_index = index;
			let peak_indexes = peak_indexes(self.next_index);
			if let Some(hash) = self.bagging_peaks(Some(peak_index), &peak_indexes) {
				hashes.push(hash);
			}
			for peak in self.left_peaks(peak_index, &peak_indexes).into_iter().rev() {
				hashes.push(peak);
			}
		}

		Self {
			next_index: self.next_index,
			hashes,
		}
	}

	pub fn proof(&self, mut index: u32) -> bool {
		let MMR {
			next_index: size,
			hashes,
		} = self.to_proof_at_index(index);
		let peak_indexes = peak_indexes(size);
		let mut proof_hash = self.hashes[index as usize].clone();
		let mut height = 0;

		for hash in &hashes {
			let mut hasher = Blake2b::new();
			if peak_indexes.contains(&index) {
				if peak_indexes[peak_indexes.len() - 1] == index {
					hasher = hasher.chain(&proof_hash).chain(hash);
				} else {
					hasher = hasher.chain(hash).chain(&proof_hash);
					index = peak_indexes[peak_indexes.len() - 1];
				}
				proof_hash = hasher.result().to_vec();
			} else {
				let current_node_height = node_height(index);
				let next_node_height = node_height(index + 1);
				if next_node_height > current_node_height {
					hasher = hasher.chain(hash).chain(&proof_hash);
					index += 1;
				} else {
					hasher = hasher.chain(&proof_hash).chain(hash);
					index += 2 << height;
				}
				proof_hash = hasher.result().to_vec();
				height += 1;
			}
		}

		self.root_hash() == proof_hash
	}

	fn bagging_peaks<A: AsRef<[u32]>>(&self, peak_index: Option<u32>, peak_indexes: A) -> Option<Vec<u8>> {
		let peak_indexes = peak_indexes.as_ref();
		let mut peak_hashes: Vec<_> = if let Some(peak_index) = peak_index {
			peak_indexes
				.iter()
				.filter(|peak_index_| **peak_index_ > peak_index)
				.map(|peak_index| self.hashes[*peak_index as usize].to_owned())
				.collect()
		} else {
			peak_indexes
				.iter()
				.map(|i| self.hashes[*i as usize].to_owned())
				.collect()
		};

		while peak_hashes.len() > 1 {
			let right_peak = peak_hashes.pop().unwrap();
			let left_peak = peak_hashes.pop().unwrap();
			peak_hashes.push(chain_hash(right_peak, left_peak));
		}

		peak_hashes.pop()
	}

	fn left_peaks<A: AsRef<[u32]>>(&self, peak_index: u32, peak_indexes: A) -> Vec<Vec<u8>> {
		peak_indexes
			.as_ref()
			.iter()
			.filter(|peak_index_| **peak_index_ < peak_index)
			.map(|peak_index| self.hashes[*peak_index as usize].clone())
			.collect()
	}

	fn debug_proof_indexes<A: AsRef<[Vec<u8>]>>(all_hashes: A, proof_hashes: A) -> Vec<u32> {
		let all_hashes = all_hashes.as_ref();
		let proof_hashes = proof_hashes.as_ref();
		let mut indexes = vec![];
		for proof_hash in proof_hashes {
			if let Some(index) = all_hashes.iter().position(|hash| hash == proof_hash) {
				indexes.push(index as u32);
			}
		}

		indexes
	}
}

impl fmt::Display for MMR {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}: {}\n", "next_index".cyan(), self.next_index.to_string().yellow())?;
		for (i, hash) in self.hashes.iter().enumerate() {
			write!(
				f,
				"{}{}{}{} = {}\n",
				"hashes".cyan(),
				"[".green(),
				i.to_string().yellow(),
				"]".green(),
				"[".green(),
			)?;
			for chunk in hash.chunks(16) {
				write!(f, "\t")?;
				for bit in chunk {
					write!(f, "{:3}, ", bit.to_string().yellow())?;
				}
				write!(f, "\n")?;
			}
			write!(f, "{}\n", "]".green())?;
		}

		Ok(())
	}
}

struct Block(Vec<u8>);

impl Block {
	pub fn forge() -> Self {
		Block(hash(&random_id()).to_vec())
	}

	pub fn hash(&self) -> &[u8] {
		&self.0
	}
}

fn random_id() -> Vec<u8> {
	thread_rng().sample_iter(&Standard).take(512).collect()
}

fn hash(data: &[u8]) -> GenericArray<u8, <Blake2b as Digest>::OutputSize> {
	Blake2b::new().chain(data).result()
}

fn chain_hash<A: AsRef<[u8]>>(a: A, b: A) -> Vec<u8> {
	Blake2b::new().chain(a.as_ref()).chain(b.as_ref()).result().to_vec()
}

fn node_height(mut index: u32) -> u32 {
	fn bit_len(mut num: u32) -> u32 {
		let mut len = 0;
		while num != 0 {
			num >>= 1;
			len += 1;
		}

		len
	}

	index += 1;
	while ((1 << bit_len(index)) - 1) != index {
		index -= (1 << (bit_len(index) - 1)) - 1;
	}

	bit_len(index) - 1
}

fn sibling_offset(height: u32) -> u32 {
	(2 << height) - 1
}

fn peak_indexes(size: u32) -> Vec<u32> {
	fn right_index(size: u32, mut height: u32, mut index: u32) -> (Option<u32>, u32) {
		index += sibling_offset(height);
		while index > (size - 1) {
			if height == 0 {
				return (None, 0);
			}

			height -= 1;
			index -= 2 << height
		}

		return (Some(height), index);
	}

	let mut indexes = vec![];
	let (mut height, mut index) = {
		let (height, index) = left_peak_index(size);
		(Some(height), index)
	};

	indexes.push(index);
	while let Some(height_) = height {
		if height_ == 0 {
			break;
		}

		let (height_, index_) = right_index(size, height_, index);
		height = height_;
		index = index_;
		if height.is_some() {
			indexes.push(index);
		}
	}

	indexes
}

fn left_peak_index(size: u32) -> (u32, u32) {
	fn left_index(height: u32) -> u32 {
		(1 << height + 1) - 2
	}

	let mut height = 0;
	let mut prev_index = 0;
	let mut index = left_index(height);
	while index < size {
		height += 1;
		prev_index = index;
		index = left_index(height);
	}

	(height - 1, prev_index)
}

#[test]
fn t1() {
	let mut mmr = MMR::new();
	for _ in 0..4 {
		mmr.add(Block::forge());
	}

	assert_eq!(mmr.next_index, 7);
	assert_eq!(chain_hash(&mmr.hashes[0], &mmr.hashes[1]), mmr.hashes[2]);
	assert_eq!(chain_hash(&mmr.hashes[3], &mmr.hashes[4]), mmr.hashes[5]);
	assert_eq!(chain_hash(&mmr.hashes[2], &mmr.hashes[5]), mmr.hashes[6]);
}

#[test]
fn t2() {
	assert_eq!(peak_indexes(15), vec![14]);
	assert_eq!(peak_indexes(18), vec![14, 17]);
	assert_eq!(peak_indexes(19), vec![14, 17, 18]);
}

#[test]
fn t3() {
	let mut mmr = MMR::new();
	for _ in 0..11 {
		mmr.add(Block::forge());
	}

	assert_eq!(
		chain_hash(&chain_hash(&mmr.hashes[18], &mmr.hashes[17]), &mmr.hashes[14]),
		mmr.root_hash()
	);
}

#[test]
fn t4() {
	{
		let mut mmr = MMR::new();
		let mut indexes = vec![];
		for _ in 0..3 {
			indexes.push(mmr.add(Block::forge()));
		}

		for (index, proof_indexes) in indexes[..1]
			.iter()
			.zip([[1, 3].as_ref(), [1, 5, 13].as_ref(), [1, 5, 13].as_ref()].iter())
		{
			assert_eq!(
				&MMR::debug_proof_indexes(&mmr.hashes, &mmr.to_proof_at_index(*index).hashes),
				proof_indexes
			);
			assert!(mmr.proof(*index));
		}
	}

	{
		let mut indexes = vec![];
		let mut mmr = MMR::new();
		for _ in 0..15 {
			indexes.push(mmr.add(Block::forge()));
		}

		assert!(mmr.proof(23));

		for (index, proof_indexes) in indexes.iter().zip(
			[
				[1, 5, 13].as_ref(),
				[0, 5, 13].as_ref(),
				[4, 2, 13].as_ref(),
				[3, 2, 13].as_ref(),
				[8, 12, 6].as_ref(),
				[7, 12, 6].as_ref(),
				[11, 9, 6].as_ref(),
				[10, 9, 6].as_ref(),
				[16, 20, 14].as_ref(),
				[15, 20, 14].as_ref(),
				[19, 17, 14].as_ref(),
				[18, 17, 14].as_ref(),
				[23, 25, 21, 14].as_ref(),
				[22, 25, 21, 14].as_ref(),
				[24, 21, 14].as_ref(),
			]
			.iter(),
		) {
			assert_eq!(
				&MMR::debug_proof_indexes(&mmr.hashes, &mmr.to_proof_at_index(*index).hashes),
				proof_indexes
			);
			assert!(mmr.proof(*index));
		}
	}
}
