use bloom_filter_plus::*;

fn main() {
    // test 1
    let mut filter = BloomFilter::new();
    filter.insert("key");
    assert_eq!(true, filter.contains("key"));
    assert_eq!(false, filter.contains("key1"));

    // test2
    let mut filter2 = BloomFilter::new().set_size(10);
    filter2.insert("key");
    filter2.debug();
    assert_eq!(true, filter2.contains("key"));
    assert_eq!(false, filter2.contains("key1"));
}
