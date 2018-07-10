use byteorder::{BigEndian, WriteBytesExt};
use crate::segment::{
    error::{ChecksumErr, SegmentErr},
    types::Append,
};
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicU64;
use std::{
    fs,
    io::{self, Read, Seek, Write},
};

///
/// Segment could shared underlying Space.
///
#[derive(Debug)]
pub struct SegmentFile {
    id: u64,
    data_file: fs::File,
    checksum_file: fs::File,
    size: AtomicU64,
}

impl Append for SegmentFile {
    fn append(&mut self, buffer: &[u8]) -> Result<usize, io::Error> {
        let seek = self.data_file.seek(io::SeekFrom::End(0))?;
        trace!("data file seek into end {}", seek);

        let size = self.data_file.write(&buffer)? as usize;
        trace!("data written into buffer {}", size);

        let seek = self.checksum_file.seek(io::SeekFrom::Start(0))?;
        trace!("checksum file seek into start {}", seek);

        self.checksum_file.write_u64::<BigEndian>(size as u64)?;

        *self.size.get_mut() = size as u64;

        Ok(size)
    }
}

impl io::Read for SegmentFile {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        self.data_file.read(buf)
    }
}

impl io::Seek for SegmentFile {
    #[inline]
    fn seek(&mut self, pos: io::SeekFrom) -> Result<u64, io::Error> {
        self.data_file.seek(pos)
    }
}

impl SegmentFile {
    pub(crate) fn path_of<P>(base: P, id: u64) -> (PathBuf, PathBuf)
    where
        P: AsRef<Path>,
    {
        let mut path = PathBuf::new();
        path.push(base);
        path.push(id.to_string());

        (
            {
                let mut target = path.clone();
                target.push(".data");
                target
            },
            {
                let mut target = path.clone();
                target.push(".checksum");
                target
            },
        )
    }

    pub fn new<P>(base: P, id: u64) -> Result<Self, SegmentErr>
    where
        P: AsRef<Path>,
    {
        let (data_path, checksum_path) = SegmentFile::path_of(base, id);

        let mut data_file = fs::OpenOptions::new()
            .write(true)
            .read(true)
            .create_new(true)
            .open(&data_path)
            .map_err(|e| SegmentErr::IoError(e))?;

        // always start from the eof (append)
        let size = data_file
            .seek(io::SeekFrom::End(0))
            .map_err(|e| SegmentErr::IoError(e))?;

        let mut checksum_file = fs::OpenOptions::new()
            .write(true)
            .read(true)
            .create_new(true)
            .open(&checksum_path)
            .map_err(|e| SegmentErr::IoError(e))?;

        let _ = checksum_file
            .write_u64::<BigEndian>(size)
            .map_err(|e| SegmentErr::IoError(e))?;

        Ok(SegmentFile {
            id: id,
            data_file: data_file,
            checksum_file: checksum_file,
            size: AtomicU64::new(size),
        })
    }

    pub fn from_path<P>(base: P, id: u64) -> Result<Self, SegmentErr>
    where
        P: AsRef<Path>,
    {
        let (data_path, checksum_path) = SegmentFile::path_of(base, id);

        let mut data_file = fs::OpenOptions::new()
            .write(true)
            .read(true)
            .open(&data_path)
            .map_err(|e| SegmentErr::IoError(e))?;

        let size = data_file
            .seek(io::SeekFrom::End(0))
            .map_err(|e| SegmentErr::IoError(e))?;

        let mut checksum_file = fs::OpenOptions::new()
            .write(true)
            .read(true)
            .open(&checksum_path)
            .map_err(|e| SegmentErr::IoError(e))?;

        let mut buffer = String::new();
        let _ = checksum_file
            .read_to_string(&mut buffer)
            .map_err(|e| SegmentErr::IoError(e))?;

        match buffer.parse::<u64>() {
            Ok(csize) => {
                if csize == size {
                    Ok(SegmentFile {
                        id: id,
                        data_file: data_file,
                        checksum_file: checksum_file,
                        size: AtomicU64::new(csize),
                    })
                } else {
                    Err(SegmentErr::ChecksumErr(ChecksumErr::SizeMismatchErr(
                        csize, size,
                    )))
                }
            }
            Err(e) => Err(SegmentErr::ChecksumErr(ChecksumErr::ParseErr(e))),
        }
    }
}
