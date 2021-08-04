use std::{
    cell::{Cell, RefCell},
    collections::hash_map::DefaultHasher,
    error::Error,
    fmt,
    fs::{File, OpenOptions},
    hash::Hasher,
    io::{Read, Write},
    path::Path,
    sync::RwLock,
};

#[derive(Debug)]
pub struct FilterError(String);

impl fmt::Display for FilterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "filter error: {}", self.0)
    }
}

impl std::error::Error for FilterError {}

impl FilterError {
    fn new_err(msg: &str) -> Result<(), FilterError> {
        Err(FilterError(msg.into()))
    }
}

/// bitmap size = 1024 * 8 bit
pub const DEFAULT_SIZE: usize = 1 << 10;
pub const DEFAULT_HASH_LOOP: usize = 10;
// pub struct BloomConfig {
//     pub size: Option<usize>,
//     pub hash_loop: Option<usize>,
// }
/// BloomFilter
pub struct BloomFilter {
    size: Cell<usize>,
    hash_loop: Cell<usize>,
    is_null: RwLock<Cell<bool>>,
    bitmap: RwLock<RefCell<Box<Vec<u8>>>>,
}

impl BloomFilter {
    /// create bloomfilter
    /// # example
    /// ```
    /// //test 1
    /// let filter = BloomFilter::new();
    /// filter.insert("key").unwrap();
    /// assert_eq!(true, filter.contains("key").unwrap());
    /// assert_eq!(false, filter.contains("key1").unwrap());

    /// // test2
    /// let filter2 = BloomFilter::new();
    /// filter2.set_size(10).unwrap();
    /// filter2.insert("key").unwrap();
    /// assert_eq!(true, filter2.contains("key").unwrap());
    /// assert_eq!(false, filter2.contains("key1").unwrap());
    ///
    /// ```
    pub fn new() -> Self {
        let bitmap: Vec<u8> = vec![0; DEFAULT_SIZE];
        Self {
            size: Cell::new(DEFAULT_SIZE),
            hash_loop: Cell::new(DEFAULT_HASH_LOOP),
            is_null: RwLock::new(Cell::new(true)),
            bitmap: RwLock::new(RefCell::new(Box::new(bitmap))),
        }
    }

    /// Set the bitmap size, the data in the bitmap needs to be empty
    /// # example
    /// // test2
    /// ```
    /// let filter2 = BloomFilter::new();
    /// filter2.set_size(10).unwrap();
    /// filter2.insert("key").unwrap();
    /// assert_eq!(true, filter2.contains("key").unwrap());
    /// assert_eq!(false, filter2.contains("key1").unwrap());
    /// ```
    pub fn set_size(&self, size: usize) -> &Self {
        match self.is_null.read() {
            Ok(is_null) => {
                if is_null.get() {
                    self.size.set(size);
                    match self.bitmap.write() {
                        Ok(mut bitmap) => {
                            let bitmap = bitmap.get_mut();
                            *bitmap = Box::new(vec![0; size]);
                        }
                        _ => {}
                    }
                } else {
                    println!(
                        "{}",
                        "The modification is invalid because the bitmap already has data"
                    );
                }
            }
            _ => {
                println!("{}", "get val err");
            }
        }
        self
    }

    /// Set the bit occupied by each data
    /// # example
    /// // test2
    /// ```
    /// let filter2 = BloomFilter::new();
    /// filter2.set_size(10).unwrap();
    /// filter2.insert("key").unwrap();
    /// assert_eq!(true, filter2.contains("key").unwrap());
    /// assert_eq!(false, filter2.contains("key1").unwrap());
    /// ```
    pub fn set_hash_loop(&self, hash_loop: usize) -> &Self {
        match self.is_null.read() {
            Ok(is_null) => {
                if is_null.get() {
                    self.hash_loop.set(hash_loop);
                } else {
                    println!(
                        "{}",
                        "The modification is invalid because the bitmap already has data"
                    );
                }
            }
            _ => {
                println!("{}", "get val err");
            }
        }
        self
    }

