pub struct Graveyard {
    data: SegmentPool,
}

impl Append for Graveyard {
    fn append(&mut self, buffer: &[u8]) -> Result<usize, io::Err> {}
}
