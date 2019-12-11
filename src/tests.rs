#[cfg(test)]
mod tests {
    use test::Bencher;
    use bloom_filter::*;

    #[bench]
    fn bench_add_multiple(bencher: &mut Bencher) -> () {
        let mut bf = BloomFilter::new();
        bencher.iter(move || {
            let adders: Vec<&[u8]> = vec!(b"felix", b"markus", b"isabel", b"jonathan", b"denis");
            bf.add(adders);
        });
    }

    #[bench]
    fn bench_add_one(bencher: &mut Bencher) -> () {
        let mut bf = BloomFilter::new();
        bencher.iter(move || {
            bf.add_one(b"felix");
            bf.add_one(b"markus");
            bf.add_one(b"isabel");
            bf.add_one(b"jonathan");
            bf.add_one(b"denis");
        });
    }

    #[bench]
    fn bench_hash(bencher: &mut Bencher) -> () {
        bencher.iter(|| {
            hash(&b"felix".as_ref());
            hash(&b"markus".as_ref());
            hash(&b"isabel".as_ref());
            hash(&b"jonathan".as_ref());
            hash(&b"denis".as_ref());
        })
    }

    #[bench]
    fn bench_hash_vec(bencher: &mut Bencher) -> () {
        bencher.iter(|| {
            hash_vec(&b"felix".as_ref());
            hash_vec(&b"markus".as_ref());
            hash_vec(&b"isabel".as_ref());
            hash_vec(&b"jonathan".as_ref());
            hash_vec(&b"denis".as_ref());
        })
    }

    #[bench]
    fn bench_murmur32(bencher: &mut Bencher) {
        bencher.iter(|| {
            hash_vec(&b"markus".as_ref());
        })
    }

    #[bench]
    fn bench_hash_many(bencher: &mut Bencher) -> () {
        bencher.iter(|| {
            hash_many(vec!(b"felix", b"markus", b"isabel", b"jonathan", b"denis"));
        })
    }
}
