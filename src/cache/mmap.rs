use libc::mmap;
use std::fs;
use crate::cache::types::Cache;

pub struct CacheMap<K, V> where K: Hash, V {
    inner: c_int,
    file: fs::File,
}

impl <K, V> Cache<K, V> for CacheMap<K, V> {
    
}
