# Rust语言bloom_filter包（布隆过滤器）

## [在https://crates.io中查看](https://crates.io/crates/bloom_filter_plus)

## 测试用例
```rust
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

```
## Go版本基准测试 
### [查看Go版本实现](https://github.com/t924417424/BloomFilter)
### Insert:
```
goos: darwin
goarch: amd64
pkg: github.com/t924417424/BloomFilter
cpu: Intel(R) Core(TM) i3-8100B CPU @ 3.60GHz
Benchmark_Insert-4   	  976336	      1075 ns/op	     160 B/op	       1 allocs/op
PASS
ok  	github.com/t924417424/BloomFilter	1.505s
```
### Contains:
```
goos: darwin
goarch: amd64
pkg: github.com/t924417424/BloomFilter
cpu: Intel(R) Core(TM) i3-8100B CPU @ 3.60GHz
Benchmark_Contains-4   	 1000000	      1054 ns/op	     160 B/op	       1 allocs/op
PASS
ok  	github.com/t924417424/BloomFilter	1.169s
```