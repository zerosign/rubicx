use crate::alloc::error::AllocErr;
use std::{io, num};

#[derive(Debug)]
pub enum ChecksumErr {
    SizeMismatchErr(u64, u64),
    ParseErr(num::ParseIntError),
}

#[derive(Debug)]
pub enum SegmentErr {
    IoError(io::Error),
    ChecksumErr(ChecksumErr),
    AllocErr(AllocErr),
}
