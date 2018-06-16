#[cfg(test)]
mod tests {
    use test::Bencher;
    use bloom_filter::*;

    #[bench]
    fn bench_add(bencher: &mut Bencher) -> () {
        let mut bf = BloomFilter::new();
        bencher.iter(move || {
            let adders = vec!("felix", "markus", "isabel", "jonathan", "denis");
            bf.add(adders);
        });
    }

    #[bench]
    fn bench_hash(bencher: &mut Bencher) -> () {
        bencher.iter(|| {
            hash("felix");
            hash("markus");
            hash("isabel");
            hash("jonathan");
            hash("denis");
        })
    }

    #[bench]
    fn bench_hash_vec(bencher: &mut Bencher) -> () {
        bencher.iter(|| {
            hash_vec("felix");
            hash_vec("markus");
            hash_vec("isabel");
            hash_vec("jonathan");
            hash_vec("denis");
        })
    }

    #[bench]
    fn bench_hash_many(bencher: &mut Bencher) -> () {
        bencher.iter(|| {
            hash_many(vec!("felix", "markus", "isabel", "jonathan", "denis"));
        })
    }
}
