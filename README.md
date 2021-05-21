# Rust语言bloom_filter包（布隆过滤器）

## [在https://crates.io中查看](https://crates.io/crates/bloom_filter_plus)

## 测试用例
```rust
use bloom_filter_plus::*;

fn main() {
    let mut filter = BloomFilter::new(BloomConfig {
        size: Some(DEFAULT_SIZE),
        hash_loop: Some(20),
    });
    filter.insert("key");
    println!("{}", filter.contains("key"));
    println!("{}", filter.contains("key1"));
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