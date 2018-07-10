#![feature(uniform_paths, integer_atomics)]

extern crate byteorder;
extern crate libc;

#[macro_use]
extern crate log;
extern crate env_logger;

pub mod alloc;
pub mod cache;
pub mod metric;
pub mod ops;
pub mod segment;
pub mod storage;