    /// create filter form file
    /// # example
    /// ```
    /// let filter = BloomFilter::new();
    /// filter.load_file("myfilter").unwrap();
    /// filter.insert("key").unwrap();
    /// filter.debug();
    /// ```
    pub fn load_file<P: AsRef<Path>>(&self, filename: P) -> Result<&Self, FilterError> {
        match self.is_null.read() {
            Ok(is_null) => {
                if !is_null.get() {
                    FilterError::new_err("get self.is_null err")?
                }
            }
            _ => FilterError::new_err("get config err")?,
        }
        let mut load_file: Option<File> =
            match OpenOptions::new().read(true).create(false).open(filename) {
                Ok(file) => Some(file),
                _ => None,
            };

        let mut data: Vec<u8> = Vec::new();
        // match load_file {
        //     Some(mut file) => match file.read_to_end(&mut data) {
        //         Ok(_) => {}
        //         Err(e) => FilterError::NewErr(e.to_string().as_str())?,
        //     },
        //     None => {}
        // }
        if let Some(file) = load_file.as_mut() {
            // let mut t = file;
            match file.read_to_end(&mut data) {
                Ok(_) => {}
                Err(e) => FilterError::new_err(e.to_string().as_str())?,
            }
        } else {
            FilterError::new_err("open file error")?
        }
        if data.len() >= 8 {
            let loops = data[data.len() - 8..].as_ptr();
            let loops = loops as *const usize;
            let loops = unsafe { *loops };
            self.size.set(data.len() - 8);
            self.hash_loop.set(loops);
            match self.is_null.write() {
                Ok(is_null) => {
                    is_null.set(false);
                }
                _ => FilterError::new_err("init filter form file error")?,
            }
            // self.bitmap = RwLock::new(RefCell::new(Box::new(data[..data.len() - 8].to_vec())));
            match self.bitmap.write() {
                Ok(mut bitmap) => {
                    let bitmap = bitmap.get_mut();
                    *bitmap = Box::new(data[..data.len() - 8].to_vec());
                }
                _ => FilterError::new_err("set bitmap err")?,
            }
        } else {
            FilterError::new_err("file error".into())?
        }
        Ok(self)
    }

    /// save filter to file
    /// # example
    /// ```
    /// let filter = BloomFilter::new();
    /// filter.save_to_file("key").unwrap();
    /// filter.debug();
    /// ```
    pub fn save_to_file<P: AsRef<Path>>(&self, filename: P) -> Result<(), Box<dyn Error>> {
        let mut load_file = OpenOptions::new().write(true).create(true).open(filename)?;
        match self.bitmap.read() {
            Ok(bitmap) => {
                let mut loops = self.hash_loop.get().to_ne_bytes().to_vec();
                let mut bitmap_tmp = bitmap.borrow_mut();
                bitmap_tmp.append(&mut loops);
                load_file.write_all(bitmap_tmp.as_slice())?;
            }
            Err(_) => FilterError::new_err("get bitmap err")?,
        }
        Ok(())
    }

    /// add key to bloomfilter
    /// # example
    /// // test2
    /// ```
    /// let filter2 = BloomFilter::new();
    /// filter2.set_size(10).unwrap();
    /// filter2.insert("key").unwrap();
    /// assert_eq!(true, filter2.contains("key").unwrap());
    /// assert_eq!(false, filter2.contains("key1").unwrap());
    /// ```
    pub fn insert(&self, key: &str) -> Result<(), FilterError> {
        match self.is_null.write() {
            Ok(is_null) => {
                is_null.set(false);
            }
            _ => FilterError::new_err("get self.is_null.try_write() err")?,
        }
        let indexs = self.hash(key);
        self.insert_bitmap(indexs)?;
        Ok(())
    }

