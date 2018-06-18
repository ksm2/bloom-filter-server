use std::sync::{Arc, Mutex};
use murmur3::murmur3_32;
use bit_vec::BitVec;
use rayon::prelude::*;

// Number of hashes
const K: usize = 3;

// Number of bits
const M: usize = 256;

pub struct BloomFilter {
    bv: Arc<Mutex<BitVec>>,
    cv: Arc<Mutex<[u32; M]>>,
}

impl BloomFilter {
    /// Creates a new Bloom filter.
    pub fn new() -> BloomFilter {
        BloomFilter {
            bv: Arc::new(Mutex::new(BitVec::from_elem(M, false))),
            cv: Arc::new(Mutex::new([0; M])),
        }
    }

    /// Adds many new elements to the Bloom filter.
    pub fn add(&mut self, elements: Vec<&[u8]>) -> () {
        elements.par_iter()
            .flat_map(|e: &&[u8]| {
                let mut b = *e;
                let h1 = murmur3_32(&mut b, 0);
                make_hash_vec(h1 as usize, murmur3_32(&mut b, h1) as usize)
            })
            .for_each(|hash: usize| {
                let cv = self.cv.clone();
                let mut cv = cv.lock().unwrap();
                if cv[hash] == 0 {
                    let bv = self.bv.clone();
                    let mut bv = bv.lock().unwrap();
                    bv.set(hash, true);
                }
                cv[hash] += 1;
            });
    }

    /// Removes an element from the Bloom filter.
    ///
    /// Returns `true`, if element is not contained afterwards.
    pub fn remove(&mut self, element: &[u8]) -> bool {
        if !self.has(element) {
            return true
        }

        let mut result = false;
        let hashes = hash(&element);
        for hash in hashes.iter() {
            let mut arc = self.cv.lock().unwrap();
            arc[*hash] -= 1;
            if arc[*hash] == 0 {
                result = true;
                self.bv.lock().unwrap().set(*hash, false);
            }
        }

        result
    }

    /// Checks, whether the given element is contained in the Bloom filter.
    pub fn has(&self, element: &[u8]) -> bool {
        let hashes = hash(&element);
        let bv = self.bv.lock().unwrap();
        hashes.par_iter().all(|hash: &usize| bv.get(*hash).unwrap())
    }

    /// Counts the occurrence of an element within a Bloom filter.
    pub fn count(&self, element: &[u8]) -> u32 {
        let hashes = hash(&element);
        let cv = self.cv.lock().unwrap();
        hashes.par_iter().map(|hash: &usize| cv[*hash]).min().unwrap()
    }

    /// Returns a byte vector of the bits.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bv.lock().unwrap().to_bytes()
    }
}

/// Hashes the given element string
pub fn hash(element: &&[u8]) -> [usize; K] {
    let mut b = *element;
    let hash1 = murmur3_32(&mut b, 0) as usize;
    let hash2 = murmur3_32(&mut b, hash1 as u32) as usize;
    let mut hashes = [0usize; K];
    for k in 0..K {
        hashes[k] = (hash1 + k * hash2) % M;
    }
    hashes
}

/// Hashes the given element string
#[cfg(test)]
pub fn hash_vec(element: &[u8]) -> Vec<usize> {
    hash(&element).to_vec()
}

pub fn make_hash_vec(hash1: usize, hash2: usize) -> Vec<usize> {
    let mut hashes = [0usize; K];
    for k in 0..K {
        hashes[k] = (hash1 + k * hash2) % M;
    }
    hashes.to_vec()
}

/// Hashes the given elements strings
#[cfg(test)]
pub fn hash_many(elements: Vec<&[u8]>) -> Vec<usize> {
    elements.par_iter()
        .flat_map(|e: &&[u8]| {
            let mut b = *e;
            let h1 = murmur3_32(&mut b, 0);
            make_hash_vec(h1 as usize, murmur3_32(&mut b, h1) as usize)
        })
        .collect::<Vec<usize>>()
}
