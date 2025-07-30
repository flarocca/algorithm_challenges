pub mod algorithms;

fn main() {
    let mut bloom = algorithms::bloom_filter::BloomFilter::new(1000, 0.01);

    bloom.add("example");
    bloom.add("test");
    bloom.add("rust");

    println!("Contains 'example': {}", bloom.contains("example"));
    println!("Contains 'test': {}", bloom.contains("not-test"));
}