    /// Check whether the bloomfilter has key
    /// # example
    /// // test2
    /// ```
    /// let filter2 = BloomFilter::new();
    /// filter2.set_size(10).unwrap();
    /// filter2.insert("key").unwrap();
    /// assert_eq!(true, filter2.contains("key").unwrap());
    /// assert_eq!(false, filter2.contains("key1").unwrap());
    /// ```
    pub fn contains(&self, key: &str) -> Result<bool, FilterError> {
        let indexs = self.hash(key);
        Ok(self.contains_bitmap(indexs))
    }
    /// Binary print bitmap
    /// # example
    /// ```
    /// let filter2 = BloomFilter::new();
    /// filter2.set_size(10).unwrap();
    /// filter2.insert("key").unwrap();
    /// assert_eq!(true, filter2.contains("key").unwrap());
    /// assert_eq!(false, filter2.contains("key1").unwrap());
    /// filter2.debug();
    /// }
    ///
    /// ```
    pub fn debug(&self) {
        match self.bitmap.read() {
            Ok(bitmap) => {
                let bitmap_tmp = bitmap.borrow();
                for (index, _) in bitmap_tmp.iter().enumerate() {
                    print!("{:0>8}", format!("{:02b}", bitmap_tmp[index]));
                }
            }
            Err(_) => {}
        };
    }

    /// Reset Bitmap
    pub fn clear(&self) -> Result<(), FilterError> {
        // self.bitmap = RwLock::new(RefCell::new(Box::new(vec![0; self.size.get()])));
        match self.bitmap.write() {
            Ok(bitmap) => match bitmap.try_borrow_mut() {
                Ok(mut bitmap) => {
                    *bitmap = Box::new(vec![0; self.size.get()]);
                    match self.is_null.write() {
                        Ok(is_null) => {
                            is_null.set(true);
                        }
                        _ => FilterError::new_err("get self.is_null.try_write err")?,
                    }
                }
                Err(e) => FilterError::new_err(e.to_string().as_str())?,
            },
            _ => FilterError::new_err("set bitmap err")?,
        }
        Ok(())
    }

    fn insert_bitmap(&self, indexs: Vec<usize>) -> Result<(), FilterError> {
        match self.bitmap.write() {
            Ok(bitmap) => {
                // let mut bitmap_tmp = bitmap.try_borrow_mut();
                match bitmap.try_borrow_mut() {
                    Ok(mut bitmap) => {
                        for index in indexs {
                            //let m = 1 << (indexs[*index] % 8);
                            bitmap[index] |= 1 << (index % 8);
                        }
                    }
                    _ => {}
                }
                // for index in indexs {
                //     //let m = 1 << (indexs[*index] % 8);
                //     bitmap_tmp[index] |= 1 << (index % 8);
                // }
                return Ok(());
            }
            Err(e) => return FilterError::new_err(e.to_string().as_str()),
        };
        // Ok(())
    }

    fn contains_bitmap(&self, indexs: Vec<usize>) -> bool {
        match self.bitmap.read() {
            Ok(bitmap) => {
                let bitmap_tmp = bitmap.borrow();
                for index in indexs {
                    //let m = 1 << (indexs[*index] % 8);
                    //bitmap[index] |= 1 << (index % 8);
                    // if b.data[indexs[i]]&(1<<(indexs[i]%8)) != byte(match) {
                    //     return false
                    // }
                    if bitmap_tmp[index] & (1 << (index % 8)) != 1 << (index % 8) {
                        return false;
                    }
                }
            }
            Err(_) => return false,
        };
        true
    }

    fn hash(&self, key: &str) -> Vec<usize> {
        let mut result: Vec<usize> = vec![0];
        let mut hasher1 = DefaultHasher::new();
        let size = self.size.get();
        for i in 0..self.hash_loop.get() {
            hasher1.write(key.as_bytes());
            hasher1.write_usize(i);
            result.push(hasher1.finish() as usize % size);
        }
        result
    }
}
