use std::fmt;

use blake2::{Blake2b, Digest};
use colored::*;

use crate::{block::Block, hasher::*, merkle_node::MerkleNode};

pub struct MMR {
	pub next_index: u32,
	pub hashes: Vec<Vec<u8>>,
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

	pub fn to_merkle_proof_at_index(&self, mut index: u32) -> Self {
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
		let MMR { next_index, hashes } = self.to_merkle_proof_at_index(index);
		let peak_indexes = peak_indexes(next_index);
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

	pub fn merkle_path_at_index(size: u32, mut index: u32) -> Vec<MerkleNode> {
		let mut merkle_path = vec![];

		{
			let mut height = 0;
			while index <= size {
				let current_node_height = node_height(index);
				let next_node_height = node_height(index + 1);

				if next_node_height > current_node_height {
					let left_child_index = index - sibling_offset(height);

					if left_child_index > size {
						break;
					}

					merkle_path.push(MerkleNode::new(left_child_index));
					index += 1;
				} else {
					let right_child_index = index + sibling_offset(height);

					if right_child_index > size {
						break;
					}

					merkle_path.push(MerkleNode::new(right_child_index));
					index += 2 << height;
				}

				height += 1;
			}
		}

		{
			let peak_index = index;
			let peak_indexes = peak_indexes(size + 1);
			{
				let mut merkle_nodes: Vec<_> = peak_indexes
					.clone()
					.into_iter()
					.map(|peak_index| MerkleNode::new(peak_index))
					.collect();
				while merkle_nodes.len() > 1 {
					let right_peak = merkle_nodes.pop().unwrap();
					let left_peak = merkle_nodes.pop().unwrap();
					merkle_nodes.push(MerkleNode {
						index: right_peak.index + 1,
						left: Some(Box::new(left_peak)),
						right: Some(Box::new(right_peak)),
					});
				}
				if let Some(merkle_node) = merkle_nodes.pop() {
					merkle_path.push(merkle_node);
				}
			}
			for peak_index in peak_indexes
				.into_iter()
				.filter(|peak_index_| *peak_index_ < peak_index)
				.rev()
			{
				merkle_path.push(MerkleNode::new(peak_index));
			}
		}

		merkle_path
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
