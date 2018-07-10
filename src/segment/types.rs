use std::io;

pub(crate) trait Append {
    fn append(&mut self, buffer: &[u8]) -> Result<usize, io::Error>;
}
