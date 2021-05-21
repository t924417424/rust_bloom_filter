use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::sync::RwLock;

/// bitmap size = 1024 * 8 bit
pub const DEFAULT_SIZE: usize = 1 << 10;
pub const DEFAULT_HASH_LOOP: usize = 10;
pub struct BloomConfig {
    pub size: Option<usize>,
    pub hash_loop: Option<usize>,
}
/// BloomFilter
pub struct BloomFilter {
    size: usize,
    hash_loop: usize,
    bitmap: RwLock<Box<Vec<u8>>>,
}

impl BloomFilter {
    /// create bloomfilter
    /// # example
    /// ```
    /// use bloom_filter::*;
    ///
    /// fn main() {
    ///     let mut filter = BloomFilter::new(BloomConfig {
    ///         size: Some(DEFAULT_SIZE),
    ///         hash_loop: Some(20),
    ///     });
    ///     filter.insert("key");
    ///     println!("{}", filter.contains("key"));
    ///     println!("{}", filter.contains("key1"));
    /// }
    ///
    /// ```
    pub fn new(c: BloomConfig) -> Self {
        let size = match c.size {
            Some(size) => {
                if size <= 0 {
                    DEFAULT_SIZE
                } else {
                    size
                }
            }
            None => DEFAULT_SIZE,
        };
        let hash_loop = match c.hash_loop {
            Some(hash_loop) => {
                if size <= 0 {
                    DEFAULT_HASH_LOOP
                } else {
                    hash_loop
                }
            }
            None => DEFAULT_HASH_LOOP,
        };
        let bitmap: Vec<u8> = vec![0; size];
        Self {
            size: size,
            hash_loop: hash_loop,
            bitmap: RwLock::new(Box::new(bitmap)),
        }
    }

    /// add key to bloomfilter
    /// # example
    /// ```
    /// use bloom_filter::*;
    ///
    /// fn main() {
    ///     let mut filter = BloomFilter::new(BloomConfig {
    ///         size: Some(DEFAULT_SIZE),
    ///         hash_loop: Some(20),
    ///     });
    ///     filter.insert("key");
    ///     println!("{}", filter.contains("key"));
    ///     println!("{}", filter.contains("key1"));
    /// }
    ///
    /// ```
    pub fn insert(&mut self, key: &str) {
        let indexs = self.hash(key);
        self.insert_bitmap(indexs);
    }


    /// Check whether the bloomfilter has key
    /// # example
    /// ```
    /// use bloom_filter::*;
    ///
    /// fn main() {
    ///     let mut filter = BloomFilter::new(BloomConfig {
    ///         size: Some(DEFAULT_SIZE),
    ///         hash_loop: Some(20),
    ///     });
    ///     filter.insert("key");
    ///     println!("{}", filter.contains("key"));
    ///     println!("{}", filter.contains("key1"));
    /// }
    ///
    /// ```
    pub fn contains(&self, key: &str) -> bool {
        let indexs = self.hash(key);
        self.contains_bitmap(indexs)
    }
    /// Binary print bitmap
    /// # example
    /// ```
    /// use bloom_filter::*;
    ///
    /// fn main() {
    ///     let mut filter = BloomFilter::new(BloomConfig {
    ///         size: Some(DEFAULT_SIZE),
    ///         hash_loop: Some(20),
    ///     });
    ///     filter.debug();
    ///     filter.insert("key");
    ///     filter.debug();
    /// }
    ///
    /// ```
    pub fn debug(&self) {
        match self.bitmap.read() {
            Ok(bitmap) => {
                for (index, _) in bitmap.iter().enumerate() {
                    print!("{:0>8}", format!("{:02b}", bitmap[index]));
                }
            }
            Err(_) => {}
        };
    }

    fn insert_bitmap(&mut self, indexs: Vec<usize>) {
        match self.bitmap.get_mut() {
            Ok(bitmap) => {
                for index in indexs {
                    //let m = 1 << (indexs[*index] % 8);
                    bitmap[index] |= 1 << (index % 8);
                }
            }
            Err(_) => {}
        };
    }

    fn contains_bitmap(&self, indexs: Vec<usize>) -> bool {
        match self.bitmap.read() {
            Ok(bitmap) => {
                for index in indexs {
                    //let m = 1 << (indexs[*index] % 8);
                    //bitmap[index] |= 1 << (index % 8);
                    // if b.data[indexs[i]]&(1<<(indexs[i]%8)) != byte(match) {
                    //     return false
                    // }
                    if bitmap[index] & (1 << (index % 8)) != 1 << (index % 8) {
                        return false;
                    }
                }
            }
            Err(_) => {}
        };
        true
    }

    fn hash(&self, key: &str) -> Vec<usize> {
        let mut result: Vec<usize> = vec![0];
        let mut hasher1 = DefaultHasher::new();
        for i in 0..self.hash_loop {
            hasher1.write(key.as_bytes());
            hasher1.write_usize(i);
            result.push(hasher1.finish() as usize % self.size);
        }
        result
    }
}
