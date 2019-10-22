#[derive(Debug)]
pub struct MerkleNode {
	pub index: u32,
	pub left: Option<Box<MerkleNode>>,
	pub right: Option<Box<MerkleNode>>,
}

impl MerkleNode {
	pub fn new(index: u32) -> Self {
		Self {
			index,
			left: None,
			right: None,
		}
	}
}
