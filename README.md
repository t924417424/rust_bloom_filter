# Rust语言bloom_filter包（布隆过滤器）

## [在https://crates.io中查看](https://crates.io/crates/bloom_filter_plus)

## 测试用例
```rust
use bloom_filter_plus::*;

fn main() {
    // test 1
    let filter = BloomFilter::new();
    filter.insert("key").unwrap();
    filter.debug();
    assert_eq!(true, filter.contains("key").unwrap());
    assert_eq!(false, filter.contains("key1").unwrap());
    filter.save_to_file("test.bitmap").unwrap();

    let filter_2 = BloomFilter::new();
    filter_2.load_file("test.bitmap").unwrap();
    println!("");
    filter_2.debug();
    assert_eq!(true, filter_2.contains("key").unwrap());
    assert_eq!(false, filter_2.contains("key1").unwrap());

    // // test2
    let filter3 = BloomFilter::new();
    filter3.set_size(1024);
    filter3.insert("key").unwrap();
    filter3.debug();
    assert_eq!(true, filter3.contains("key").unwrap());
    assert_eq!(false, filter3.contains("key1").unwrap());
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