#[cfg(test)]
mod tests {
    use test::Bencher;
    use bloom_filter::*;

    #[bench]
    fn bench_add(bencher: &mut Bencher) -> () {
        let mut bf = BloomFilter::new();
        bencher.iter(move || {
            let adders: Vec<&[u8]> = vec!(b"felix", b"markus", b"isabel", b"jonathan", b"denis");
            bf.add(adders);
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
            hash_vec(b"felix");
            hash_vec(b"markus");
            hash_vec(b"isabel");
            hash_vec(b"jonathan");
            hash_vec(b"denis");
        })
    }

    #[bench]
    fn bench_hash_many(bencher: &mut Bencher) -> () {
        bencher.iter(|| {
            hash_many(vec!(b"felix", b"markus", b"isabel", b"jonathan", b"denis"));
        })
    }
}
