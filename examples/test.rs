use bloom_filter_plus::*;

fn main() {
    // test 1
    // let filter = BloomFilter::new();
    // filter.insert("key").unwrap();
    // filter.debug();
    // assert_eq!(true, filter.contains("key").unwrap());
    // assert_eq!(false, filter.contains("key1").unwrap());
    // filter.save_to_file("test.bitmap").unwrap();

    let filter_2 = BloomFilter::new();
    filter_2.load_file("test.bitmap").unwrap();
    println!("");
    filter_2.debug();
    assert_eq!(true, filter_2.contains("key").unwrap());
    assert_eq!(false, filter_2.contains("key1").unwrap());
    // // test2
    // let mut filter3 = BloomFilter::new().set_size(10);
    // filter3.insert("key");
    // filter3.debug();
    // assert_eq!(true, filter3.contains("key"));
    // assert_eq!(false, filter3.contains("key1"));
}
