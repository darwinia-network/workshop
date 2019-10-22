use blake2::{digest::generic_array::GenericArray, Blake2b, Digest};
use rand::{distributions::Standard, thread_rng, Rng};

pub fn random_id() -> Vec<u8> {
	thread_rng().sample_iter(&Standard).take(512).collect()
}

pub fn hash(data: &[u8]) -> GenericArray<u8, <Blake2b as Digest>::OutputSize> {
	Blake2b::new().chain(data).result()
}

pub fn chain_hash<A: AsRef<[u8]>>(a: A, b: A) -> Vec<u8> {
	Blake2b::new().chain(a.as_ref()).chain(b.as_ref()).result().to_vec()
}
