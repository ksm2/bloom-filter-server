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
            hash("felix")
        })
    }

    #[bench]
    fn bench_hash_vec(bencher: &mut Bencher) -> () {
        bencher.iter(|| {
            hash_vec("felix")
        })
    }
}
