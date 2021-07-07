use bloom_filter_plus::*;

fn main() {
    // test 1
    let mut filter = BloomFilter::new();
    filter.insert("key");
    filter.debug();
    assert_eq!(true, filter.contains("key"));
    assert_eq!(false, filter.contains("key1"));
    filter.save_to_file("test.bitmap").unwrap();

    let filter_2 = BloomFilter::new().load_file("test.bitmap").unwrap();
    println!("");
    filter_2.debug();
    assert_eq!(true, filter_2.contains("key"));
    assert_eq!(false, filter_2.contains("key1"));
    // test2
    let mut filter3 = BloomFilter::new().set_size(10);
    filter3.insert("key");
    filter3.debug();
    assert_eq!(true, filter3.contains("key"));
    assert_eq!(false, filter3.contains("key1"));
}
