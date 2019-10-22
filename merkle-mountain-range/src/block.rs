use crate::hasher::*;

pub struct Block(Vec<u8>);

impl Block {
	pub fn forge() -> Self {
		Block(hash(&random_id()).to_vec())
	}

	pub fn hash(&self) -> &[u8] {
		&self.0
	}
}
