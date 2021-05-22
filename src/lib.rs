use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::sync::RwLock;

/// bitmap size = 1024 * 8 bit
pub const DEFAULT_SIZE: usize = 1 << 10;
pub const DEFAULT_HASH_LOOP: usize = 10;
// pub struct BloomConfig {
//     pub size: Option<usize>,
//     pub hash_loop: Option<usize>,
// }
/// BloomFilter
pub struct BloomFilter {
    size: usize,
    hash_loop: usize,
    is_null: bool,
    bitmap: RwLock<Box<Vec<u8>>>,
}

impl BloomFilter {
    /// create bloomfilter
    /// # example
    /// ```
    /// test 1
    /// let mut filter = BloomFilter::new();
    /// filter.insert("key");
    /// assert_eq!(true, filter.contains("key"));
    /// assert_eq!(false, filter.contains("key1"));

    /// // test2
    /// let mut filter2 = BloomFilter::new().set_size(10);
    /// filter2.insert("key");
    /// assert_eq!(true, filter2.contains("key"));
    /// assert_eq!(false, filter2.contains("key1"));
    ///
    /// ```
    pub fn new() -> Self {
        let bitmap: Vec<u8> = vec![0; DEFAULT_SIZE];
        Self {
            size: DEFAULT_SIZE,
            hash_loop: DEFAULT_HASH_LOOP,
            is_null: true,
            bitmap: RwLock::new(Box::new(bitmap)),
        }
    }

    /// Set the bitmap size, the data in the bitmap needs to be empty
    /// # example
    /// // test2
    /// ```
    /// let mut filter2 = BloomFilter::new().set_size(10);
    /// filter2.insert("key");
    /// assert_eq!(true, filter2.contains("key"));
    /// assert_eq!(false, filter2.contains("key1"));
    /// ```
    pub fn set_size(mut self, size: usize) -> Self {
        if self.is_null {
            self.size = size;
            self.bitmap = RwLock::new(Box::new(vec![0; size]));
        } else {
            println!(
                "{}",
                "The modification is invalid because the bitmap already has data"
            );
        }
        self
    }

    /// Set the bit occupied by each data
    /// # example
    /// // test2
    /// ```
    /// let mut filter2 = BloomFilter::new().set_size(10);
    /// filter2.insert("key");
    /// assert_eq!(true, filter2.contains("key"));
    /// assert_eq!(false, filter2.contains("key1"));
    /// ```
    pub fn set_hash_loop(mut self, hash_loop: usize) -> Self {
        if self.is_null {
            self.hash_loop = hash_loop;
        } else {
            println!(
                "{}",
                "The modification is invalid because the bitmap already has data"
            );
        }
        self
    }

    /// add key to bloomfilter
    /// # example
    /// // test2
    /// ```
    /// let mut filter2 = BloomFilter::new().set_size(10);
    /// filter2.insert("key");
    /// assert_eq!(true, filter2.contains("key"));
    /// assert_eq!(false, filter2.contains("key1"));
    /// ```
    pub fn insert(&mut self, key: &str) {
        self.is_null = false;
        let indexs = self.hash(key);
        self.insert_bitmap(indexs);
    }

    /// Check whether the bloomfilter has key
    /// # example
    /// // test2
    /// ```
    /// let mut filter2 = BloomFilter::new().set_size(10);
    /// filter2.insert("key");
    /// assert_eq!(true, filter2.contains("key"));
    /// assert_eq!(false, filter2.contains("key1"));
    /// ```
    pub fn contains(&self, key: &str) -> bool {
        let indexs = self.hash(key);
        self.contains_bitmap(indexs)
    }
    /// Binary print bitmap
    /// # example
    /// ```
    /// let mut filter2 = BloomFilter::new().set_size(10);
    /// filter2.insert("key");
    /// assert_eq!(true, filter2.contains("key"));
    /// assert_eq!(false, filter2.contains("key1"));
    /// filter2.debug();
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

    /// Reset Bitmap
    pub fn clear(&mut self) {
        self.bitmap = RwLock::new(Box::new(vec![0; self.size]));
        self.is_null = true;
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
