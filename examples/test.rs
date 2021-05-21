use bloom_filter_plush::*;

fn main() {
    let mut filter = BloomFilter::new(BloomConfig {
        size: Some(DEFAULT_SIZE),
        hash_loop: Some(20),
    });
    filter.insert("key");
    println!("{}", filter.contains("key"));
    println!("{}", filter.contains("key1"));
}
