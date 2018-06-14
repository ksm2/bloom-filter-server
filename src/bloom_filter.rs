use murmur3::murmur3_32;
use bit_vec::BitVec;

// Number of hashes
const K: usize = 3;

// Number of bits
const M: usize = 256;

pub struct BloomFilter {
    bv: BitVec,
    cv: [u32; M],
}

impl BloomFilter {
    /// Creates a new Bloom filter.
    pub fn new() -> BloomFilter {
        BloomFilter {
            bv: BitVec::from_elem(M, false),
            cv: [0; M],
        }
    }

    /// Adds a new element to the Bloom filter.
    pub fn add(&mut self, element: &str) {
        let hashes = hash(element);
        for hash in hashes.iter() {
            self.bv.set(*hash, true);
            self.cv[*hash] += 1
        }
    }

    /// Removes an element from the Bloom filter.
    ///
    /// Returns `true`, if element is not contained afterwards.
    pub fn remove(&mut self, element: &str) -> bool {
        if !self.has(element) {
            return true
        }

        let mut result = false;
        let hashes = hash(element);
        for hash in hashes.iter() {
            self.cv[*hash] -= 1;
            if self.cv[*hash] == 0 {
                result = true;
                self.bv.set(*hash, false);
            }
        }

        result
    }

    /// Checks, whether the given element is contained in the Bloom filter.
    pub fn has(&self, element: &str) -> bool {
        let hashes = hash(element);
        hashes.iter().all(|hash| self.bv.get(*hash).unwrap())
    }

    /// Counts the occurrence of an element within a Bloom filter.
    pub fn count(&self, element: &str) -> u32 {
        let hashes = hash(element);
        hashes.iter().map(|hash| self.cv[*hash]).min().unwrap()
    }

    /// Returns a byte vector of the bits.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bv.to_bytes()
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
