use murmur3::murmur3_32;
use bit_vec::BitVec;

// Number of hashes
const K: usize = 3;

// Number of bits
const M: usize = 256;

pub struct BloomFilter {
    bv: BitVec
}

impl BloomFilter {
    pub fn new() -> BloomFilter {
        BloomFilter { bv: BitVec::from_elem(M, false) }
    }

    pub fn add(&mut self, element: &str) {
        let hashes = hash(element);
        for hash in hashes.iter() {
            self.bv.set(*hash, true);
        }
    }

    pub fn has(&self, element: &str) -> bool {
        let hashes = hash(element);
        hashes.iter().all(|hash| self.bv.get(*hash).unwrap())
    }
}

/// Hashes the given element string
fn hash(element: &str) -> [usize; K] {
    let hash1 = murmur3_32(&mut element.as_bytes(), 0) as usize;
    let hash2 = murmur3_32(&mut element.as_bytes(), hash1 as u32) as usize;
    let mut hashes = [0usize; K];
    for k in 0..K {
        hashes[k] = (hash1 + k * hash2) % M;
    }
    hashes
}
